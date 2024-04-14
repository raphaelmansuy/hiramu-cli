use futures_util::StreamExt;
use hiramu::bedrock::model_info::{ModelInfo, ModelName};
use hiramu::bedrock::models::mistral::mistral_client::{MistralClient, MistralOptions};
use hiramu::bedrock::models::mistral::mistral_request_message::MistralRequestBuilder;

use tokio::io::AsyncWriteExt;

use crate::model::Generate;

pub struct MistralGenerator {
    region: String,
    profile: String,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
    model_name: Option<ModelName>,
}

impl MistralGenerator {
    pub fn new(
        region: Option<String>,
        profile: Option<String>,
        max_tokens: Option<u32>,
        temperature: Option<f32>,
        model_name: Option<ModelName>,
    ) -> MistralGenerator {
        MistralGenerator {
            region: region.unwrap_or_else(|| "us-west-2".to_string()),
            profile: profile.unwrap_or_else(|| "bedrock".to_string()),
            max_tokens,
            temperature,
            model_name,
        }
    }
}

impl Generate for MistralGenerator {
    async fn generate(&self, prompt: &str) {
        let mistral_options = MistralOptions::new()
            .profile_name(&self.profile)
            .region(&self.region);

        let client = MistralClient::new(mistral_options).await;

        let query_prompt = format!("{}{}{}", "[inst]", prompt, "[/inst]");
        let mut request_builder = MistralRequestBuilder::new(query_prompt.to_owned());
        if let Some(max_tokens) = self.max_tokens {
            request_builder = request_builder.max_tokens(max_tokens);
        }
        if let Some(temperature) = self.temperature {
            request_builder = request_builder.temperature(temperature);
        }
        let stops: Vec<String> = vec!["[/inst]","[/resp]","[/ans]","[/out]","[/sol]"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        request_builder = request_builder.stop(stops);

        let request = request_builder.build();

        let model_name = self
            .model_name
            .as_ref()
            .unwrap_or(&ModelName::MistralMixtral8X7BInstruct0x)
            .clone();

        let model_id = ModelInfo::from_model_name(model_name);
        let mut stream = client
            .generate_with_stream(model_id, &request)
            .await
            .unwrap();

        while let Some(result) = stream.next().await {
            match result {
                Ok(response) => {
                    let mut stdout = tokio::io::stdout();
                    stdout
                        .write_all(response.outputs[0].text.as_bytes())
                        .await
                        .unwrap();
                    stdout.flush().await.unwrap();
                }
                Err(err) => {
                    eprintln!("Error: {:?}", err);
                }
            }
        }
        println!("");
    }
}
