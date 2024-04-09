As a highly skilled Technical Lead with extensive experience in Rust, I need your assistance in breaking down a product specification and providing a detailed plan for implementation. Please follow these steps:

1. Carefully review the provided product specification and identify the key requirements and functionalities of the tool. Review the provided example and the Hiramu library. You must use the Hiramu library to make call to Ollama, Claude or Mistral. An example that shows how to call Hiramu is provided.

2. Based on the specification, identify and list the necessary Rust libraries that would be most suitable for implementing the required features. Provide a brief explanation of why each library is chosen and how it will contribute to the project.

3. Break down the problem into manageable modules, sub-modules, and functions. Create a clear hierarchy and structure for the codebase, ensuring that it is modular, maintainable, and follows Rust best practices.

4. For each identified function, write comprehensive documentation that includes:
   - A clear and concise description of the function's purpose and behavior
   - Input parameters and their types, along with any constraints or assumptions
   - Return values and their types, if applicable
   - Any side effects or exceptions that may occur during the function's execution
   - Examples of how to use the function, showcasing common use cases and edge cases

Please provide your analysis and design in a well-formatted, markdown-compatible response. Use appropriate headings, subheadings, and code blocks to enhance readability and clarity.

Format as markdown.

<example_hiramu>

# Table of Contents
- [src/main.rs](#srcmain.rs)

## File: src/main.rs

- Extension: .rs
- Size: 1499 bytes
- Created: 2024-04-07 11:40:55
- Modified: 2024-04-07 11:40:55

### Code
```.rs
use futures_util::TryStreamExt;
use hiramu::bedrock::model_info::{ModelInfo, ModelName};
use hiramu::bedrock::models::claude::claude_client::{ClaudeClient, ClaudeOptions};
use hiramu::bedrock::models::claude::claude_request_message::{
    ChatOptions, ContentBlockDelta, ConversationRequest, Message, StreamResultData,
};
use std::io::Write;

#[tokio::main]
async fn main() {
    let claude_options = ClaudeOptions::new()
        .profile_name("bedrock")
        .region("us-west-2");

    let client = ClaudeClient::new(claude_options).await;

    let mut conversation_request = ConversationRequest::default();
    conversation_request
        .messages
        .push(Message::new_user_message("Hello, Claude!".to_owned()));

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

```
</example_hiramu>

<product_specification>

hiramu-cli utility:

## Principal Functionalities:

1. Interact with Large Language Models (LLMs) via remote APIs and locally installed models (ollama)
2. Run prompts from the command-line
3. Generate embeddings
7. Start chat sessions with models
8. Use system prompts to provide instructions for processing input

## First Draft of Product Specification:

1. Overview
   - The hiramu-cli utility is a command-line tool developed in Rust that enables users to interact with Large Language Models (LLMs) through remote APIs and locally installed models.

2. Key Features
   - Command-line interface for running prompts
   - Support for remote APIs (Mistral, Claude anthropics) and locally installed models
   - Ability to generate embeddings
   - Chat sessions with locally installed models
   - System prompts for providing instructions to process input

3. Installation
   - The utility can be installed using Homebrew or cargo
   - Detailed installation instructions should be provided in the documentation


5. Usage
   - Running prompts: `hiramu-cli "prompt text"`
   - Listing available models: `hiramu-cli models`
   - Running prompts with specific models: `hiramu-cli -m model_name "prompt text"`
   - Starting chat sessions: `hiramu-cli chat -m model_name`
   - Using system prompts: `cat input_file | hiramu-cli -s "system prompt"`

6. Documentation
   - Comprehensive documentation should be available, covering installation, configuration, usage, and troubleshooting
   - Example use cases and best practices should be included

7. Performance
   - The utility should be optimized for fast response times and efficient resource usage
   - Guidelines for minimum system requirements (e.g., RAM) for running local models should be provided

8. Security
   - Best practices for securely storing API keys should be implemented
   - Guidelines for ensuring the security of locally installed models should be provided

9. Maintenance and Support
   - Regular updates and bug fixes should be released
   - A clear process for reporting issues and requesting features should be established
   - Community support channels (e.g., forums, chat) should be available

10. Roadmap
    - Future development plans should be outlined, including new features, integrations, and performance improvements
    - A process for gathering and prioritizing user feedback should be established

This first draft of the product specification provides an overview of the LLM CLI utility's key features, installation process, usage, documentation, performance considerations, security aspects, maintenance and support, and future roadmap. It can serve as a starting point for further refinement and discussion among the product team and stakeholders.

</product_specification>
