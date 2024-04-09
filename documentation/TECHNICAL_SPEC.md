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
