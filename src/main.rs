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
                .arg_required_else_help(true),
        )
}

pub async fn generate(question: &str) {
    let claude_generator = ClaudeGenerator::new();
    claude_generator.generate(question).await;
}

#[tokio::main]
async fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("prompt", sub_matches)) => {
            let prompt = sub_matches.get_one::<String>("PROMPT").expect("required");
            generate(prompt).await;
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable!()
    }

    // Continued program logic goes here...
}
