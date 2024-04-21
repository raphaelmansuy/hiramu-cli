use crate::model::Generate;
use futures_util::TryStreamExt;
use hiramu::ollama::model::GenerateRequestBuilder;
use hiramu::ollama::ollama_client::OllamaClient;

use hiramu::ollama::OptionsBuilder;
use tokio::io::AsyncWriteExt;

pub struct OllamaProvider {
    endpoint: String,
    model: String,
    maxtoken: Option<u32>,
    temperature: Option<f32>,
}

impl OllamaProvider {
    pub fn new(
        endpoint: String,
        model: String,
        maxtoken: Option<u32>,
        temperature: Option<f32>,
    ) -> OllamaProvider {
        OllamaProvider {
            endpoint,
            model,
            maxtoken,
            temperature,
        }
    }
}

impl Generate for OllamaProvider {
    async fn generate(&self, prompt: &str) -> () {
        let client = OllamaClient::new(self.endpoint.clone());

        let options_builder = OptionsBuilder::new();

        let options_builder = match self.maxtoken {
            Some(maxtoken) => options_builder.num_predict(maxtoken),
            None => options_builder,
        };

        let options_builder = match self.temperature {
            Some(temperature) => options_builder.temperature(temperature),
            None => options_builder,
        };

        let request = GenerateRequestBuilder::new(self.model.clone())
            .options_from_builder(options_builder)
            .prompt(prompt.to_owned())
            .build();

        let response_stream = client.generate(request).await.unwrap();

        response_stream
            .try_for_each(|chunk| async move {
                let mut stdout = tokio::io::stdout();
                stdout.write_all(chunk.response.as_bytes()).await.unwrap();
                stdout.flush().await.unwrap();

                Ok(())
            })
            .await
            .unwrap();

        println!("");
    }
}
