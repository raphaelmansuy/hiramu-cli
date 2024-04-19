use tokio::io::AsyncWriteExt;

pub mod claude_generator;
pub mod mistral_generator;
pub mod ollama_provider;


pub enum Generator {
    Claude(claude_generator::ClaudeGenerator),
    Mistral(mistral_generator::MistralGenerator),
    Ollama(ollama_provider::OllamaProvider),
}

impl crate::model::Generate for Generator {
    async fn generate(&self, question: &str) {
        match self {
            Generator::Claude(generator) => generator.generate(question).await,
            Generator::Mistral(generator) => generator.generate(question).await,
            Generator::Ollama(provider) => provider.generate(question).await,
        }
    }
}


