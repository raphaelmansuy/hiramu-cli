use hiramu::bedrock::ModelName;
use serde::{Deserialize, Serialize};

use crate::error::GenerationError;

#[derive(Debug, Clone, clap::ValueEnum,Serialize, Deserialize)]
pub enum ModelAlias {
    Haiku,
    Sonnet,
    Opus,
    Mistral8x7b,
    Mistral7b,
    MistralLarge
}

pub fn model_alias_from_str(s: &str) -> Option<ModelAlias> {
    match s {
        "haiku" => Some(ModelAlias::Haiku),
        "sonnet" => Some(ModelAlias::Sonnet),
        "opus" => Some(ModelAlias::Opus),
        "mistral8x7b" => Some(ModelAlias::Mistral8x7b),
        "mistral7b" => Some(ModelAlias::Mistral7b),
        "mistral-large" => Some(ModelAlias::MistralLarge),
        _ => None,
    }
}


pub fn get_model_name_from_alias(model_alias: ModelAlias) -> Result<ModelName, GenerationError> {
    match model_alias {
        ModelAlias::Haiku => Ok(ModelName::AnthropicClaudeHaiku1x),
        ModelAlias::Sonnet => Ok(ModelName::AnthropicClaudeSonnet1x),
        ModelAlias::Opus => Ok(ModelName::AnthropicClaudeOpus1x),
        ModelAlias::Mistral7b => Ok(ModelName::MistralMistral7BInstruct0x),
        ModelAlias::Mistral8x7b => Ok(ModelName::MistralMixtral8X7BInstruct0x),
        ModelAlias::MistralLarge => Ok(ModelName::MistralLarge),
    }
}


// Return a list of model aliases
pub fn model_aliases() -> Vec<&'static str> {
    vec!["haiku", "sonnet","opus", "mistral8x7b", "mistral7b", "mitral-large"]
}