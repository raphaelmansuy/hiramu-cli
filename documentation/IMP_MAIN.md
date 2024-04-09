As a software developer expert in Rust takes your time to read this implementation plan and review the hiramu library.

Your task : implement the version of the file main.rs,  `fn main()`


- Define what to implement
- Define how to implement 
- Write the full main function


<implementation_plan>
# Hiramu-CLI Utility Implementation Plan

## 1. Key Requirements and Functionalities

Based on the provided product specification, the key requirements and functionalities of the hiramu-cli utility are:

1. Command-line interface for interacting with Large Language Models (LLMs)
2. Support for remote APIs (Mistral, Claude anthropics) and locally installed models (ollama)
3. Ability to run prompts from the command-line
4. Generation of embeddings
5. Starting chat sessions with models
6. Using system prompts to provide instructions for processing input

## 2. Rust Libraries

To implement the required features, the following Rust libraries can be used:

1. `clap` - A powerful command-line argument parser for building CLI applications. It will help in creating a user-friendly and intuitive command-line interface for the hiramu-cli utility.

2. `tokio` - An asynchronous runtime for Rust that allows writing scalable and concurrent applications. It will be useful for handling asynchronous operations, such as making API calls to remote models and managing chat sessions.

3. `reqwest` - A high-level HTTP client library for making HTTP requests. It will be used to interact with remote APIs, such as Mistral and Claude anthropics.

4. `serde` and `serde_json` - Libraries for serializing and deserializing Rust data structures to and from JSON. They will be used for parsing and generating JSON data when communicating with APIs and handling configuration files.

5. `futures` - A library for working with asynchronous values and streams. It will be used in conjunction with `tokio` for handling asynchronous operations and processing streams of data.

6. `hiramu` - The provided Hiramu library will be used to make calls to Ollama, Claude, or Mistral models.

## 3. Modular Structure

The codebase can be structured into the following modules and sub-modules:

```
hiramu-cli/
├── src/
│   ├── main.rs
│   ├── cli/
│   │   ├── mod.rs
│   │   ├── args.rs
│   │   └── commands/
│   │       ├── mod.rs
│   │       ├── prompt.rs
│   │       ├── models.rs
│   │       ├── chat.rs
│   │       └── embed.rs
│   ├── models/
│   │   ├── mod.rs
│   │   ├── remote/
│   │   │   ├── mod.rs
│   │   │   ├── mistral.rs
│   │   │   └── claude.rs
│   │   └── local/
│   │       ├── mod.rs
│   │       └── ollama.rs
│   ├── utils/
│   │   ├── mod.rs
│   │   ├── config.rs
│   │   └── api.rs
│   └── error.rs
└── Cargo.toml
```

- `main.rs`: The entry point of the application, responsible for initializing the CLI and dispatching commands.
- `cli/`: Contains the command-line interface logic, including argument parsing and command handling.
- `models/`: Implements the functionality for interacting with remote APIs and local models.
- `utils/`: Provides utility functions for configuration handling and API communication.
- `error.rs`: Defines custom error types and handles error propagation throughout the application.

## 4. Function Documentation

### `fn main()`

The entry point of the hiramu-cli utility.

- Description: Initializes the command-line interface, parses the provided arguments, and dispatches the appropriate command.
- Input: None
- Output: None
- Side Effects:
  - Parses command-line arguments
  - Initializes the necessary configurations and clients
  - Executes the specified command based on the provided arguments
- Example Usage:
  ```bash
  $ hiramu-cli -m mistral "What is the capital of France?"
  ```

### `fn run_prompt(model: &str, prompt: &str) -> Result<(), Error>`

Runs a prompt on the specified model.

- Description: Sends a prompt to the specified model and retrieves the generated response.
- Input:
  - `model`: The name of the model to use for generating the response.
  - `prompt`: The text prompt to send to the model.
- Output:
  - Returns a `Result` indicating the success or failure of the operation.
  - On success, returns `Ok(())`.
  - On failure, returns an `Err` variant containing an `Error`.
- Side Effects:
  - Sends an API request to the specified model with the provided prompt.
  - Prints the generated response to the console.
- Example Usage:
  ```rust
  let model = "mistral";
  let prompt = "What is the capital of France?";
  match run_prompt(model, prompt) {
      Ok(()) => println!("Prompt executed successfully"),
      Err(e) => eprintln!("Error: {}", e),
  }
  ```

### `fn start_chat(model: &str) -> Result<(), Error>`

Starts a chat session with the specified model.

- Description: Initiates a chat session with the specified model, allowing the user to have an interactive conversation.
- Input:
  - `model`: The name of the model to use for the chat session.
- Output:
  - Returns a `Result` indicating the success or failure of the operation.
  - On success, returns `Ok(())`.
  - On failure, returns an `Err` variant containing an `Error`.
- Side Effects:
  - Establishes a connection with the specified model for the chat session.
  - Reads user input from the console and sends it to the model.
  - Prints the model's responses to the console.
  - Continues the chat session until the user terminates it.
- Example Usage:
  ```rust
  let model = "ollama";
  match start_chat(model) {
      Ok(()) => println!("Chat session ended"),
      Err(e) => eprintln!("Error: {}", e),
  }
  ```

### `fn generate_embeddings(model: &str, input: &str) -> Result<Vec<f32>, Error>`

Generates embeddings for the given input using the specified model.

- Description: Sends the input text to the specified model and retrieves the generated embeddings.
- Input:
  - `model`: The name of the model to use for generating embeddings.
  - `input`: The input text for which embeddings should be generated.
- Output:
  - Returns a `Result` containing a vector of `f32` values representing the generated embeddings.
  - On success, returns `Ok(embeddings)`, where `embeddings` is a `Vec<f32>` containing the generated embeddings.
  - On failure, returns an `Err` variant containing an `Error`.
- Side Effects:
  - Sends an API request to the specified model with the provided input text.
- Example Usage:
  ```rust
  let model = "ollama";
  let input = "The quick brown fox jumps over the lazy dog";
  match generate_embeddings(model, input) {
      Ok(embeddings) => println!("Embeddings: {:?}", embeddings),
      Err(e) => eprintln!("Error: {}", e),
  }
  ```

This implementation plan provides a breakdown of the key requirements, suitable Rust libraries, modular structure, and documentation for the main functions of the hiramu-cli utility. It serves as a starting point for the development process and can be further refined and expanded as needed.
</implementation_plan>
<hiramu>
# Table of Contents
- [src/util.rs](#srcutil.rs)
- [src/error.rs](#srcerror.rs)
- [src/lib.rs](#srclib.rs)
- [src/main.rs](#srcmain.rs)
- [src/bedrock/error.rs](#srcbedrockerror.rs)
- [src/bedrock/bedrock_client.rs](#srcbedrockbedrock_client.rs)
- [src/bedrock/mod.rs](#srcbedrockmod.rs)
- [src/bedrock/model_info.rs](#srcbedrockmodel_info.rs)
- [src/examples/demo_mistral_stream.rs](#srcexamplesdemo_mistral_stream.rs)
- [src/examples/demo_bedrock_raw_stream.rs](#srcexamplesdemo_bedrock_raw_stream.rs)
- [src/examples/demo_claude_multimedia.rs](#srcexamplesdemo_claude_multimedia.rs)
- [src/examples/demo_claude_chat_stream.rs](#srcexamplesdemo_claude_chat_stream.rs)
- [src/examples/demo_ollama.rs](#srcexamplesdemo_ollama.rs)
- [src/examples/mod.rs](#srcexamplesmod.rs)
- [src/examples/demo_bedrock_raw_generate.rs](#srcexamplesdemo_bedrock_raw_generate.rs)
- [src/examples/demo_claude_chat.rs](#srcexamplesdemo_claude_chat.rs)
- [src/examples/demo_ollama_embedding.rs](#srcexamplesdemo_ollama_embedding.rs)
- [src/examples/demo_bedrock_raw_mistral.rs](#srcexamplesdemo_bedrock_raw_mistral.rs)
- [src/ollama/error.rs](#srcollamaerror.rs)
- [src/ollama/ollama_client.rs](#srcollamaollama_client.rs)
- [src/ollama/mod.rs](#srcollamamod.rs)
- [src/ollama/model.rs](#srcollamamodel.rs)
- [src/bedrock/models/mod.rs](#srcbedrockmodelsmod.rs)
- [src/bedrock/models/claude/claude_request_message.rs](#srcbedrockmodelsclaudeclaude_request_message.rs)
- [src/bedrock/models/claude/error.rs](#srcbedrockmodelsclaudeerror.rs)
- [src/bedrock/models/claude/mod.rs](#srcbedrockmodelsclaudemod.rs)
- [src/bedrock/models/claude/claude_client.rs](#srcbedrockmodelsclaudeclaude_client.rs)
- [src/bedrock/models/mistral/mistral_client.rs](#srcbedrockmodelsmistralmistral_client.rs)
- [src/bedrock/models/mistral/error.rs](#srcbedrockmodelsmistralerror.rs)
- [src/bedrock/models/mistral/mod.rs](#srcbedrockmodelsmistralmod.rs)
- [src/bedrock/models/mistral/mistral_request_message.rs](#srcbedrockmodelsmistralmistral_request_message.rs)
- [src/examples/simple_examples/generate_text_stream_with_mistral.rs](#srcexamplessimple_examplesgenerate_text_stream_with_mistral.rs)
- [src/examples/simple_examples/image_with_claude.rs](#srcexamplessimple_examplesimage_with_claude.rs)
- [src/examples/simple_examples/mod.rs](#srcexamplessimple_examplesmod.rs)
- [src/examples/simple_examples/generating_text_with_ollama.rs](#srcexamplessimple_examplesgenerating_text_with_ollama.rs)
- [src/examples/simple_examples/chat_with_claude.rs](#srcexamplessimple_exampleschat_with_claude.rs)
- [src/examples/simple_examples/generating_text_with_mistral.rs](#srcexamplessimple_examplesgenerating_text_with_mistral.rs)

## File: src/util.rs

- Extension: .rs
- Size: 2045 bytes
- Created: 2024-04-02 22:13:19
- Modified: 2024-04-02 22:13:19

### Code
```.rs

use std::io::Read;

use base64::{engine::general_purpose, Engine as _};
use reqwest::Response;

use std::error::Error;
use std::fs::File;
use url::Url;

/// Fetches an image from a given path and encodes it to a base64 string.
///
/// The path can be either a URL or a local file path. If the path is a valid URL,
/// the function will download the image and encode it. If the path is not a URL,
/// the function will assume it's a local file path, read the file, and encode it.
///
/// # Arguments
///
/// * `path` - A string slice that holds the path to the image.
///
/// # Returns
///
/// This function returns a `Result` that contains a base64 encoded string of the image
/// if the operation was successful, or an error if the operation failed.
///
/// # Errors
///
/// This function will return an error if the path is not a valid URL or local file path,
/// or if the image could not be downloaded or read.
///
pub async fn fetch_and_base64_encode_image(path: &str) -> Result<String, Box<dyn Error>> {
    // Check if the path is a valid URL
    if let Ok(url) = Url::parse(path) {
        // If it's a URL, download the image
        let client = reqwest::Client::new();
        let response = client.get(url.as_str()).send().await?;
        let base64_string = encode_response_to_base64(response).await?;
        Ok(base64_string)
    } else {
        // If it's not a URL, assume it's a local file path
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let base64_string = general_purpose::STANDARD.encode(buffer);
        Ok(base64_string)
    }
}

// Helper function to encode a response to a base64 string
async fn encode_response_to_base64(response: Response) -> Result<String, Box<dyn Error>> {
    if response.status().is_success() {
        let bytes = response.bytes().await?;
        let base64_string = general_purpose::STANDARD.encode(bytes);
        Ok(base64_string)
    } else {
        Err(From::from("Failed to download the image"))
    }
}
```

## File: src/error.rs

- Extension: .rs
- Size: 71 bytes
- Created: 2024-04-04 21:29:59
- Modified: 2024-04-04 21:29:59

### Code
```.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HiramuError {}

```

## File: src/lib.rs

- Extension: .rs
- Size: 210 bytes
- Created: 2024-04-06 08:30:26
- Modified: 2024-04-06 08:30:26

### Code
```.rs
//! # Hiramu
//!
#![doc = include_str!("../README.md")]

pub mod ollama;
pub mod bedrock;
pub mod error;
pub mod util;
pub mod examples;

pub use error::HiramuError;
pub use util::fetch_and_base64_encode_image;
```

## File: src/main.rs

- Extension: .rs
- Size: 247 bytes
- Created: 2024-04-02 22:13:19
- Modified: 2024-04-02 22:13:19

### Code
```.rs
use hiramu::examples::demo_ollama::chat_response_loop;

#[tokio::main]
async fn main() {
    // A simple example that demonstrates how to use the Ollama API to generate responses to chat messages.
    chat_response_loop(u32::max_value()).await;
}

```

## File: src/bedrock/error.rs

- Extension: .rs
- Size: 1319 bytes
- Created: 2024-04-04 21:29:59
- Modified: 2024-04-04 21:29:59

### Code
```.rs
use aws_sdk_bedrockruntime::operation::invoke_model::InvokeModelError;
use serde_json::Error as SerdeJsonError;
use thiserror::Error;

use aws_sdk_bedrockruntime::error::SdkError;
use aws_sdk_bedrockruntime::operation::invoke_model_with_response_stream::InvokeModelWithResponseStreamError;
use aws_sdk_bedrockruntime::types::error::ResponseStreamError;
use aws_smithy_types::event_stream::RawMessage;

#[derive(Error, Debug)]
pub enum BedrockError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] SerdeJsonError),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::str::Utf8Error),

    #[error("Bedrock runtime error: {0}")]
    AwsBedrockRuntimeError(#[from] aws_sdk_bedrockruntime::Error),

    #[error("Bedrock error: {0}")]
    BedrockError(#[from] aws_sdk_bedrock::Error),

    #[error("AWS SDK error: {0}")]
    AwsSdkError(#[from] SdkError<ResponseStreamError, RawMessage>),

    #[error("AWS SDK invoke model error: {0}")]
    AwsSdkErrorInvoke(#[from] SdkError<InvokeModelWithResponseStreamError>),

    #[error("AWS SDK invoke model error: {0}")]
    AwsSdkErrorInvokeModel(#[from] SdkError<InvokeModelError>),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

```

## File: src/bedrock/bedrock_client.rs

- Extension: .rs
- Size: 8384 bytes
- Created: 2024-04-04 21:29:59
- Modified: 2024-04-04 21:29:59

### Code
```.rs
use crate::bedrock::error::BedrockError;
use aws_config::Region;
use aws_sdk_bedrock::config::BehaviorVersion;
use aws_sdk_bedrockruntime::Client;
use futures::stream::Stream;
use serde_json::Value;
use std::borrow::Cow;
use tokio_stream::wrappers::UnboundedReceiverStream;

/// Configuration options for creating a `BedrockClient`.
///
/// This struct holds optional configuration values that can be used when creating a `BedrockClient`.
/// These include the AWS profile name, region, endpoint URL, and behavior version.
///
/// # Fields
///
/// * `profile_name` - The name of the AWS profile to use for authentication. If `None`, the default profile is used.
/// * `region` - The AWS region to use. If `None`, the region is determined from the environment or AWS configuration.
/// * `endpoint_url` - The endpoint URL to use for the Bedrock service. If `None`, the default endpoint for the region is used.
/// * `behavior_version` - The behavior version to use for the Bedrock service. If `None`, the latest version is used.
///
#[derive(Debug, Clone)]
pub struct BedrockClientOptions {
    profile_name: Option<String>,
    region: Option<String>,
    endpoint_url: Option<String>,
    behavior_version: Option<BehaviorVersion>,
}

impl BedrockClientOptions {
    pub fn new() -> Self {
        Self {
            profile_name: None,
            region: Some("us-west-2".to_string()),
            endpoint_url: None,
            behavior_version: Some(BehaviorVersion::v2023_11_09()),
        }
    }

    pub fn profile_name<S: Into<String>>(mut self, profile_name: S) -> Self {
        self.profile_name = Some(profile_name.into());
        self
    }

    pub fn region<S: Into<String>>(mut self, region: S) -> Self {
        self.region = Some(region.into());
        self
    }

    pub fn endpoint_url<S: Into<String>>(mut self, endpoint_url: S) -> Self {
        self.endpoint_url = Some(endpoint_url.into());
        self
    }

    pub fn behavior_version(mut self, behavior_version: BehaviorVersion) -> Self {
        self.behavior_version = Some(behavior_version);
        self
    }
}

pub struct BedrockClient {
    client: Client,
}

//
impl BedrockClient {
    /// Constructs a new `BedrockClient`.
    ///
    /// This function takes a `BedrockClientOptions` struct, which provides configuration options for the client,
    /// and returns a new `BedrockClient`.
    ///
    /// # Arguments
    ///
    /// * `options` - A `BedrockClientOptions` struct that provides configuration options for the client.
    ///
    /// # Returns
    ///
    /// This function returns a new `BedrockClient`.
    pub async fn new(options: BedrockClientOptions) -> Self {
        let client = Self::create_client(options).await;
        Self { client }
    }
    /// Creates a new `Client` using the provided options.
    ///
    /// This function is used internally by `new` to create a new `Client`.
    ///
    /// # Arguments
    ///
    /// * `options` - A `BedrockClientOptions` struct that provides configuration options for the client.
    ///
    /// # Returns
    ///
    /// This function returns a new `Client`.
    async fn create_client(options: BedrockClientOptions) -> Client {
        let mut config_loader = aws_config::ConfigLoader::default();

        if let Some(profile_name) = options.profile_name {
            config_loader = config_loader.profile_name(profile_name);
        }

        if let Some(region) = options.region {
            config_loader = config_loader.region(Region::new(region));
        }

        if let Some(endpoint_url) = options.endpoint_url {
            config_loader = config_loader.endpoint_url(endpoint_url);
        }

        if let Some(behavior_version) = options.behavior_version {
            config_loader = config_loader.behavior_version(behavior_version);
        }

        let config = config_loader.load().await;

        Client::new(&config)
    }

    /// Generates a raw stream of responses from the Bedrock service.
    ///
    /// This function takes a model ID and a payload, sends a request to the Bedrock service,
    /// and returns a stream of responses.
    ///
    /// # Arguments
    ///
    /// * `model_id` - A string that represents the model ID.
    /// * `payload` - A `Value` that represents the payload to send in the request.
    ///
    /// # Returns
    ///
    /// This function returns a `Result` that contains a stream of responses if the operation was successful,
    /// or an error if the operation failed.
    pub async fn generate_raw_stream(
        &self,
        model_id: String,
        payload: Value,
    ) -> Result<impl Stream<Item = Result<Value, BedrockError>>, BedrockError> {
        let payload_bytes = serde_json::to_vec(&payload);

        let payload_bytes = match payload_bytes {
            Ok(bytes) => bytes,
            Err(err) => return Err(BedrockError::from(err)),
        };

        let payload_blob = aws_smithy_types::Blob::new(payload_bytes);

        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();

        let client = self.client.clone();

        tokio::spawn(async move {
            let resp = client
                .invoke_model_with_response_stream()
                .model_id(&model_id)
                .content_type("application/json")
                .body(payload_blob)
                .send()
                .await;

            match resp {
                Ok(output) => {
                    let mut response_stream = output.body;

                    loop {
                        match response_stream.recv().await {
                            Ok(Some(aws_sdk_bedrockruntime::types::ResponseStream::Chunk(
                                payload_part,
                            ))) => {
                                if let Some(blob) = &payload_part.bytes {
                                    let data: Cow<'_, str> =
                                        String::from_utf8_lossy(&blob.as_ref());
                                    let value: Value = serde_json::from_str(&data).unwrap();
                                    sender.send(Ok(value)).unwrap();
                                }
                            }
                            Err(err) => {
                                let sdk_error = err;
                                let bedrock_error = BedrockError::from(sdk_error);
                                sender.send(Err(bedrock_error)).unwrap();
                                break;
                            }
                            Ok(None) => {
                                break;
                            }
                            Ok(Some(_)) => {}
                        }
                    }
                }
                Err(err) => {
                    let bedrock_error = BedrockError::from(err);
                    sender.send(Err(bedrock_error)).unwrap();
                }
            }
        });

        Ok(UnboundedReceiverStream::new(receiver))
    }

    /// Generates a raw response from the Bedrock service.
    ///
    /// This function takes a model ID and a payload, sends a request to the Bedrock service,
    /// and returns a response.
    ///
    /// # Arguments
    ///
    /// * `model_id` - A string that represents the model ID.
    /// * `payload` - A `Value` that represents the payload to send in the request.
    ///
    /// # Returns
    ///
    /// This function returns a `Result` that contains a response if the operation was successful,
    /// or an error if the operation failed.
    pub async fn generate_raw(
        &self,
        model_id: String,
        payload: Value,
    ) -> Result<Value, BedrockError> {
        let payload_bytes = serde_json::to_vec(&payload).unwrap();
        let payload_blob = aws_smithy_types::Blob::new(payload_bytes);

        let client = self.client.clone();

        // Invoke the model with the payload
        let resp = client
            .invoke_model()
            .model_id(model_id)
            .content_type("application/json")
            .body(payload_blob)
            .send()
            .await;

        let resp = match resp {
            Ok(resp) => resp,
            Err(err) => return Err(BedrockError::from(err)),
        };

        let response: serde_json::Value = serde_json::from_slice(resp.body().as_ref()).unwrap();
        Ok(response)
    }
}

```

## File: src/bedrock/mod.rs

- Extension: .rs
- Size: 187 bytes
- Created: 2024-04-04 23:44:04
- Modified: 2024-04-04 23:44:04

### Code
```.rs

pub mod bedrock_client;
pub mod model_info;
pub mod models;
pub mod error;

pub use bedrock_client::BedrockClient;
pub use error::BedrockError;
pub use model_info::{ModelName,ModelInfo};
```

## File: src/bedrock/model_info.rs

- Extension: .rs
- Size: 4005 bytes
- Created: 2024-04-05 21:08:53
- Modified: 2024-04-05 21:08:53

### Code
```.rs
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum ModelName {
    AmazonTitanTextG1Express1x,
    AmazonTitanTextG1Lite1x,
    AmazonTitanEmbeddingsG1Text1x,
    AmazonTitanMultimodalEmbeddingsG1x,
    AmazonTitanImageGeneratorG1x,
    AnthropicClaude2x,
    AnthropicClaudeSonnet1x,
    AnthropicClaudeHaiku1x,
    AnthropicClaudeInstantx,
    AI21JurassicMid1x,
    AI21JurassicUltra1x,
    CohereCmdTxt14x,
    CohereCmdLightTxt15x,
    CohereEmbedEnglish3x,
    CohereEmbedMultilingual3x,
    MetaLlama2Chat13B1x,
    MetaLlama2Chat70B1x,
    MistralMistral7BInstruct0x,
    MistralMixtral8X7BInstruct0x,
    MistralLarge,
    StabilityStableDiffusionXL0x,
    StabilityStableDiffusionXL1x,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ModelInfo {
    pub name: ModelName,
    pub text: &'static str,
}

impl ModelInfo {
    pub const MODELS: &'static [ModelInfo] = &[
        ModelInfo {
            name: ModelName::AmazonTitanTextG1Express1x,
            text: "amazon.titan-text-express-v1",
        },
        ModelInfo {
            name: ModelName::AmazonTitanTextG1Lite1x,
            text: "amazon.titan-text-lite-v1",
        },
        ModelInfo {
            name: ModelName::AmazonTitanEmbeddingsG1Text1x,
            text: "amazon.titan-embed-text-v1",
        },
        ModelInfo {
            name: ModelName::AmazonTitanMultimodalEmbeddingsG1x,
            text: "amazon.titan-embed-image-v1",
        },
        ModelInfo {
            name: ModelName::AmazonTitanImageGeneratorG1x,
            text: "amazon.titan-image-generator-v1",
        },
        ModelInfo {
            name: ModelName::AnthropicClaude2x,
            text: "anthropic.claude-v2",
        },
        ModelInfo {
            name: ModelName::AnthropicClaudeSonnet1x,
            text: "anthropic.claude-3-sonnet-20240229-v1:0",
        },
        ModelInfo {
            name: ModelName::AnthropicClaudeHaiku1x,
            text: "anthropic.claude-3-haiku-20240307-v1:0",
        },
        ModelInfo {
            name: ModelName::AnthropicClaudeInstantx,
            text: "anthropic.claude-instant-v1",
        },
        ModelInfo {
            name: ModelName::AI21JurassicMid1x,
            text: "ai21.j2-mid-v1",
        },
        ModelInfo {
            name: ModelName::AI21JurassicUltra1x,
            text: "ai21.j2-ultra-v1",
        },
        ModelInfo {
            name: ModelName::CohereCmdTxt14x,
            text: "cohere.command-text-v14",
        },
        ModelInfo {
            name: ModelName::CohereCmdLightTxt15x,
            text: "cohere.command-light-text-v14",
        },
        ModelInfo {
            name: ModelName::CohereEmbedEnglish3x,
            text: "cohere.embed-english-v3",
        },
        ModelInfo {
            name: ModelName::CohereEmbedMultilingual3x,
            text: "cohere.embed-multilingual-v3",
        },
        ModelInfo {
            name: ModelName::MetaLlama2Chat13B1x,
            text: "meta.llama2-13b-chat-v1",
        },
        ModelInfo {
            name: ModelName::MetaLlama2Chat70B1x,
            text: "meta.llama2-70b-chat-v1",
        },
        ModelInfo {
            name: ModelName::MistralMistral7BInstruct0x,
            text: "mistral.mistral-7b-instruct-v0:2",
        },
        ModelInfo {
            name: ModelName::MistralMixtral8X7BInstruct0x,
            text: "mistral.mixtral-8x7b-instruct-v0:1",
        },
        ModelInfo {
            name: ModelName::MistralLarge,
            text: "mistral.mistral-large-2402-v1:0",
        },
        ModelInfo {
            name: ModelName::StabilityStableDiffusionXL0x,
            text: "stability.stable-diffusion-xl-v0",
        },
        ModelInfo {
            name: ModelName::StabilityStableDiffusionXL1x,
            text: "stability.stable-diffusion-xl-v1",
        },
    ];

    pub fn from_model_name(name: ModelName) -> String  {
        ModelInfo::MODELS.iter().find(|model| model.name == name).unwrap().text.to_string()
    }

}
```

## File: src/examples/demo_mistral_stream.rs

- Extension: .rs
- Size: 1677 bytes
- Created: 2024-04-07 11:28:05
- Modified: 2024-04-07 11:28:05

### Code
```.rs
use futures::TryStreamExt;

use crate::bedrock::model_info::{ModelInfo, ModelName};
use crate::bedrock::models::mistral::MistralClient;
use crate::bedrock::models::mistral::MistralOptions;
use crate::bedrock::models::mistral::MistralRequestBuilder;


pub async fn demo_mistra_with_stream(model_id: &str, prompt: &str) {

    let mistral_otions
     = MistralOptions::new()
        .profile_name("bedrock")
        .region("us-west-2");

    let client = MistralClient::new(mistral_otions).await;



    let request = MistralRequestBuilder::new(prompt.to_owned())
        .max_tokens(200)
        .temperature(0.5)
        .top_p(0.9)
        .top_k(100)
        .build();

    let response_stream = client
        .generate_with_stream(
            model_id.to_string(),
            &request
        )
        .await;
     
    let response_stream = match response_stream {
        Ok(response_stream) => response_stream,
        Err(e) => {
            println!("Error: {:?}", e);
            return;
        }
    };

    // consumme the stream and print the response
    response_stream
        .try_for_each(|chunk| async move {
            let json_display = serde_json::to_string_pretty(&chunk).unwrap();
            println!("{:?}", json_display);
            Ok(())
        })
        .await
        .unwrap();

}

// Test
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_demo_chat_mistral_with_stream() {
        let model_id = ModelInfo::from_model_name(ModelName::MistralMixtral8X7BInstruct0x);
        let prompt = "<s>[INST] What is the capital of France ?[/INST]";
        demo_mistra_with_stream(&model_id, prompt).await;
    }
}

```

## File: src/examples/demo_bedrock_raw_stream.rs

- Extension: .rs
- Size: 1707 bytes
- Created: 2024-04-02 22:13:19
- Modified: 2024-04-02 22:13:19

### Code
```.rs
use futures::TryStreamExt;
use std::io;
use std::io::Write;

use crate::bedrock::bedrock_client::{BedrockClient, BedrockClientOptions};
use crate::bedrock::model_info::{ModelInfo, ModelName};

pub async fn demo_generate_raw_stream() {
    let model_id = ModelInfo::from_model_name(ModelName::AnthropicClaudeHaiku1x);
    let profile_name = "bedrock";
    let region = "us-west-2";

    let prompt = "Hi. In a short paragraph, explain what you can do.";

    let payload = serde_json::json!({
        "anthropic_version": "bedrock-2023-05-31",
        "max_tokens": 1000,
        "messages": [{
            "role": "user",
            "content": [{
                "type": "text",
                "text": prompt
            }]
        }]
    });

    let options = BedrockClientOptions::new()
        .profile_name(profile_name)
        .region(region);
    

    let client = BedrockClient::new(options).await;

    let stream = client
        .generate_raw_stream(
            model_id.to_string(),
            payload,
        )
        .await;

    let stream = match stream {
        Ok(stream) => stream,
        Err(err) => {
            println!("Error: {:?}", err);
            return;
        }
    };

    // consumme the stream and print the response
    stream
        .try_for_each(|chunk| async move {
            println!("{:?}", chunk);
            // Flush the output to ensure the prompt is displayed.
            io::stdout().flush().unwrap();
            Ok(())
        })
        .await
        .unwrap();
}

// Write a test

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_demo_generate_raw_stream() {
        demo_generate_raw_stream().await;
    }
}

```

## File: src/examples/demo_claude_multimedia.rs

- Extension: .rs
- Size: 1695 bytes
- Created: 2024-04-07 11:23:55
- Modified: 2024-04-07 11:23:55

### Code
```.rs
use futures::TryStreamExt;

use crate::{bedrock::models::claude::{claude_client::{ClaudeClient, ClaudeOptions}, claude_request_message::{ChatOptions, ConversationRequest, Message}}, fetch_and_base64_encode_image};


pub async fn demo_claude_multimedia() {
    let claude_options = ClaudeOptions::new().profile_name("bedrock").region("us-west-2");

    let client = ClaudeClient::new(claude_options).await;

    let image_url = "./data/mario.png";

    let mut conversation_request = ConversationRequest::default();

    let input_text = "What's in this image?".to_string(); // Convert a string literal to a String object
    let image = fetch_and_base64_encode_image(image_url).await.unwrap().to_string(); // Example base64 encoded image data
    let mime_type = "image/png".to_string(); // MIME type for the image

    // Now, you can call the function with the correct types
    let message = Message::new_user_message_with_image(&input_text, &image, &mime_type);

    conversation_request.messages.push(message);

    let chat_options = ChatOptions::default().with_temperature(0.7).with_max_tokens(100);

    // display the conversation request, JSON pretty print
    println!("{}", serde_json::to_string_pretty(&conversation_request).unwrap());

    let response_stream = client
        .chat_with_stream(&conversation_request, &chat_options).await
        .unwrap();

    response_stream
        .try_for_each(|chunk| async move {
            println!("{:?}", chunk);
            Ok(())
        }).await
        .unwrap();
}

// Test
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_demo_claude_multimedia() {
        demo_claude_multimedia().await;
    }
}

```

## File: src/examples/demo_claude_chat_stream.rs

- Extension: .rs
- Size: 3048 bytes
- Created: 2024-04-07 11:23:42
- Modified: 2024-04-07 11:23:42

### Code
```.rs
use futures::TryStreamExt;

use crate::bedrock::model_info::{ModelInfo, ModelName};
use crate::bedrock::models::claude::claude_client::{ClaudeClient, ClaudeOptions};
use crate::bedrock::models::claude::claude_request_message::ConversationRequest;
use crate::bedrock::models::claude::claude_request_message::Message;
use crate::bedrock::models::claude::claude_request_message::{ChatOptions, StreamResultData};

pub async fn demo_chat_claude_with_stream() {
    let claude_options = ClaudeOptions::new()
        .profile_name("bedrock")
        .region("us-west-2");

    let client = ClaudeClient::new(claude_options).await;

    let mut conversation_request = ConversationRequest::default();

    conversation_request
        .messages
        .push(Message::new_user_message(
            "What is the capital of France ?".to_owned(),
        ));

    println!(
        "{}",
        serde_json::to_string_pretty(&conversation_request).unwrap()
    );

    let chat_options = ChatOptions::default()
        .with_model_id(ModelInfo::from_model_name(
            ModelName::AnthropicClaudeHaiku1x,
        ))
        .with_temperature(0.5);

    let response_stream = client
        .chat_with_stream(&conversation_request, &chat_options)
        .await;

    let response_stream = match response_stream {
        Ok(response_stream) => response_stream,
        Err(e) => {
            println!("Error: {:?}", e);
            return;
        }
    };

    // consumme the stream and print the response
    response_stream
        .try_for_each(|chunk| async move {
            display_streamresult_data(chunk);
            Ok(())
        })
        .await
        .unwrap();
}

fn display_streamresult_data(data: StreamResultData) {
    match data {
        StreamResultData::ContentBlockStart(content_block_start) => {
            println!("ContentBlockStart: {:?}", content_block_start);
        }
        StreamResultData::ContentBlockStop(content_block_end) => {
            println!("ContentBlockEnd: {:?}", content_block_end);
        }
        StreamResultData::MessageStart(message_start) => {
            println!("MessageStart: {:?}", message_start);
        }
        StreamResultData::MessageStop(message_end) => {
            println!("MessageStop: {:?}", message_end);
        }
        StreamResultData::MessageDelta(message_delta) => {
            println!("MessageDelta: {:?}", message_delta);
        }
        StreamResultData::ContentBlockStart(content_block_start) => {
            println!("ContentBlockStart: {:?}", content_block_start);
        }
        StreamResultData::ContentBlockStop(content_block_end) => {
            println!("ContentBlockEnd: {:?}", content_block_end);
        }
        StreamResultData::ContentBlockDelta(content_block_delta) => {
            println!("ContentBlockDelta: {:?}", content_block_delta);
        }
    }
}

// Test
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_demo_chat_claude_with_stream() {
        demo_chat_claude_with_stream().await;
    }
}

```

## File: src/examples/demo_ollama.rs

- Extension: .rs
- Size: 3367 bytes
- Created: 2024-04-07 11:16:44
- Modified: 2024-04-07 11:16:44

### Code
```.rs
use std::io::{self, Write};
use std::u32;

use futures::stream::TryStream;
use futures_util::TryStreamExt;

use crate::ollama::{ChatRequestBuilder, ChatResponse, Message};
use crate::ollama::{GenerateRequestBuilder, GenerateResponse};
use crate::ollama::OllamaClient;
use crate::ollama::OllamaError;


pub async fn chat_response_loop(max_loop: u32) {
    let client = OllamaClient::new("http://localhost:11434".to_string());

    let mut messages = Vec::new();
    let mut counter = 0;

    loop {
        let input = prompt_input("\nUser: ").unwrap();
        messages.push(Message {
            role: "user".to_string(),
            content: input,
            images: vec![],
        });

        let request = ChatRequestBuilder::new("mistral".to_string())
            .messages(messages.clone())
            .build();

        let response_stream = client.chat(request).await.unwrap();

        let response = process_and_collect_chat_response(response_stream, |chunk| {
            print!("{}", chunk);
            io::stdout().flush().unwrap();
        })
        .await
        .unwrap();
        // get last response from the chat

        messages.push(Message {
            role: "assistant".to_string(),
            content: response,
            images: vec![],
        });

        counter += 1;
        if counter >= max_loop {
            break;
        }
    }
}

pub async fn generate_response_loop(max_loop: usize) {
    let client = OllamaClient::new("http://localhost:11434".to_string());

    let mut counter = 0;
    loop {
        let input = prompt_input("\n> ").unwrap();
        let request = GenerateRequestBuilder::new("mistral".to_string())
            .prompt(input)
            .build();

        let response = client.generate(request).await.unwrap();

        print_generate_response(response).await.unwrap();

        counter += 1;
        if counter >= max_loop {
            break;
        }
    }
}

fn prompt_input(prompt: &str) -> Result<String, std::io::Error> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

async fn process_and_collect_chat_response<F>(
    response: impl TryStream<Ok = ChatResponse, Error = OllamaError>,
    callback: F,
) -> Result<String, OllamaError>
where
    F: Fn(&str) + Send + Sync + 'static,
{
    let words = response
        .try_fold(String::new(), |mut f, chunk| async {
            let response = chunk.message.content;
            callback(&response);
            f.push_str(&response);
            Ok(f)
        })
        .await
        .unwrap();

    Ok(words)
}

pub async fn print_generate_response(
    response: impl TryStream<Ok = GenerateResponse, Error = OllamaError>,
) -> Result<(), OllamaError> {
    response
        .try_for_each(|chunk| async {
            let response = chunk.response;
            print!("{}", response);
            // Flush the output to ensure the prompt is displayed.
            io::stdout().flush().unwrap();
            Ok(())
        })
        .await
}

// Create a test

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn test_chat_response_loop() {
        // chat_response_loop(1).await;
    }

    #[tokio::test]
    async fn test_generate_response_loop() {
        //      generate_response_loop(1).await;
    }
}

```

## File: src/examples/mod.rs

- Extension: .rs
- Size: 300 bytes
- Created: 2024-04-07 07:56:05
- Modified: 2024-04-07 07:56:05

### Code
```.rs
pub mod simple_examples;

pub mod demo_ollama;
pub mod demo_bedrock_raw_generate;
pub mod demo_bedrock_raw_stream;
pub mod demo_bedrock_raw_mistral;
pub mod demo_claude_chat;
pub mod demo_claude_chat_stream;
pub mod demo_claude_multimedia;
pub mod demo_ollama_embedding;
pub mod demo_mistral_stream;

```

## File: src/examples/demo_bedrock_raw_generate.rs

- Extension: .rs
- Size: 1188 bytes
- Created: 2024-04-02 22:13:19
- Modified: 2024-04-02 22:13:19

### Code
```.rs
use crate::bedrock::bedrock_client::{BedrockClient, BedrockClientOptions};
use crate::bedrock::model_info::{ModelInfo, ModelName};

pub async fn demo_generate_raw() {
    let model_id = ModelInfo::from_model_name(ModelName::AnthropicClaudeHaiku1x);
    let profile_name = "bedrock";
    let region = "us-west-2";

    let prompt = "Hi. In a short paragraph, explain what you can do.";

    let payload = serde_json::json!({
        "anthropic_version": "bedrock-2023-05-31",
        "max_tokens": 1000,
        "messages": [{
            "role": "user",
            "content": [{
                "type": "text",
                "text": prompt
            }]
        }]
    });

    let options = BedrockClientOptions::new()
        .profile_name(profile_name)
        .region(region);
    

    let client = BedrockClient::new(options).await;

    let result = client
        .generate_raw(
            model_id.to_string(),
            payload,
        )
        .await
        .unwrap();

    println!("{:?}", result);
}

// Create a test

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_demo_generate_raw() {
        demo_generate_raw().await;
    }
}

```

## File: src/examples/demo_claude_chat.rs

- Extension: .rs
- Size: 1635 bytes
- Created: 2024-04-07 07:43:07
- Modified: 2024-04-07 07:43:07

### Code
```.rs
use crate::bedrock::model_info::{ModelInfo, ModelName};
use crate::bedrock::models::claude::claude_client::{ClaudeClient, ClaudeOptions};
use crate::bedrock::models::claude::claude_request_message::ChatOptions;
use crate::bedrock::models::claude::claude_request_message::ConversationRequest;
use crate::bedrock::models::claude::claude_request_message::Message;

pub async fn demo_chat_claude() {
    let claude_options = ClaudeOptions::new()
        .profile_name("bedrock")
        .region("us-west-2");

    let client = ClaudeClient::new(claude_options).await;

    let mut conversation_request = ConversationRequest::default();

    conversation_request
        .messages
        .push(Message::new_user_message(
            "What is the capital of France ?".to_owned(),
        ));

    println!(
        "{}",
        serde_json::to_string_pretty(&conversation_request).unwrap()
    );

    let chat_options =
        ChatOptions::default()
            .with_temperature(0.5)
            .with_model_id(ModelInfo::from_model_name(
                ModelName::AnthropicClaudeHaiku1x,
            ));

    let response = client.chat(&conversation_request, &chat_options).await;

    match response {
        Ok(response) => {
            println!("{:?}", response);
            let json_display = serde_json::to_string_pretty(&response).unwrap();
            println!("{:?}", json_display);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}

// Test
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_demo_chat_claude() {
        demo_chat_claude().await;
    }
}

```

## File: src/examples/demo_ollama_embedding.rs

- Extension: .rs
- Size: 938 bytes
- Created: 2024-04-07 11:24:13
- Modified: 2024-04-07 11:24:13

### Code
```.rs
use crate::ollama::{EmbeddingsRequestBuilder, OllamaClient};

pub async fn demo_ollama_embedding() -> Result<(), Box<dyn std::error::Error>> {
    let client = OllamaClient::new("http://localhost:11434".to_string());

    let prompt = "The quick brown fox jumps over the lazy dog.";

    let request = EmbeddingsRequestBuilder::new("nomic-embed-text".to_string(), prompt.to_string())
        .keep_alive("10m".to_string())
        .build();

    match client.embeddings(request).await {
        Ok(response) => {
            // Print embeddings dimensions
            println!("Embeddings dimensions: {:?}", response.embedding.len());
            println!("Embeddings: {:?}", response);
        }
        Err(error) => {
            eprintln!("Error: {:?}", error);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn main() {
        demo_ollama_embedding().await.unwrap();
    }
}

```

## File: src/examples/demo_bedrock_raw_mistral.rs

- Extension: .rs
- Size: 2332 bytes
- Created: 2024-04-05 21:08:53
- Modified: 2024-04-05 21:08:53

### Code
```.rs
use futures::TryStreamExt;
use std::io;
use std::io::Write;

use crate::bedrock::bedrock_client::{BedrockClient, BedrockClientOptions};


pub async fn demo_bedrock_mistral_raw_stream(model_id: &str, prompt: &str) {
    let profile_name = "bedrock";
    let region = "us-west-2";


    let payload = serde_json::json!({
        "prompt": prompt,
        "max_tokens" : 200,
        "stop" : ["[INST]"],    
        "temperature": 0.5,
        "top_p": 0.9,
        "top_k": 100,
    });

    let options = BedrockClientOptions::new()
        .profile_name(profile_name)
        .region(region);
    

    let client = BedrockClient::new(options).await;

    let stream = client
        .generate_raw_stream(
            model_id.to_string(),
            payload,
        )
        .await;

    let stream = match stream {
        Ok(stream) => stream,
        Err(err) => {
            println!("Error: {:?}", err);
            return;
        }
    };

    // consumme the stream and print the response
    stream
        .try_for_each(|chunk| async move {
            println!("{:?}", chunk);
            // Flush the output to ensure the prompt is displayed.
            io::stdout().flush().unwrap();
            Ok(())
        })
        .await
        .unwrap();
}

// Write a test

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bedrock::model_info::{ModelInfo, ModelName};

    #[tokio::test]
    async fn test_demo_bedrock_mistral_raw_stream_8x7() {
        let model_id = ModelInfo::from_model_name(ModelName::MistralMixtral8X7BInstruct0x);
        let prompt = "<s>[INST] What is your favourite condiment? [/INST]";
        demo_bedrock_mistral_raw_stream(&model_id,&prompt).await;
    }

    #[tokio::test]
    async fn test_demo_bedrock_mistral_raw_stream_7b() {
        let model_id = ModelInfo::from_model_name(ModelName::MistralMistral7BInstruct0x);
        let prompt = "<s>[INST] What is your favourite condiment? [/INST]";
        demo_bedrock_mistral_raw_stream(&model_id,&prompt).await;
    }

    #[tokio::test]
    async fn test_demo_bedrock_mistral_raw_stream_large() {
        let model_id = ModelInfo::from_model_name(ModelName::MistralLarge);
        let prompt = "<s>[INST] What is your favourite condiment? [/INST]";
        demo_bedrock_mistral_raw_stream(&model_id,&prompt).await;
    }


}

```

## File: src/ollama/error.rs

- Extension: .rs
- Size: 1106 bytes
- Created: 2024-04-04 21:29:59
- Modified: 2024-04-04 21:29:59

### Code
```.rs
use serde_json::Error as SerdeJsonError;
use thiserror::Error;



#[derive(Error, Debug)]
pub enum OllamaError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] SerdeJsonError),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Utf8 error: {0}")]
    Utf8(#[from] std::str::Utf8Error),

    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    #[error("Bad Request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Not Found: {0}")]
    NotFound(String),

    #[error("Too Many Requests: {0}")]
    TooManyRequests(String),

    #[error("Internal Server Error: {0}")]
    InternalServerError(String),

    #[error("Unknown API Error: {0}")]
    UnknownApiError(String),

    #[error("Request Builder Error: {0}")]
    RequestBuilderError(String),

    #[error("Deserialization Error: {0}")]
    DeserializationError(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}
```

## File: src/ollama/ollama_client.rs

- Extension: .rs
- Size: 5477 bytes
- Created: 2024-04-04 23:28:56
- Modified: 2024-04-04 23:28:56

### Code
```.rs
use crate::ollama::model::{ChatRequest, ChatResponse, GenerateRequest, GenerateResponse};
use futures::stream::TryStream;
use futures::stream::TryStreamExt;
use reqwest::{Client, RequestBuilder};
use serde::de::DeserializeOwned;

use super::error::OllamaError;
use crate::ollama::model::{EmbeddingsRequest, EmbeddingsResponse};

pub struct OllamaClient {
    client: Client,
    base_url: String,
}

async fn fetch_stream<T>(
    request: RequestBuilder,
) -> Result<impl TryStream<Ok = T, Error = OllamaError>, OllamaError>
where
    T: DeserializeOwned,
{
    let response = request.send().await?;

    let status = response.status();
    let body = response.bytes_stream();

    if status.is_success() {
        Ok(body
            .map_err(OllamaError::from)
            .and_then(|chunk| async move {
                let chunk = serde_json::from_slice(&chunk).map_err(OllamaError::from)?;
                Ok(chunk)
            }))
    } else {
        let message = format!("API request failed with status code: {}", status);
        match status.as_u16() {
            400 => Err(OllamaError::BadRequest(message)),
            401 => Err(OllamaError::Unauthorized(message)),
            403 => Err(OllamaError::Forbidden(message)),
            404 => Err(OllamaError::NotFound(message)),
            429 => Err(OllamaError::TooManyRequests(message)),
            500 => Err(OllamaError::InternalServerError(message)),
            _ => Err(OllamaError::UnknownApiError(message)),
        }
    }
}

impl OllamaClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    pub async fn generate(
        &self,
        request: GenerateRequest,
    ) -> Result<impl TryStream<Ok = GenerateResponse, Error = OllamaError>, OllamaError> {
        let url = format!("{}/api/generate", self.base_url);

        let request = self.client.post(&url).json(&request);

        let stream = fetch_stream::<GenerateResponse>(request).await?;

        Ok(stream)
    }

    pub async fn chat(
        &self,
        request: ChatRequest,
    ) -> Result<impl TryStream<Ok = ChatResponse, Error = OllamaError>, OllamaError> {
        let url = format!("{}/api/chat", self.base_url);

        let request = self.client.post(&url).json(&request);

        let stream = fetch_stream::<ChatResponse>(request).await?;

        Ok(stream)
    }

    pub async fn embeddings(
        &self,
        request: EmbeddingsRequest,
    ) -> Result<EmbeddingsResponse, OllamaError> {
        let url = format!("{}/api/embeddings", self.base_url);

        let response = self.client.post(&url).json(&request).send().await?;

        let status = response.status();
        let body = response.text().await?;

        if status.is_success() {
            let embedding_response: EmbeddingsResponse =
                serde_json::from_str(&body).map_err(OllamaError::from)?;
            Ok(embedding_response)
        } else {
            let message = format!("API request failed with status code: {}", status);
            match status.as_u16() {
                400 => Err(OllamaError::BadRequest(message)),
                401 => Err(OllamaError::Unauthorized(message)),
                403 => Err(OllamaError::Forbidden(message)),
                404 => Err(OllamaError::NotFound(message)),
                429 => Err(OllamaError::TooManyRequests(message)),
                500 => Err(OllamaError::InternalServerError(message)),
                _ => Err(OllamaError::UnknownApiError(message)),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ollama::EmbeddingsRequestBuilder;

    use super::*;

    #[tokio::test]
    async fn test_embeddings() {
        let client = OllamaClient::new("http://localhost:11434".to_string());

        let request = crate::ollama::EmbeddingsRequestBuilder::new(
            "nomic-embed-text".to_string(),
            "Here is an article about llamas...".to_string(),
        )
        .options(serde_json::json!({ "temperature": 0.8 }))
        .keep_alive("10m".to_string())
        .build();

        let response = client.embeddings(request).await;

        let response = match response {
            Ok(response) => response,
            Err(err) => panic!("Error: {:?}", err),
        };

        print!("Embeddings: {:?}", response);

        assert_eq!(response.embedding.len(), 768);
    }

    #[tokio::test]
    async fn test_embeddings_builder() {
        let json_request: String = EmbeddingsRequestBuilder::new(
            "all-minilm".to_string(),
            "Here is an article about llamas...".to_string(),
        )
        .options(serde_json::json!({ "temperature": 0.8 }))
        .keep_alive("10m".to_string())
        .into();

        let expected_json = r#"{"model":"all-minilm","prompt":"Here is an article about llamas...","options":{"temperature":0.8},"keep_alive":"10m"}"#;
        assert_eq!(json_request, expected_json);
    }

    #[tokio::test]
    async fn test_embeddings_error() {
        let client = OllamaClient::new("http://localhost:11434".to_string());

        let request = EmbeddingsRequestBuilder::new(
            "invalid-model".to_string(),
            "Here is an article about llamas...".to_string(),
        )
        .build();

        let result = client.embeddings(request).await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), OllamaError::NotFound(_)));
    }
}

```

## File: src/ollama/mod.rs

- Extension: .rs
- Size: 353 bytes
- Created: 2024-04-04 23:28:56
- Modified: 2024-04-04 23:28:56

### Code
```.rs
pub mod ollama_client;
pub mod model;
pub mod error;

pub use error::OllamaError;
pub use ollama_client::OllamaClient;
pub use model::{ GenerateRequest, GenerateRequestBuilder, GenerateResponse };
pub use model::{ ChatRequest, ChatRequestBuilder, ChatResponse, Message };
pub use model::{ EmbeddingsRequest,EmbeddingsResponse, EmbeddingsRequestBuilder};
```

## File: src/ollama/model.rs

- Extension: .rs
- Size: 12520 bytes
- Created: 2024-04-06 08:30:26
- Modified: 2024-04-06 08:30:26

### Code
```.rs

use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use serde_json::Value;
use pin_project::pin_project;

use super::error::OllamaError;

/// Represents a request to generate text using the Ollama API.
#[derive(Debug, Serialize, Clone)]
pub struct GenerateRequest {
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub images: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Vec<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_alive: Option<String>,
}
/// Represents a response from the Ollama API for a generate request.
#[pin_project]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GenerateResponse {
    pub model: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub response: String,
    pub done: bool,
    pub context: Option<Vec<u32>>,
    pub total_duration: Option<u128>,
    pub load_duration: Option<u128>,
    pub prompt_eval_count: Option<u32>,
    pub prompt_eval_duration: Option<u128>,
    pub eval_count: Option<u32>,
    pub eval_duration: Option<u128>,
}

// Implement the TryFrom trait to convert a JSON string into a GenerateResponse.
impl TryFrom<&str> for GenerateResponse {
    type Error = OllamaError;

    fn try_from(json: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(json).map_err(|e| {
            OllamaError::DeserializationError(format!(
                "Failed to deserialize GenerateResponse: {}",
                e
            ))
        })
    }
}

// Represents a builder for constructing a GenerateRequest.
pub struct GenerateRequestBuilder {
    model: String,
    prompt: Option<String>,
    images: Option<Vec<String>>,
    format: Option<String>,
    options: Option<Value>,
    system: Option<String>,
    template: Option<String>,
    context: Option<Vec<u32>>,
    stream: Option<bool>,
    raw: Option<bool>,
    keep_alive: Option<String>,
}

impl GenerateRequestBuilder {
    // Create a new GenerateRequestBuilder with the specified model.
    // The model is a required field and must be provided.
    pub fn new(model: String) -> Self {
        Self {
            model,
            prompt: None,
            images: None,
            format: None,
            options: None,
            system: None,
            template: None,
            context: None,
            stream: None,
            raw: None,
            keep_alive: None,
        }
    }

    // Set the prompt field of the GenerateRequestBuilder.
    // This field is used to provide a prompt for the generation process.
    // The value should be a string representing the prompt.
    // If the value is not provided, the prompt will be empty.
    pub fn prompt(mut self, prompt: String) -> Self {
        self.prompt = Some(prompt);
        self
    }
    

    // Set the images field of the GenerateRequestBuilder.
    // This field is used to provide a list of images for the generation process.
    // The value should be a vector of strings representing the images.
    // The images should be base64 encoded strings.
    // If the value is not provided, the images will be empty.
    pub fn images(mut self, images: Vec<String>) -> Self {
        self.images = Some(images);
        self
    }

    // Set the format field of the GenerateRequestBuilder.
    // This field is used to specify the format of the response.
    // The value should be a string representing the format.
    // If the value is not provided, the response will be returned as a string.
    pub fn format(mut self, format: String) -> Self {
        self.format = Some(format);
        self
    }

    // Set the options field of the GenerateRequestBuilder.
    // This field is used to provide options for the generation process.
    // The value should be a JSON object representing the options.
    pub fn options(mut self, options: Value) -> Self {
        self.options = Some(options);
        self
    }

    // Set the system field of the GenerateRequestBuilder.
    // This field is used to provide a system prompt for the generation process.
    // The value should be a string representing the system prompt.
    // If the value is not provided, the system prompt will be empty.
    pub fn system(mut self, system: String) -> Self {
        self.system = Some(system);
        self
    }

    // Set the template field of the GenerateRequestBuilder.
    // This field is used to provide a template for the generation process.
    // The value should be a string representing the template.
    // If the value is not provided, the template will be empty.
    pub fn template(mut self, template: String) -> Self {
        self.template = Some(template);
        self
    }

    // Set the context field of the GenerateRequestBuilder.
    // This field is used to provide a context for the generation process.
    // The value should be a vector of integers representing the context.
    // If the value is not provided, the context will be empty.
    pub fn context(mut self, context: Vec<u32>) -> Self {
        self.context = Some(context);
        self
    }


    // Set the stream field of the GenerateRequestBuilder.
    // This field is used to return a stream of responses from the API.
    // If the value is not provided, the response will be processed and returned as a string.
    pub fn stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }

    // Set the raw field of the GenerateRequestBuilder.
    // This field is used to return the raw response from the API without any additional processing.
    // If the value is not provided, the response will be processed and returned as a string.
    pub fn raw(mut self, raw: bool) -> Self {
        self.raw = Some(raw);
        self
    }

    // Set the keep_alive field of the GenerateRequestBuilder.
    // This field is used to keep the connection alive for a specified duration.
    // The value should be a string representing the duration in seconds.
    // If the value is not provided, the connection will be closed after the response is received.
    pub fn keep_alive(mut self, keep_alive: String) -> Self {
        self.keep_alive = Some(keep_alive);
        self
    }

    // Build the GenerateRequest struct from the builder.
    pub fn build(self) -> GenerateRequest {
        GenerateRequest {
            model: self.model,
            prompt: self.prompt,
            images: self.images.unwrap_or_default(),
            format: self.format,
            options: self.options,
            system: self.system,
            template: self.template,
            context: self.context,
            raw: self.raw, 
            stream: self.stream,
            keep_alive: self.keep_alive,
        }
    }
}

// Implement the From trait to convert a GenerateRequestBuilder into a JSON string.
impl From<GenerateRequestBuilder> for String {
    fn from(request: GenerateRequestBuilder) -> Self {
        serde_json::to_string(&request.build()).unwrap()
    }
}
/// Represents a request to initiate a chat session with the Ollama API.
#[derive(Debug, Serialize, Clone)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_alive: Option<String>,
}

// Represents a message in a chat session, containing the role and content.
#[derive(Debug, Serialize,Deserialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub images: Vec<String>,
}

/// Represents a response from the Ollama API for a chat request.
#[pin_project]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatResponse {
    pub model: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub message: Message,
    pub done: bool,
    pub total_duration: Option<u128>,
    pub load_duration: Option<u128>,
    pub prompt_eval_count: Option<u32>,
    pub prompt_eval_duration: Option<u128>,
    pub eval_count: Option<u32>,
    pub eval_duration: Option<u128>,
}

impl TryFrom<&str> for ChatResponse {
    type Error = OllamaError;

    fn try_from(json: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(json).map_err(|e| {
            OllamaError::DeserializationError(format!(
                "Failed to deserialize ChatResponse: {}",
                e
            ))
        })
    }
}

pub struct ChatRequestBuilder {
    model: String,
    messages: Vec<Message>,
    format: Option<String>,
    options: Option<Value>,
    template: Option<String>,
    stream: Option<bool>,
    keep_alive: Option<String>,
}

impl ChatRequestBuilder {
    pub fn new(model: String) -> Self {
        Self {
            model,
            messages: Vec::new(),
            format: None,
            options: None,
            template: None,
            stream: None,
            keep_alive: None,
        }
    }
    
    pub fn messages(mut self, messages: Vec<Message>) -> Self {
        self.messages = messages;
        self
    }

    pub fn format(mut self, format: String) -> Self {
        self.format = Some(format);
        self
    }

    pub fn options(mut self, options: Value) -> Self {
        self.options = Some(options);
        self
    }

    pub fn template(mut self, template: String) -> Self {
        self.template = Some(template);
        self
    }

    pub fn stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }

    pub fn keep_alive(mut self, keep_alive: String) -> Self {
        self.keep_alive = Some(keep_alive);
        self
    }

    pub fn build(self) -> ChatRequest {
        ChatRequest {
            model: self.model,
            messages: self.messages,
            format: self.format,
            options: self.options,
            template: self.template,
            stream: self.stream,
            keep_alive: self.keep_alive,
        }
    }
}

impl From<ChatRequestBuilder> for String {
    fn from(request: ChatRequestBuilder) -> Self {
        serde_json::to_string(&request.build()).unwrap()
    }
}

#[derive(Debug, Serialize)]
pub struct EmbeddingsRequest {
    pub model: String,
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_alive: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EmbeddingsResponse {
    pub embedding: Vec<f32>,
}


pub struct EmbeddingsRequestBuilder {
    model: String,
    prompt: String,
    options: Option<Value>,
    keep_alive: Option<String>,
}

impl EmbeddingsRequestBuilder {
    pub fn new(model: String, prompt: String) -> Self {
        Self {
            model,
            prompt,
            options: None,
            keep_alive: None,
        }
    }

    pub fn options(mut self, options: Value) -> Self {
        self.options = Some(options);
        self
    }

    pub fn keep_alive(mut self, keep_alive: String) -> Self {
        self.keep_alive = Some(keep_alive);
        self
    }

    pub fn build(self) -> EmbeddingsRequest {
        EmbeddingsRequest {
            model: self.model,
            prompt: self.prompt,
            options: self.options,
            keep_alive: self.keep_alive,
        }
    }
}

impl From<EmbeddingsRequestBuilder> for String {
    fn from(request: EmbeddingsRequestBuilder) -> Self {
        serde_json::to_string(&request.build()).unwrap()
    }
}
```

## File: src/bedrock/models/mod.rs

- Extension: .rs
- Size: 32 bytes
- Created: 2024-04-05 21:08:53
- Modified: 2024-04-05 21:08:53

### Code
```.rs
pub mod claude;
pub mod mistral;
```

## File: src/bedrock/models/claude/claude_request_message.rs

- Extension: .rs
- Size: 12666 bytes
- Created: 2024-04-07 08:28:28
- Modified: 2024-04-07 08:28:28

### Code
```.rs
use serde::{Deserialize, Serialize};


pub struct ChatOptions {
    pub model_id: String,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub top_k: Option<u32>,
    pub max_tokens: u32,
    pub stop_sequences: Option<Vec<String>>,
}

impl Default for ChatOptions {
    fn default() -> Self {
        ChatOptions {
            model_id: "anthropic.claude-3-haiku-20240307-v1:0".to_string(),
            temperature: Some(0.5),
            top_p: Some(1.0),
            top_k: Some(50),
            max_tokens: 100,
            stop_sequences: Some(vec![]),
        }
    }
}

impl ChatOptions {
    pub fn with_model_id(mut self, model_id: String) -> Self {
        self.model_id = model_id;
        self
    }

    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    pub fn with_top_p(mut self, top_p: f32) -> Self {
        self.top_p = Some(top_p);
        self
    }

    pub fn with_top_k(mut self, top_k: u32) -> Self {
        self.top_k = Some(top_k);
        self
    }

    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = max_tokens;
        self
    }

    pub fn with_stop_sequences(mut self, stop_sequences: Vec<String>) -> Self {
        self.stop_sequences = Some(stop_sequences);
        self
    }

    pub fn add_stop_sequence(mut self, stop_sequence: String) -> Self {
        match &mut self.stop_sequences {
            Some(sequences) => sequences.push(stop_sequence),
            None => self.stop_sequences = Some(vec![stop_sequence]),
        }
        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageContent {
    Text(String),
    Blocks(Vec<ContentBlock>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: MessageContent,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ContentBlock {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image")]
    Image { source: ImageSource },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageSource {
    #[serde(rename = "type")]
    pub source_type: String,
    pub media_type: String,
    pub data: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConversationRequest {
    pub system: Option<String>,
    pub messages: Vec<Message>,
    pub max_tokens: Option<i32>,
    pub anthropic_version: String,
}

impl Default for ConversationRequest {
    fn default() -> Self {
        ConversationRequest {
            system: Some("Your are a useful assistant.".to_string()),
            messages: Vec::new(),
            max_tokens: Some(1024),
            anthropic_version: "bedrock-2023-05-31".to_string(),
        }
    }
}

impl Message {
    pub fn new_user_message(content: impl Into<MessageContent>) -> Self {
        Message {
            role: Role::User,
            content: content.into(),
        }
    }

    pub fn new_assistant_message(content: impl Into<MessageContent>) -> Self {
        Message {
            role: Role::Assistant,
            content: content.into(),
        }
    }

    pub fn new_user_message_with_image(text: &String, image: &String, mime_type: &String) -> Self {
        let image_block = ContentBlock::Image {
            source: ImageSource {
                source_type: "base64".to_string(),
                media_type: mime_type.to_string(),
                data: image.to_string(),
            },
        };

        let text_block = ContentBlock::Text { text: text.to_string() };


        Message {
            role: Role::User,
            content: MessageContent::Blocks(vec![text_block, image_block]),
        }
    }
}

impl From<String> for MessageContent {
    fn from(text: String) -> Self {
        MessageContent::Text(text)
    }
}

impl From<Vec<ContentBlock>> for MessageContent {
    fn from(blocks: Vec<ContentBlock>) -> Self {
        MessageContent::Blocks(blocks)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StopReason {
    #[serde(rename = "end_turn")]
    EndTurn,
    #[serde(rename = "max_tokens")]
    MaxTokens,
    #[serde(rename = "stop_sequence")]
    StopSequence,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConversationResponse {
    pub id: String,
    pub model: String,
    #[serde(rename = "type")]
    pub response_type: String,
    pub role: Role,
    pub content: Vec<ContentBlock>,
    pub stop_reason: StopReason,
    pub stop_sequence: Option<String>,
    pub usage: UsageInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsageInfo {
    pub input_tokens: i32,
    pub output_tokens: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamResult {
    #[serde(rename = "type")]
    pub result_type: String,
    #[serde(flatten)]
    pub data: serde_json::Value,
}


#[derive(Debug, Serialize, Deserialize)]
pub enum StreamResultData {
    MessageStart(MessageStart),
    ContentBlockStart(ContentBlockStart),
    ContentBlockDelta(ContentBlockDelta),
    ContentBlockStop(ContentBlockStop),
    MessageDelta(MessageDelta),
    MessageStop(MessageStop),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageStart {
    pub message: Message,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Usage {
    pub input_tokens: i32,
    pub output_tokens: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContentBlockStart {
    pub content_block: ContentBlock,
    pub index: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContentBlockDelta {
    pub delta: Delta,
    pub index: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Delta {
    pub text: String,
    #[serde(rename = "type")]
    pub delta_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContentBlockStop {
    pub index: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageDelta {
    pub delta: MessageDeltaData,
    pub usage: MessageDeltaUsage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageDeltaData {
    pub stop_reason: String,
    pub stop_sequence: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageDeltaUsage {
    pub output_tokens: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageStop {
    #[serde(rename = "amazon-bedrock-invocationMetrics")]
    pub invocation_metrics: InvocationMetrics,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvocationMetrics {
    #[serde(rename = "firstByteLatency")]
    pub first_byte_latency: i32,
    #[serde(rename = "inputTokenCount")]
    pub input_token_count: i32,
    #[serde(rename = "invocationLatency")]
    pub invocation_latency: i32,
    #[serde(rename = "outputTokenCount")]
    pub output_token_count: i32,
}

/// --------------------------------
///  Stream Message Deserialization
/// --------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamMessage {
    message: StreamMessageContent,
    #[serde(rename = "type")]
    message_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamMessageContent {
    content: Vec<serde_json::Value>,
    id: String,
    model: String,
    role: String,
    stop_reason: Option<serde_json::Value>,
    stop_sequence: Option<serde_json::Value>,
    #[serde(rename = "type")]
    content_type: String,
    usage: StreamUsage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamUsage {
    input_tokens: u32,
    output_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamContentBlock {
    content_block: StreamContentBlockContent,
    index: u32,
    #[serde(rename = "type")]
    block_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamContentBlockContent {
    text: String,
    #[serde(rename = "type")]
    content_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamContentBlockDelta {
    delta: StreamDelta,
    index: u32,
    #[serde(rename = "type")]
    delta_type: String,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct StreamDelta {
    text: String,
    #[serde(rename = "type")]
    delta_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamMessageDelta {
    delta: StreamMessageDeltaContent,
    #[serde(rename = "type")]
    delta_type: String,
    usage: StreamMessageDeltaUsage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamMessageDeltaContent {
    stop_reason: String,
    stop_sequence: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamMessageDeltaUsage {
    output_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamInvocationMetrics {
    #[serde(rename = "amazon-bedrock-invocationMetrics")]
    metrics: StreamInvocationMetricsContent,
    #[serde(rename = "type")]
    metrics_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamInvocationMetricsContent {
    #[serde(rename = "firstByteLatency")]
    first_byte_latency: u32,
    #[serde(rename = "inputTokenCount")]
    input_token_count: u32,
    #[serde(rename = "invocationLatency")]
    invocation_latency: u32,
    #[serde(rename = "outputTokenCount")]
    output_token_count: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_stream_documents() {
        let documents = [
            r#"{"message":{"content":[],"id":"msg_01SSVH6oAf3LoGzpz3YrdxgH","model":"claude-3-haiku-48k-20240307","role":"assistant","stop_reason":null,"stop_sequence":null,"type":"message","usage":{"input_tokens":20,"output_tokens":1}},"type":"message_start"}"#,
            r#"{"content_block":{"text":"","type":"text"},"index":0,"type":"content_block_start"}"#,
            r#"{"delta":{"text":"The","type":"text_delta"},"index":0,"type":"content_block_delta"}"#,
            r#"{"delta":{"text":" capital","type":"text_delta"},"index":0,"type":"content_block_delta"}"#,
            r#"{"delta":{"text":" of","type":"text_delta"},"index":0,"type":"content_block_delta"}"#,
            r#"{"delta":{"text":" France","type":"text_delta"},"index":0,"type":"content_block_delta"}"#,
            r#"{"delta":{"text":" is","type":"text_delta"},"index":0,"type":"content_block_delta"}"#,
            r#"{"delta":{"text":" Paris","type":"text_delta"},"index":0,"type":"content_block_delta"}"#,
            r#"{"delta":{"text":".","type":"text_delta"},"index":0,"type":"content_block_delta"}"#,
            r#"{"index":0,"type":"content_block_stop"}"#,
            r#"{"delta":{"stop_reason":"end_turn","stop_sequence":null},"type":"message_delta","usage":{"output_tokens":10}}"#,
            r#"{"amazon-bedrock-invocationMetrics":{"firstByteLatency":320,"inputTokenCount":20,"invocationLatency":394,"outputTokenCount":10},"type":"message_stop"}"#,
        ];

        for document in &documents {
            match serde_json::from_str::<serde_json::Value>(document).unwrap() {
                serde_json::Value::Object(map) => {
                    match map.get("type").and_then(|v| v.as_str()) {
                        Some("message_start") => {
                            let _message: StreamMessage = serde_json::from_str(document).unwrap();
                        }
                        Some("content_block_start") => {
                            let _content_block: StreamContentBlock =
                                serde_json::from_str(document).unwrap();
                        }
                        Some("content_block_delta") => {
                            let _content_block_delta: StreamContentBlockDelta =
                                serde_json::from_str(document).unwrap();
                        }
                        Some("content_block_stop") => {
                            // No specific struct for content_block_stop
                        }
                        Some("message_delta") => {
                            let _message_delta: StreamMessageDelta =
                                serde_json::from_str(document).unwrap();
                        }
                        Some("message_stop") => {
                            let _invocation_metrics: StreamInvocationMetrics =
                                serde_json::from_str(document).unwrap();
                        }
                        _ => panic!("Unknown document type"),
                    }
                }
                _ => panic!("Invalid JSON document"),
            }
        }
    }
}
```

## File: src/bedrock/models/claude/error.rs

- Extension: .rs
- Size: 627 bytes
- Created: 2024-04-06 23:22:06
- Modified: 2024-04-06 23:22:06

### Code
```.rs
use serde_json::Error as SerdeJsonError;
use thiserror::Error;

use crate::bedrock::error::BedrockError;


#[derive(Error, Debug)]
pub enum ClaudeError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] SerdeJsonError),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::str::Utf8Error),
    
    #[error("Bedrock error: {0}")]
    Aws(#[from] BedrockError),

    #[error("Unknown error: {0}")]
    Unknown(String),

    #[error("Deserialization error: {0}")]
    Deserialization(String),
}

```

## File: src/bedrock/models/claude/mod.rs

- Extension: .rs
- Size: 380 bytes
- Created: 2024-04-04 21:29:59
- Modified: 2024-04-04 21:29:59

### Code
```.rs
pub mod claude_request_message;
pub mod claude_client;
pub mod error;

pub use claude_client::ClaudeClient;
pub use error::ClaudeError;
pub use claude_request_message::ChatOptions;
pub use claude_request_message::Message;
pub use claude_request_message::ConversationRequest;
pub use claude_request_message::ConversationResponse;
pub use claude_request_message::StreamResult;






```

## File: src/bedrock/models/claude/claude_client.rs

- Extension: .rs
- Size: 4427 bytes
- Created: 2024-04-07 00:11:07
- Modified: 2024-04-07 00:11:07

### Code
```.rs
use crate::bedrock::bedrock_client::{BedrockClient, BedrockClientOptions};
use crate::bedrock::models::claude::claude_request_message::{
    ChatOptions, ConversationRequest, ConversationResponse, StreamResult,
};
use crate::bedrock::models::claude::error::ClaudeError;
use futures::stream::Stream;
use futures::TryStreamExt;
use serde_json::Value;

use super::claude_request_message::{
    ContentBlockDelta, ContentBlockStart, ContentBlockStop, MessageDelta, MessageStart,
    MessageStop, StreamResultData,
};

pub type ClaudeOptions = BedrockClientOptions;

pub struct ClaudeClient {
    client: BedrockClient,
}

impl ClaudeClient {
    /// Constructs a new `BedrockClient`.
    pub async fn new(options: ClaudeOptions) -> Self {
        Self {
            client: BedrockClient::new(options).await,
        }
    }

    pub async fn chat(
        &self,
        request: &ConversationRequest,
        options: &ChatOptions,
    ) -> Result<ConversationResponse, ClaudeError> {
        let model_id = options.model_id.to_string();
        let payload = serde_json::to_value(request);

        let payload = match payload {
            Ok(payload) => payload,
            Err(err) => return Err(ClaudeError::Json(err)),
        };

        let response = self.client.generate_raw(model_id, payload).await;
        match response {
            Ok(response) => {
                let conversation_response = serde_json::from_value(response).unwrap();
                Ok(conversation_response)
            }
            Err(err) => Err(ClaudeError::from(err)),
        }
    }

    pub async fn chat_with_stream(
        &self,
        request: &ConversationRequest,
        options: &ChatOptions,
    ) -> Result<impl Stream<Item = Result<StreamResultData, ClaudeError>>, ClaudeError> {
        let model_id = options.model_id.to_string();
        let payload = serde_json::to_value(request).map_err(|err| ClaudeError::Json(err))?;

        let response = self.client.generate_raw_stream(model_id, payload).await?;

        let stream = response
            .map_err(ClaudeError::from)
            .and_then(|chunk| async move {
                let stream_result = deserialize_stream_result(chunk)?;
                Ok(stream_result)
            });

        Ok(stream)
    }
}

fn deserialize_stream_result(value: Value) -> Result<StreamResultData, ClaudeError> {
    let stream_result: StreamResult = serde_json::from_value(value)
        .map_err(|err| ClaudeError::Deserialization(err.to_string()))?;

    match stream_result.result_type.as_str() {
        "message_start" => {
            let message_start: MessageStart = serde_json::from_value(stream_result.data)
                .map_err(|err| ClaudeError::Deserialization(err.to_string()))?;
            Ok(StreamResultData::MessageStart(message_start))
        }
        "content_block_start" => {
            let content_block_start: ContentBlockStart = serde_json::from_value(stream_result.data)
                .map_err(|err| ClaudeError::Deserialization(err.to_string()))?;
            Ok(StreamResultData::ContentBlockStart(content_block_start))
        }
        "content_block_delta" => {
            let content_block_delta: ContentBlockDelta = serde_json::from_value(stream_result.data)
                .map_err(|err| ClaudeError::Deserialization(err.to_string()))?;
            Ok(StreamResultData::ContentBlockDelta(content_block_delta))
        }
        "content_block_stop" => {
            let content_block_stop: ContentBlockStop =
                serde_json::from_value(stream_result.data)
                    .map_err(|err| ClaudeError::Deserialization(err.to_string()))?;
            Ok(StreamResultData::ContentBlockStop(content_block_stop))
        }
        "message_delta" => {
            let message_delta: MessageDelta = serde_json::from_value(stream_result.data)
                .map_err(|err| ClaudeError::Deserialization(err.to_string()))?;
            Ok(StreamResultData::MessageDelta(message_delta))
        }
        "message_stop" => {
            let message_stop: MessageStop = serde_json::from_value(stream_result.data)
                .map_err(|err| ClaudeError::Deserialization(err.to_string()))?;
            Ok(StreamResultData::MessageStop(message_stop))
        }
        _ => Err(ClaudeError::Deserialization(format!(
            "Unknown StreamResult type: {}",
            stream_result.result_type
        ))),
    }
}

```

## File: src/bedrock/models/mistral/mistral_client.rs

- Extension: .rs
- Size: 4134 bytes
- Created: 2024-04-07 11:17:24
- Modified: 2024-04-07 11:17:24

### Code
```.rs
use crate::bedrock::bedrock_client::{BedrockClient, BedrockClientOptions};
use crate::bedrock::models::mistral::error::MistralError;
use crate::bedrock::models::mistral::mistral_request_message::{MistralRequest, MistralResponse};
use futures::stream::Stream;
use futures::TryStreamExt;

pub type MistralOptions = BedrockClientOptions;

pub struct MistralClient {
    client: BedrockClient,
}

impl MistralClient {
    /// Constructs a new `MistralClient`.
    pub async fn new(options: MistralOptions) -> Self {
        Self {
            client: BedrockClient::new(options).await,
        }
    }

    /// Generates a response from the Mistral model.
    pub async fn generate(
        &self,
        model_id: String,
        request: &MistralRequest,
    ) -> Result<MistralResponse, MistralError> {
        let payload = serde_json::to_value(request).map_err(MistralError::Json)?;

        let response = self.client.generate_raw(model_id, payload).await?;

        let mistral_response = serde_json::from_value(response).map_err(MistralError::Json)?;
        Ok(mistral_response)
    }

    /// Generates a stream of responses from the Mistral model.
    pub async fn generate_with_stream(
        &self,
        model_id: String,
        request: &MistralRequest,
    ) -> Result<impl Stream<Item = Result<MistralResponse, MistralError>>, MistralError> {
        let payload = serde_json::to_value(request).map_err(MistralError::Json)?;

        let response = self.client.generate_raw_stream(model_id, payload).await?;


        Ok(response
            .map_ok(|value| serde_json::from_value(value).map_err(MistralError::Json))
            .map_err(|err| MistralError::Bedrock(err))
            .and_then(futures::future::ready))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::bedrock::{models::mistral::mistral_request_message::MistralRequestBuilder, ModelInfo};
    use futures::stream::StreamExt;

    #[tokio::test]
    async fn test_generate() {
        let options = MistralOptions::new().profile_name("bedrock").region("us-west-2");
        let client = MistralClient::new(options).await;

        let request = MistralRequestBuilder::new("<s>[INST] What is the capital of France ?[/INST]".to_string())
            .max_tokens(200)
            .temperature(0.8)
            .build();

        let model_name = ModelInfo::from_model_name(crate::bedrock::ModelName::MistralMixtral8X7BInstruct0x);

        let response = client.generate(model_name, &request).await;

        let response = match response {
            Ok(response) => response,
            Err(err) => panic!("Error: {:?}", err),
        };

        println!("Response: {:?}", response.outputs[0].text.to_string());

        assert!(!response.outputs.is_empty());
    }

    #[tokio::test]
    async fn test_generate_with_stream() {
        let options = MistralOptions::new().profile_name("bedrock").region("us-west-2");
        let client = MistralClient::new(options).await;

        let request = MistralRequestBuilder::new("<s>[INST] What is the capital of France ?[/INST]".to_string())
            .max_tokens(200)
            .temperature(0.8)
            .build();

       let model_name = ModelInfo::from_model_name(crate::bedrock::ModelName::MistralMixtral8X7BInstruct0x);

        // display the request as a pretty-printed JSON string
        let display_request = serde_json::to_string_pretty(&request).unwrap();
        println!("Request: {}", display_request);



        let mut stream = client
            .generate_with_stream(model_name.to_owned(), &request)
            .await
            .unwrap();

        let mut response_text = String::new();
        while let Some(result) = stream.next().await {
            match result {
                Ok(response) => {
                    println!("Response: {:?}", response.outputs[0].text.to_string());
                    response_text.push_str(&response.outputs[0].text);
                }
                Err(err) => {
                    panic!("Error: {:?}", err);
                }
            }
        }

        assert!(!response_text.is_empty());

    }


}
```

## File: src/bedrock/models/mistral/error.rs

- Extension: .rs
- Size: 620 bytes
- Created: 2024-04-05 21:08:53
- Modified: 2024-04-05 21:08:53

### Code
```.rs
use crate::bedrock::error::BedrockError;
use serde_json::Error as SerdeJsonError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MistralError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] SerdeJsonError),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::str::Utf8Error),

    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    #[error("Unknown error: {0}")]
    Unknown(String),

    #[error("Bedrock error: {0}")]
    Bedrock(#[from] BedrockError),
}
```

## File: src/bedrock/models/mistral/mod.rs

- Extension: .rs
- Size: 393 bytes
- Created: 2024-04-05 21:08:53
- Modified: 2024-04-05 21:08:53

### Code
```.rs
pub mod error;
pub mod mistral_client;
pub mod mistral_request_message;


pub use mistral_client::MistralClient;
pub use mistral_client::MistralOptions;
pub use error::MistralError;
pub use mistral_request_message::MistralRequest;
pub use mistral_request_message::MistralResponse;
pub use mistral_request_message::MistralOptionsBuilder;
pub use mistral_request_message::MistralRequestBuilder;

```

## File: src/bedrock/models/mistral/mistral_request_message.rs

- Extension: .rs
- Size: 3490 bytes
- Created: 2024-04-05 21:08:53
- Modified: 2024-04-05 21:08:53

### Code
```.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MistralRequest {
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
}



#[derive(Debug, Deserialize,Serialize)]
pub struct MistralResponse {
    pub outputs: Vec<MistralOutput>,
}

#[derive(Debug, Deserialize,Serialize)]
pub struct MistralOutput {
    pub text: String,
    pub stop_reason: Option<String>,
}

pub struct MistralRequestBuilder {
    prompt: String,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
    top_p: Option<f32>,
    top_k: Option<u32>,
    stop: Option<Vec<String>>,
}

impl MistralRequestBuilder {
    pub fn new(prompt: String) -> Self {
        Self {
            prompt,
            max_tokens: None,
            temperature: None,
            top_p: None,
            top_k: None,
            stop: None,
        }
    }

    pub fn max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    pub fn temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    pub fn top_p(mut self, top_p: f32) -> Self {
        self.top_p = Some(top_p);
        self
    }

    pub fn top_k(mut self, top_k: u32) -> Self {
        self.top_k = Some(top_k);
        self
    }

    pub fn stop(mut self, stop: Vec<String>) -> Self {
        self.stop = Some(stop);
        self
    }

    pub fn build(self) -> MistralRequest {
        MistralRequest {
            prompt: self.prompt,
            max_tokens: self.max_tokens,
            temperature: self.temperature,
            top_p: self.top_p,
            top_k: self.top_k,
            stop: self.stop,
        }
    }
}

pub struct MistralOptionsBuilder {
    max_tokens: Option<u32>,
    temperature: Option<f32>,
    top_p: Option<f32>,
    top_k: Option<u32>,
    stop: Option<Vec<String>>,
}

impl Default for MistralOptionsBuilder {
    fn default() -> Self {
        Self {
            max_tokens: Some(400),
            temperature: Some(0.7),
            top_p: Some(0.7),
            top_k: Some(50),
            stop: None,
        }
    }
}

impl MistralOptionsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    pub fn temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    pub fn top_p(mut self, top_p: f32) -> Self {
        self.top_p = Some(top_p);
        self
    }

    pub fn top_k(mut self, top_k: u32) -> Self {
        self.top_k = Some(top_k);
        self
    }

    pub fn stop(mut self, stop: Vec<String>) -> Self {
        self.stop = Some(stop);
        self
    }

    pub fn build(self) -> MistralRequest {
        MistralRequest {
            prompt: String::new(),
            max_tokens: self.max_tokens,
            temperature: self.temperature,
            top_p: self.top_p,
            top_k: self.top_k,
            stop: self.stop,
        }
    }
}
```

## File: src/examples/simple_examples/generate_text_stream_with_mistral.rs

- Extension: .rs
- Size: 1384 bytes
- Created: 2024-04-07 11:16:06
- Modified: 2024-04-07 11:16:06

### Code
```.rs
use futures::stream::StreamExt;
use crate::bedrock::models::mistral::mistral_client::{MistralClient, MistralOptions};
use crate::bedrock::models::mistral::mistral_request_message::MistralRequestBuilder;
use crate::bedrock::model_info::{ModelInfo, ModelName};

pub async fn generating_text_with_mistral() {
    let mistral_options = MistralOptions::new()
        .profile_name("bedrock")
        .region("us-west-2");

    let client = MistralClient::new(mistral_options).await;

    let request = MistralRequestBuilder::new("<s>[INST] What is the capital of France?[/INST]".to_string())
        .max_tokens(200)
        .temperature(0.8)
        .build();

    let model_id = ModelInfo::from_model_name(ModelName::MistralMixtral8X7BInstruct0x);
    let mut stream = client.generate_with_stream(model_id, &request).await.unwrap();

    while let Some(result) = stream.next().await {
        match result {
            Ok(response) => {
                println!("Response: {:?}", response.outputs[0].text);
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
            }
        }
    }
}

#[tokio::main]
pub async fn main() {
    generating_text_with_mistral().await;
}


// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generating_text_with_mistral() {
        generating_text_with_mistral().await;
    }
}
```

## File: src/examples/simple_examples/image_with_claude.rs

- Extension: .rs
- Size: 2116 bytes
- Created: 2024-04-07 09:48:46
- Modified: 2024-04-07 09:48:46

### Code
```.rs
use std::io::Write;

use futures::TryStreamExt;

use crate::bedrock::models::claude::claude_client::{ClaudeClient, ClaudeOptions};
use crate::bedrock::models::claude::claude_request_message::{ChatOptions, ContentBlockDelta, ConversationRequest, Message, StreamResultData};
use crate::fetch_and_base64_encode_image;

async fn image_with_claude() {
    let claude_options = ClaudeOptions::new()
        .profile_name("bedrock")
        .region("us-west-2");

    let client = ClaudeClient::new(claude_options).await;

    let image_url = "./data/mario.png";
    let input_text = "What's in this image?".to_string();
    let image = fetch_and_base64_encode_image(image_url).await.unwrap().to_string();
    let mime_type = "image/png".to_string();

    let message = Message::new_user_message_with_image(&input_text, &image, &mime_type);

    let mut conversation_request = ConversationRequest::default();
    conversation_request.messages.push(message);

    let chat_options = ChatOptions::default()
        .with_temperature(0.7)
        .with_max_tokens(100);

    let response_stream = client
        .chat_with_stream(&conversation_request, &chat_options)
        .await
        .unwrap();

        response_stream
        .try_for_each(|chunk| async move {
            match chunk {
                StreamResultData::ContentBlockStart(..) => {
                    println!("\n------------------------------");
                }
                StreamResultData::ContentBlockStop(..) => {
                    println!("\n------------------------------");
                }

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
}

// Main

#[tokio::main]
pub async fn main() {
    image_with_claude().await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_image_with_claude() {
        image_with_claude().await;
    }
}
```

## File: src/examples/simple_examples/mod.rs

- Extension: .rs
- Size: 170 bytes
- Created: 2024-04-07 11:15:43
- Modified: 2024-04-07 11:15:43

### Code
```.rs
pub mod generating_text_with_ollama;
pub mod generating_text_with_mistral;
pub mod generate_text_stream_with_mistral;
pub mod chat_with_claude;
pub mod image_with_claude;
```

## File: src/examples/simple_examples/generating_text_with_ollama.rs

- Extension: .rs
- Size: 1036 bytes
- Created: 2024-04-07 08:15:40
- Modified: 2024-04-07 08:15:40

### Code
```.rs
use std::io::Write;

use futures::TryStreamExt;

use crate::ollama::ollama_client::OllamaClient;
use crate::ollama::model::GenerateRequestBuilder;

// #[allow(unused)]
pub async fn generating_text_with_ollama() {
    let client = OllamaClient::new("http://localhost:11434".to_string());
    let request = GenerateRequestBuilder::new("mistral".to_string())
        .prompt("Complete this story, less than 100 words: Once upon a time".to_string())
        .build();

    let response_stream = client.generate(request).await.unwrap();

    response_stream
        .try_for_each(|chunk| async move {
            print!("{}", chunk.response);
            std::io::stdout().flush()?;
            Ok(())
        })
        .await
        .unwrap();
}

// Main
#[tokio::main]
// no warnings
// #[allow(unused)]
pub async fn main() {
    generating_text_with_ollama().await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generating_text_with_ollama() {
        generating_text_with_ollama().await;
    }
}


```

## File: src/examples/simple_examples/chat_with_claude.rs

- Extension: .rs
- Size: 1997 bytes
- Created: 2024-04-07 09:06:17
- Modified: 2024-04-07 09:06:17

### Code
```.rs
use std::io::Write;

use futures::TryStreamExt;

use crate::bedrock::model_info::{ModelInfo, ModelName};
use crate::bedrock::models::claude::claude_client::{ClaudeClient, ClaudeOptions};
use crate::bedrock::models::claude::claude_request_message::{
    ChatOptions, ContentBlockDelta, ConversationRequest, Message, StreamResultData,
};

pub async fn chat_with_claude() {
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
                StreamResultData::ContentBlockStart(..) => {
                    println!("\n------------------------------");
                }
                StreamResultData::ContentBlockStop(..) => {
                    println!("\n------------------------------");
                }

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
}

// Main
#[tokio::main]
pub async fn main() {
    chat_with_claude().await;
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_chat_with_claude() {
        chat_with_claude().await;
    }
}

```

## File: src/examples/simple_examples/generating_text_with_mistral.rs

- Extension: .rs
- Size: 1121 bytes
- Created: 2024-04-07 11:08:07
- Modified: 2024-04-07 11:08:07

### Code
```.rs
use crate::bedrock::model_info::{ModelInfo, ModelName};
use crate::bedrock::models::mistral::mistral_client::{MistralClient, MistralOptions};
use crate::bedrock::models::mistral::mistral_request_message::MistralRequestBuilder;

async fn generating_text_with_mistral() {
    let mistral_options = MistralOptions::new()
        .profile_name("bedrock")
        .region("us-west-2");

    let client = MistralClient::new(mistral_options).await;

    let request =
        MistralRequestBuilder::new("<s>[INST] What is the capital of France?[/INST]".to_string())
            .max_tokens(200)
            .temperature(0.8)
            .build();

    let model_id = ModelInfo::from_model_name(ModelName::MistralMixtral8X7BInstruct0x);
    let response = client.generate(model_id, &request).await.unwrap();

    println!("Response: {:?}", response.outputs[0].text);
}

#[tokio::main]
pub async fn main() {
    generating_text_with_mistral().await;
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generating_text_with_mistral() {
        generating_text_with_mistral().await;
    }
}

```
</hiramu>