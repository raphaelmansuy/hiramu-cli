use hiramu::bedrock::ModelName;

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum ModelAlias {
    Haiku,
    Sonnet,
}

impl From<ModelAlias> for ModelName {
    fn from(alias: ModelAlias) -> Self {
        match alias {
            ModelAlias::Haiku => ModelName::AnthropicClaudeHaiku1x,
            ModelAlias::Sonnet => ModelName::AnthropicClaudeSonnet1x,
        }
    }
}