
#[derive(Debug, Clone, clap::ValueEnum)]
pub enum ModelAlias {
    Haiku,
    Sonnet,
    Opus,
    Mistral8x7b,
    Mistral7b,
    MistralLarge
}


// Return a list of model aliases
pub fn model_aliases() -> Vec<&'static str> {
    vec!["haiku", "sonnet","opus", "mistral8x7b", "mistral7b", "mitral-large"]
}