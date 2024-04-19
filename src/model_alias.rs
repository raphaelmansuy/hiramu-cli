
#[derive(Debug, Clone, clap::ValueEnum)]
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


// Return a list of model aliases
pub fn model_aliases() -> Vec<&'static str> {
    vec!["haiku", "sonnet","opus", "mistral8x7b", "mistral7b", "mitral-large"]
}