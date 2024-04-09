use clap::{arg, Command};

use hiramu_cli::{generator::claude_generator::ClaudeGenerator, model::Generate};

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
                .arg_required_else_help(true),
        )
}

pub async fn generate(question: &str, region: Option<String>, profile: Option<String>) {
    let claude_generator = ClaudeGenerator::new(region, profile);
    claude_generator.generate(question).await;
}

#[tokio::main]
async fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("prompt", sub_matches)) => {
            let prompt = sub_matches.get_one::<String>("PROMPT").expect("required");
            let region = sub_matches.get_one::<String>("region").cloned();
            let profile = sub_matches.get_one::<String>("profile").cloned();
            generate(prompt, region, profile).await;
        }
        _ => unreachable!(),
    }
    // Continued program logic goes here...
}
