use futures_util::TryStreamExt;
use hiramu::bedrock::{
    models::claude::{
        claude_client::ClaudeOptions,
        claude_request_message::{ContentBlockDelta, StreamResultData},
        ChatOptions, ClaudeClient, ConversationRequest, Message,
    },
    ModelInfo, ModelName,
};
use tokio::io::AsyncWriteExt;

use crate::model::Generate;
pub struct ClaudeGenerator {
    region: String,
    profile: String,
    max_token: Option<u32>,
    temperature: Option<f32>,
    model: Option<ModelName>,
}

impl ClaudeGenerator {
    pub fn new(
        region: Option<String>,
        profile: Option<String>,
        max_token: Option<u32>,
        temperature: Option<f32>,
        model: Option<ModelName>,
    ) -> ClaudeGenerator {
        ClaudeGenerator {
            region: region.unwrap_or_else(|| "us-west-2".to_string()),
            profile: profile.unwrap_or_else(|| "bedrock".to_string()),
            max_token,
            temperature,
            model,
        }
    }
}

impl Generate for ClaudeGenerator {
    async fn generate(&self, question: &str) {
        let claude_options = ClaudeOptions::new()
            .profile_name(&self.profile)
            .region(&self.region);

        let client = ClaudeClient::new(claude_options).await;

        let mut conversation_request = ConversationRequest::default();
        conversation_request
            .messages
            .push(Message::new_user_message(question.to_owned()));

        let model_name = self
            .model
            .as_ref()
            .unwrap_or(&ModelName::AnthropicClaudeHaiku1x);

        let mut chat_options =
            ChatOptions::default().with_model_id(ModelInfo::from_model_name(model_name.clone()));
        if let Some(max_token) = self.max_token {
            chat_options = chat_options.with_max_tokens(max_token);
        }

        if let Some(temperature) = self.temperature {
            chat_options = chat_options.with_temperature(temperature);
        }

        let response_stream = client
            .chat_with_stream(&conversation_request, &chat_options)
            .await
            .unwrap();

        response_stream
            .try_for_each(|chunk| async move {
                match chunk {
                    StreamResultData::ContentBlockDelta(ContentBlockDelta { delta, .. }) => {
                        let mut stdout = tokio::io::stdout();
                        stdout.write_all(delta.text.as_bytes()).await.unwrap();
                        stdout.flush().await.unwrap();
                    }
                    _ => {}
                }

                Ok(())
            })
            .await
            .unwrap();

        println!();
    }
}
