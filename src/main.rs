use clap::{arg, Command};

use hiramu::bedrock::ModelName;
use hiramu_cli::{generator::claude_generator::ClaudeGenerator, model::Generate, model_alias::ModelAlias};



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
                .arg(
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
                        .value_parser(clap::value_parser!(ModelAlias)),
                )
                .arg_required_else_help(true),
        )
}

pub async fn generate(
    question: &str,
    region: Option<String>,
    profile: Option<String>,
    max_token: Option<u32>,
    temperature: Option<f32>,
    model: Option<ModelAlias>,
) {
    // get model_name form model 


    match model {
        Some(ModelAlias::Haiku) => {
            let model_name = ModelName::AnthropicClaudeHaiku1x;
            let claude_generator = ClaudeGenerator::new(region, profile, max_token, temperature, Some(model_name));
            claude_generator.generate(question).await;
        }
        Some(ModelAlias::Sonnet) => {
            let model_name = ModelName::AnthropicClaudeSonnet1x;
            let claude_generator = ClaudeGenerator::new(region, profile, max_token, temperature, Some(model_name));
            claude_generator.generate(question).await;
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
            let model = sub_matches.get_one::<ModelAlias>("model").cloned();
            generate(prompt, region, profile, max_token, temperature,model).await;
        }
        _ => unreachable!(),
    }
}
