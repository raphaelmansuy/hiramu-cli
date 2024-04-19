use thiserror::Error;
use std::io;

#[derive(Debug, Error)]
pub enum GenerationError {
    #[error("Failed to read from stdin")]
    StdinReadError(#[from] io::Error),
    #[error("Model not found")]
    ModelNotFoundError,
    #[error("Failed to parse Cargo.toml")]
    CargoTomlParseError(#[from] toml::de::Error),
}
