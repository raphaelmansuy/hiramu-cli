use hiramu::bedrock::ModelName;

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum ModelAlias {
    Haiku,
    Sonnet,
    Mistral8x7b,
    Mistral7b,
    MistralLarge
}

impl From<ModelAlias> for ModelName {
    fn from(alias: ModelAlias) -> Self {
        match alias {
            ModelAlias::Haiku => ModelName::AnthropicClaudeHaiku1x,
            ModelAlias::Sonnet => ModelName::AnthropicClaudeSonnet1x,
            ModelAlias::Mistral8x7b => ModelName::MistralMixtral8X7BInstruct0x,
            ModelAlias::Mistral7b => ModelName::MistralMistral7BInstruct0x,
            ModelAlias::MistralLarge => ModelName::MistralLarge,
        }
    }
}

// Return a list of model aliases
pub fn model_aliases() -> Vec<&'static str> {
    vec!["haiku", "sonnet", "mistral8x7b", "mistral7b", "mitral-large"]
}