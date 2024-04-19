use crate::model::Generate;
use futures_util::TryStreamExt;
use hiramu::ollama::model::GenerateRequestBuilder;
use hiramu::ollama::ollama_client::OllamaClient;

use tokio::io::AsyncWriteExt;

pub struct OllamaProvider {
    endpoint: String,
    model: String,
}

impl OllamaProvider {
    pub fn new(endpoint: String, model: String) -> OllamaProvider {
        OllamaProvider { endpoint, model }
    }
}

impl Generate for OllamaProvider {
    async fn generate(&self, prompt: &str) -> () {
        let client = OllamaClient::new(self.endpoint.clone());
        let request = GenerateRequestBuilder::new(self.model.clone())
            .prompt(prompt.to_string())
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
