
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, clap::ValueEnum, Serialize, Deserialize)]
pub enum Provider {
    Bedrock,
    Ollama,
}
