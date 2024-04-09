
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
