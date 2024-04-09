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
pub struct ClaudeGenerator {}

impl ClaudeGenerator {
    pub fn new() -> ClaudeGenerator {
        ClaudeGenerator {}
    }
}

impl Generate for ClaudeGenerator {
    async fn generate(&self, question: &str) {
        let claude_options = ClaudeOptions::new()
            .profile_name("bedrock")
            .region("us-west-2");

        let client = ClaudeClient::new(claude_options).await;

        let mut conversation_request = ConversationRequest::default();
        conversation_request
            .messages
            .push(Message::new_user_message(question.to_owned()));

        let chat_options = ChatOptions::default()
            .with_temperature(0.7)
            .with_max_tokens(100)
            .with_model_id(ModelInfo::from_model_name(
                ModelName::AnthropicClaudeHaiku1x,
            ));

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
