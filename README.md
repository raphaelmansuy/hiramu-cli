# Hiramu CLI

Hiramu CLI is a command-line interface tool that allows you to interact with large language models (LLMs) and generate text based on prompts. It provides a convenient way to ask questions and receive generated responses from various LLMs.

## Features

- Support for multiple LLMs, including Anthropic's Claude models (Haiku and Sonnet) (AWS Bedrock)
- Customizable generation parameters, such as maximum token count and temperature
- Streaming output for real-time response display
- Easy-to-use command-line interface with intuitive options

## Installation

To install Hiramu CLI, ensure you have Rust installed on your system. Then, follow these steps:

1. Clone the repository:

```bash
git clone https://github.com/yourusername/hiramu-cli.git
```

2. Change to the project directory:

```bash
cd hiramu-cli
```

3. Build the project:

```bash
cargo build --release
```

4. The compiled binary will be available in the `target/release` directory. You can add it to your system's PATH for easy access.

## Usage

To use Hiramu CLI, run the following command:

```bash
hiramu-cli prompt [OPTIONS] <PROMPT>
```

Replace `<PROMPT>` with the question or prompt you want to ask the LLM.

### Options

- `-r, --region <REGION>`: Specify the region to use (default: "us-west-2").
- `-p, --profile <PROFILE>`: Specify the profile to use (default: "bedrock").
- `-m, --maxtoken <MAXTOKEN>`: Set the maximum number of tokens to generate (default: 100).
- `-t, --temperature <TEMPERATURE>`: Set the temperature for generation (default: 0.7).
- `-M, --model <MODEL>`: Specify the model alias to use for generation (default: "haiku").

### Examples

1. Ask a question using the default settings:

```bash
hiramu-cli prompt "What is the capital of France?"
```

2. Ask a question using a specific model and custom parameters:

```bash
hiramu-cli prompt -M sonnet -m 200 -t 0.8 "Explain the concept of quantum entanglement."
```

## Configuration

Hiramu CLI uses the `hiramu` library for interacting with LLMs. Make sure you have the necessary credentials and configurations set up for the LLMs you want to use.

## Contributing

Contributions to Hiramu CLI are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request on the GitHub repository.

## License

Hiramu CLI is released under the [Apache 2](LICENSE).

## Acknowledgements

Hiramu CLI is built using the following libraries and frameworks:

- [hiramu](https://github.com/yourusername/hiramu): A library for interacting with LLMs.
- [clap](https://github.com/clap-rs/clap): A command-line argument parser for Rust.
- [tokio](https://github.com/tokio-rs/tokio): An asynchronous runtime for Rust.

We would like to thank the developers and contributors of these projects for their excellent work.
