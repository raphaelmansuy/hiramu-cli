**Hiramu CLI**
================

Hiramu CLI is a powerful command-line interface for interacting with language models. It provides a seamless way to ask questions and generate text using various models, including Claude (Haiku and Sonnet) and Mistral (7B, 8x7B, and Large).

**Features**
------------

* Ask questions to language models using a simple command-line interface
* Support for multiple models, including Claude and Mistral
* Customizable options for region, profile, maximum number of tokens, temperature, and model alias
* Interactive input using the `{input}` placeholder in prompts
* Real-time streaming of generated text

**Installation**
---------------

To install Hiramu CLI, ensure you have Rust installed on your system. Then, run the following command:
```bash
cargo install hiramu-cli
```
**Usage**
-----

To ask a question to a language model, use the `generate` subcommand followed by the question. You can specify additional options to customize the behavior of the CLI.
```bash
hiramu-cli generate "What is the capital of France?" -r us-west-2 -p bedrock -m 100 -t 0.7 -M haiku
```
### Options

* `-r, --region <REGION>`: The region to use (default: "us-west-2").
* `-p, --profile <PROFILE>`: The profile to use (default: "bedrock").
* `-m, --maxtoken <MAXTOKEN>`: The maximum number of tokens to generate (default: 100).
* `-t, --temperature <TEMPERATURE>`: The temperature to use for generation (default: 0.7).
* `-M, --model <MODEL>`: The model alias to use for generation (default: "haiku").

### Interactive Input

Hiramu CLI supports interactive input using the `{input}` placeholder in prompts. When the placeholder is present, the CLI will prompt you to enter the input, which will be inserted into the prompt before sending it to the language model.
```bash
hiramu-cli generate "Translate the following text from English to French: {input}" -M sonnet
```
This feature allows you to provide dynamic input to the language model during runtime.

### Model Aliases

Hiramu CLI provides convenient aliases for different language models. The following model aliases are available:

* `haiku`: Anthropic Claude 3, Haiku 1x
* `sonnet`: Anthropic Claude 3, Sonnet 1x
* `opus`: Anthropic Claude 3, Opus 1x
* `mistral7b`: Mistral 7B Instruct 0x
* `mistral8x7b`: Mistral 8x7B Instruct 0x
* `mistral-large`: Mistral Large

You can use these aliases with the `-M` or `--model` option to specify the desired model for generation.

**Examples**
------------

Here are a few examples demonstrating the usage of Hiramu CLI:

1. Ask a question using the default options:
```bash
hiramu-cli generate "What is the capital of France?"
```
2. Ask a question using a specific model and temperature:
```bash
hiramu-cli generate "What is the meaning of life?" -M sonnet -t 0.5
```
3. Translate text interactively:
```bash
hiramu-cli generate "Translate the following text from English to Spanish: {input}" -M mistral8x7b
```
4. Generate a story with a given prompt:
```bash
hiramu-cli generate "Once upon a time, in a far-off land, there lived a brave knight named {input}. The knight embarked on a quest to..." -m 200 -M mistral-large
```
Feel free to explore different prompts, models, and options to generate various types of content using Hiramu CLI.

**Contributing**
--------------

Contributions to Hiramu CLI are welcome If you encounter any issues or have suggestions for improvements, please open an issue on the GitHub repository.

**Version**
----------

### 0.1.23

* Fix issue with {input} handling

### 0.1.16

* Replace `prompt` command with `generate`
* Support for temperature and maxtoken for Ollama

**License**
---------

Hiramu CLI is open-source software licensed under the [Apache 2](LICENSE).

