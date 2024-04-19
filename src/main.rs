use std::io::Read;

use clap::{arg, Command};

use hiramu::bedrock::ModelName;
use hiramu_cli::{
    generator::ollama_provider::OllamaProvider,
    generator::{claude_generator::ClaudeGenerator, mistral_generator::MistralGenerator},
    model::Generate,
    model_alias::{model_alias_from_str, ModelAlias},
    provider::Provider,
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
                .arg(arg!(<PROMPT> "The prompt to ask. Can contain {input} to read from stdin. For example, 'What is the capital of {input}?' will read a country name from stdin and ask the model for its capital."))                .arg(
                    arg!(-r --region <REGION> "The region to use")
                        .required(false)
                        .default_value("us-west-2"),
                )
                .arg(
                    arg!(-p --profile <PROFILE> "The profile to use")
                        .required(false)
                        .default_value("bedrock"),
                )
                .arg(
                    arg!(-m --maxtoken <MAXTOKEN> "The maximum number of tokens to generate")
                        .required(false)
                        .default_value("100")
                        .value_parser(clap::value_parser!(u32)),
                )
                .arg(
                    arg!(-t --temperature <TEMPERATURE> "The temperature to use for generation")
                        .required(false)
                        .default_value("0.7")
                        .value_parser(clap::value_parser!(f32)),
                )
                .arg(
                    arg!(-M --model <MODEL> "The model alias to use for generation")
                        .required(false)
                        .default_value("haiku")
                )
                .arg(
                    arg!(-P --provider <PROVIDER> "The provider alias to use for generation ollama or bedrock")
                        .required(false)
                        .default_value("bedrock")
                        .value_parser(clap::value_parser!(Provider)),
                )
                .arg(
                    arg!(-E --endpoint <ENDPOINT> "The provider endpoint to use for generation")
                        .required(false)
                        .default_value("http://localhost:11434")),                
        )
}

pub async fn generate(
    question: &str,
    region: Option<String>,
    profile: Option<String>,
    max_token: Option<u32>,
    temperature: Option<f32>,
    model: String,
    provider: Provider,
    endpoint: Option<String>,
) {
    let mut prompt = question.to_string();

    if prompt.contains("{input}") {
        let mut input = String::new();
        std::io::stdin().read_to_string(&mut input).unwrap();
        prompt = prompt.replace("{input}", input.trim());
    }

    match provider {
        Provider::Bedrock => {
            let model = model_alias_from_str(&model);
            bedrock_generate(model, region, profile, max_token, temperature, prompt).await;
        }
        Provider::Ollama => {
            let endpoint = endpoint.unwrap_or("http://localhost:11434".to_string());
            let ollama_provider = OllamaProvider::new(endpoint, model);
            ollama_provider.generate(&prompt).await;
        }
    }
}

async fn bedrock_generate(
    model: Option<ModelAlias>,
    region: Option<String>,
    profile: Option<String>,
    max_token: Option<u32>,
    temperature: Option<f32>,
    prompt: String,
) {
    // get model_name form model
    match model {
        Some(ModelAlias::Haiku) => {
            let model_name = ModelName::AnthropicClaudeHaiku1x;
            let claude_generator =
                ClaudeGenerator::new(region, profile, max_token, temperature, Some(model_name));
            claude_generator.generate(&prompt).await;
        }
        Some(ModelAlias::Sonnet) => {
            let model_name = ModelName::AnthropicClaudeSonnet1x;
            let claude_generator =
                ClaudeGenerator::new(region, profile, max_token, temperature, Some(model_name));
            claude_generator.generate(&prompt).await;
        }
        Some(ModelAlias::Opus) => {
            let model_name = ModelName::AnthropicClaudeOpus1x;
            let claude_generator =
                ClaudeGenerator::new(region, profile, max_token, temperature, Some(model_name));
            claude_generator.generate(&prompt).await;
        }
        Some(ModelAlias::Mistral7b) => {
            let model_name = ModelName::MistralMistral7BInstruct0x;
            let mistral_generator =
                MistralGenerator::new(region, profile, max_token, temperature, Some(model_name));
            mistral_generator.generate(&prompt).await;
        }
        Some(ModelAlias::Mistral8x7b) => {
            let model_name = ModelName::MistralMixtral8X7BInstruct0x;
            let mistral_generator =
                MistralGenerator::new(region, profile, max_token, temperature, Some(model_name));
            mistral_generator.generate(&prompt).await;
        }
        Some(ModelAlias::MistralLarge) => {
            let model_name = ModelName::MistralLarge;
            let mistral_generator =
                MistralGenerator::new(region, profile, max_token, temperature, Some(model_name));
            mistral_generator.generate(&prompt).await;
        }
        _ => {
            println!("Model not found");
        }
    }
}

#[tokio::main]
async fn main() {
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
            let provider = sub_matches
                .get_one::<Provider>("provider")
                .cloned()
                .unwrap();
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
            .await;
        }
        Some(("version", _)) => {
            let cargo_toml: toml::Value =
                toml::from_str(CARGO_TOML).expect("Failed to parse Cargo.toml");

            let version = cargo_toml
                .get("package")
                .and_then(|package| package.get("version"))
                .and_then(|version| version.as_str())
                .unwrap_or("Unknown");

            println!("hiramu-cli version {}", version);
        }
        // Help and version commands are handled automatically by clap
        _ => {}
    }
}
