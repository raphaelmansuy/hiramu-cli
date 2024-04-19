
#[derive(Debug, Clone, clap::ValueEnum)]
pub enum Provider {
    Bedrock,
    Ollama,
}
