//use std::ffi::OsString;
//use std::path::PathBuf;

use std::io::Write;

use clap::{arg, Command};
use futures_util::TryStreamExt;
use hiramu::bedrock::{models::claude::{claude_client::ClaudeOptions, claude_request_message::{ContentBlockDelta, StreamResultData}, ChatOptions, ClaudeClient, ConversationRequest, Message}, ModelInfo, ModelName};

fn cli() -> Command {
    Command::new("hiramu-cli")
        .about("A fictional versioning CLI")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("prompt")
                .about("Ask a question to a LLM")
                .arg(arg!(<PROMPT> "The promp to ask"))
                .arg_required_else_help(true),
        )
}


pub async fn generate(question: &str)  {
       
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
                    print!("{}", delta.text);
                    std::io::stdout().flush().unwrap();
                }
                _ => {}
            }

            Ok(())
        })
        .await
        .unwrap();

    println!(); 
}


#[tokio::main]
async fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("prompt", sub_matches)) => {
            let prompt = sub_matches.get_one::<String>("PROMPT").expect("required");
            generate(prompt).await;
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable!()
    }

    // Continued program logic goes here...
}