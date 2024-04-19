use std::io::{self, Read};
use clap::{arg, Command};
use tokio::main;


use hiramu::bedrock::ModelName;
use hiramu_cli::{
    generator::ollama_provider::OllamaProvider,
    generator::{claude_generator::ClaudeGenerator, mistral_generator::MistralGenerator},
    model::Generate,
    model_alias::{model_alias_from_str, ModelAlias},
    provider::Provider,
    error::GenerationError,
};

const CARGO_TOML: &str = include_str!("../Cargo.toml");

fn cli() -> Command {
    Command::new("hiramu-cli")
        .about("Ask a question to a language model. You can specify the region, profile, maximum number of tokens, temperature, and model alias.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("version")
                .about("Displays the version of the application")
        )
        .subcommand(
            Command::new("prompt")
                .about("Ask a question to a LLM")
                .arg(arg!(<PROMPT> "The prompt to ask. Can contain {input} to read from stdin."))
                .arg(arg!(-r --region <REGION> "The region to use").default_value("us-west-2"))
                .arg(arg!(-p --profile <PROFILE> "The profile to use").default_value("bedrock"))
                .arg(arg!(-m --maxtoken <MAXTOKEN> "The maximum number of tokens to generate").default_value("100").value_parser(clap::value_parser!(u32)))
                .arg(arg!(-t --temperature <TEMPERATURE> "The temperature to use for generation").default_value("0.7").value_parser(clap::value_parser!(f32)))
                .arg(arg!(-M --model <MODEL> "The model alias to use for generation").default_value("haiku"))
                .arg(arg!(-P --provider <PROVIDER> "The provider alias to use for generation ollama or bedrock").default_value("bedrock").value_parser(clap::value_parser!(Provider)))
                .arg(arg!(-E --endpoint <ENDPOINT> "The provider endpoint to use for generation").default_value("http://localhost:11434"))
        )
}

async fn generate(
    question: &str,
    region: Option<String>,
    profile: Option<String>,
    max_token: Option<u32>,
    temperature: Option<f32>,
    model: String,
    provider: Provider,
    endpoint: Option<String>,
) -> Result<(), GenerationError> {
    let mut prompt = question.to_string();

    if prompt.contains("{input}") {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input)?;
        prompt = prompt.replace("{input}", input.trim());
    }

    match provider {
        Provider::Bedrock => {
            let model = model_alias_from_str(&model).ok_or(GenerationError::ModelNotFoundError)?;
            bedrock_generate(model, region, profile, max_token, temperature, prompt).await?;
        }
        Provider::Ollama => {
            let endpoint = endpoint.unwrap_or_else(|| "http://localhost:11434".to_string());
            let ollama_provider = OllamaProvider::new(endpoint, model);
            ollama_provider.generate(&prompt).await;
        }
    }

    Ok(())
}

async fn bedrock_generate(
    model: ModelAlias,
    region: Option<String>,
    profile: Option<String>,
    max_token: Option<u32>,
    temperature: Option<f32>,
    prompt: String,
) -> Result<(), GenerationError> {
    let model_name = match model {
        ModelAlias::Haiku => ModelName::AnthropicClaudeHaiku1x,
        ModelAlias::Sonnet => ModelName::AnthropicClaudeSonnet1x,
        ModelAlias::Opus => ModelName::AnthropicClaudeOpus1x,
        ModelAlias::Mistral7b => ModelName::MistralMistral7BInstruct0x,
        ModelAlias::Mistral8x7b => ModelName::MistralMixtral8X7BInstruct0x,
        ModelAlias::MistralLarge => ModelName::MistralLarge,
    };

    match model {
        ModelAlias::Haiku | ModelAlias::Sonnet | ModelAlias::Opus => {
            let generator = ClaudeGenerator::new(region, profile, max_token, temperature, Some(model_name));
            generator.generate(&prompt).await;

        }
        ModelAlias::Mistral7b | ModelAlias::Mistral8x7b | ModelAlias::MistralLarge => {
            let generator = MistralGenerator::new(region, profile, max_token, temperature, Some(model_name));
            generator.generate(&prompt).await;
        }
    };

    Ok(())
}

#[main]
async fn main() -> Result<(), GenerationError> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("prompt", sub_matches)) => {
            let prompt = sub_matches.get_one::<String>("PROMPT").expect("required");
            let region = sub_matches.get_one::<String>("region").cloned();
            let profile = sub_matches.get_one::<String>("profile").cloned();
            let max_token = sub_matches.get_one::<u32>("maxtoken").cloned();
            let temperature = sub_matches.get_one::<f32>("temperature").cloned();
            let model = sub_matches.get_one::<String>("model").cloned().unwrap();
            let endpoint = sub_matches.get_one::<String>("endpoint").cloned();
            let provider = sub_matches.get_one::<Provider>("provider").cloned().unwrap();

            generate(
                prompt,
                region,
                profile,
                max_token,
                temperature,
                model,
                provider,
                endpoint,
            )
            .await?;
        }
        Some(("version", _)) => {
            let cargo_toml: toml::Value = toml::from_str(CARGO_TOML)?;
            let version = cargo_toml
                .get("package")
                .and_then(|package| package.get("version"))
                .and_then(|version| version.as_str())
                .unwrap_or("Unknown");

            println!("hiramu-cli version {}", version);
        }
        _ => {}
    }

    Ok(())
}