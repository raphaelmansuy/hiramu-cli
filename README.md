# Hiramu CLI

Hiramu CLI is a powerful command-line interface for interacting with language models. It provides a seamless way to ask questions and generate text using various models from different providers, including Anthropic's Claude (Haiku, Sonnet, Opus), Mistral (7B, 8x7B, Large), and Ollama.

🤗 If you find it compelling, I would be grateful if you could show your support by giving it a Star 🌟.


## Table of Contents
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
  - [Options](#options)
  - [Interactive Input](#interactive-input)
  - [Model Aliases](#model-aliases)
  - [Provider Aliases](#provider-aliases)
- [Installation](#installation)
- [Examples](#examples)
- [Contributing](#contributing)
- [Versioning](#versioning)
- [License](#license)

## Features

- Ask questions to language models using a simple command-line interface
- Support for multiple models from Anthropic, Mistral, and Ollama
- Customizable options for region, profile, max tokens, temperature, and model alias
- Interactive input using the `{input}` placeholder in prompts
- Real-time streaming of generated text
- Integration with other command-line tools and databases (e.g., `git diff`, DuckDB, PostgreSQL)

## Installation

To install Hiramu CLI, ensure you have Rust installed on your system. If you don't have Rust installed, you can follow the official installation guide: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

Once Rust is installed, run the following command to install Hiramu CLI:

```bash
cargo install hiramu-cli
```

## Usage

To ask a question to a language model, use the `generate` command followed by the question. You can specify additional options to customize the behavior of the CLI.

```bash
hiramu-cli generate "What is the capital of France?" -r us-west-2 -p bedrock -m 100 -t 0.7 -M haiku -P bedrock
```

### Options

- `-r, --region <REGION>`: The region to use (default: "us-west-2").
- `-p, --profile <PROFILE>`: The profile to use (default: "bedrock").
- `-m, --maxtoken <MAXTOKEN>`: The maximum number of tokens to generate (default: 100).
- `-t, --temperature <TEMPERATURE>`: The temperature for generation (default: 0.7).
- `-M, --model <MODEL>`: The model alias to use (default: "haiku").
- `-P, --provider <PROVIDER>`: The provider alias to use for generation (default: "bedrock").
- `-E, --endpoint <ENDPOINT>`: The provider endpoint to use for generation (default: "http://localhost:11434").

### Interactive Input

Hiramu CLI supports interactive input using the `{input}` placeholder in prompts. When the placeholder is present, the CLI will prompt you to enter the input, which will be inserted into the prompt before sending it to the language model.

```bash
hiramu-cli generate "Translate the following text from English to French: {input}" -M sonnet
```

This feature allows you to provide dynamic input to the language model during runtime.

### Model Aliases

Hiramu CLI provides convenient aliases for different language models. The following model aliases are available:

- `haiku`: Anthropic Claude 3, Haiku 1x
- `sonnet`: Anthropic Claude 3, Sonnet 1x
- `opus`: Anthropic Claude 3, Opus 1x
- `mistral7b`: Mistral 7B Instruct 0x
- `mistral8x7b`: Mistral 8x7B Instruct 0x
- `mistral-large`: Mistral Large

You can use these aliases with the `-M` or `--model` option to specify the desired model for generation.

### Provider Aliases

Hiramu CLI supports different providers for language model generation. The following provider aliases are available:

- `bedrock`: Anthropic's Bedrock platform (default)
- `ollama`: Ollama provider

You can use these aliases with the `-P` or `--provider` option to specify the desired provider for generation. When using the `ollama` provider, you also need to specify the endpoint using the `-E` or `--endpoint` option.

## Examples

Here are a few examples demonstrating the usage of Hiramu CLI:

1. Ask a question using the default options:
   ```bash
   hiramu-cli generate "What is the capital of France?"
   ```

2. Specify model and temperature:
   ```bash
   hiramu-cli generate "What is the meaning of life?" -M sonnet -t 0.5
   ```

3. Translate interactively:
   ```bash
   hiramu-cli generate "Translate from English to Spanish: {input}" -M mistral8x7b
   ```

4. Generate release notes by combining with `git diff`:
   ```bash
   git diff HEAD~1..HEAD | hiramu-cli generate "Summarize the changes:" -M opus
   ```
   This pipes the output of `git diff` into Hiramu CLI to generate a summary of the code changes.

5. Generate SQL queries from natural language using DuckDB:
   ```bash
   hiramu-cli generate "SQL query to find users who signed up in the last 30 days: {input}" -M mistral7b | duckdb -c -
   ```
   The generated SQL query is piped directly into DuckDB for execution.

6. Optimize SQL queries using PostgreSQL:
   ```bash
   query="SELECT * FROM orders JOIN customers ON orders.customer_id = customers.id"
   optimized_query=$(echo "$query" | hiramu-cli generate "Optimize this SQL query: {input}" -M mistral-large)
   psql -d mydb -c "$optimized_query"
   ```
   The existing SQL query is passed to Hiramu CLI to generate an optimized version, which is then executed using PostgreSQL.

Feel free to explore different prompts, models, providers, and options to generate various types of content using Hiramu CLI.


## Installation (MacOs)

## Adding the Homebrew Tap

To utilize the `raphaelmansuy/homebrew-tap`, you'll first need to integrate it with your Homebrew setup on your Mac. Follow these steps to get started:

### Step 1: Open Terminal
Launch the Terminal application on your Mac. You can find it in the `Applications/Utilities` folder, or you can search for it using Spotlight.

### Step 2: Add the Tap
In the Terminal, execute the following command to add the `raphaelmansuy/homebrew-tap`:

```bash
brew tap raphaelmansuy/homebrew-tap
```

This command registers the `raphaelmansuy/homebrew-tap` with your Homebrew installation, enabling you to access the packages (formulas) available in this repository.

## Installing `hiramu-cli` Using Homebrew

With the tap now added, you can proceed to install the `hiramu-cli` package directly through Homebrew.

### Step 1: Install the Package
In the same Terminal window, run the following command to install `hiramu-cli`:

```bash
brew install hiramu-cli
```

This command fetches and installs the `hiramu-cli` package from the `raphaelmansuy/homebrew-tap`, setting it up on your system.

### Step 2: Verify Installation
After the installation process completes, you can verify that `hiramu-cli` is correctly installed by typing:

```bash
hiramu-cli --version
```

This command should display the version number of `hiramu-cli`, confirming that it is ready to use.

## Summary of Steps

Here’s a quick recap of what you need to do to get `hiramu-cli` up and running on your Mac:

1. **Add the Tap**:
   ```bash
   brew tap raphaelmansuy/homebrew-tap
   ```
2. **Install `hiramu-cli`**:
   ```bash
   brew install hiramu-cli
   ```

After completing these steps, `hiramu-cli` will be available in your Terminal, and you can begin using it for your tasks.

## Installation (Cargo)

To install the Hiramu CLI using Cargo, you first need to ensure that Rust is installed on your system. 

If Rust is not already installed, you can download and install it by following the official guide available at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install). 

Once Rust is installed, open your terminal and execute the command `cargo install hiramu-cli`. This command will download and install the Hiramu CLI from the available package, setting it up on your system for immediate use.


## Contributing

Contributions to Hiramu CLI are welcome! If you encounter any issues or have suggestions for improvements, please open an issue on the GitHub repository.

Before submitting a pull request, ensure that the tests pass and the code is formatted with `cargo fmt`. You can run the tests using the following command:

```bash
hiramu-cli generate "Once upon a time, in a far-off land, there lived a brave knight named {input}. The knight embarked on a quest to..." -m 200 -M mistral-large
```
Feel free to explore different prompts, models, and options to generate various types of content using Hiramu CLI.

**Contributing**
--------------

Contributions to Hiramu CLI are welcome If you encounter any issues or have suggestions for improvements, please open an issue on the GitHub repository.

If you find it compelling, I would be grateful if you could show your support by giving it a Star 🌟.

**Version**
----------

### 0.1.23

* Fix issue with {input} handling

### 0.1.16

* Replace `prompt` command with `generate`
* Support for temperature and maxtoken for Ollama

**License**
---------

Hiramu CLI is open-source software licensed under the Apache 2 License. See the [LICENSE](LICENSE) file for more details.
