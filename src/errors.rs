//! Error types for markov-rs

use thiserror::Error;

/// Errors that can occur when working with Markov chains
#[derive(Error, Debug)]
pub enum MarkovError {
    /// Errors related to invalid parameters passed to functions
    #[error("Invalid parameter: {0}")]
    ParamError(String),

    /// Errors occurring during JSON serialization or deserialization
    #[error("Invalid JSON: {0}")]
    JsonError(#[from] serde_json::Error),

    /// Errors related to invalid model format during loading
    #[error("Invalid model format: {0}")]
    ModelFormatError(String),

    /// Errors occurring when combining models with different state sizes
    #[error("State size mismatch: {0}")]
    StateSizeError(String),

    /// Errors occurring during model combination
    #[error("Model combination error: {0}")]
    CombineError(String),

    /// Errors related to failed sentence generation
    #[error("Sentence generation failed: {0}")]
    GenerationError(String),
}

pub type Result<T> = std::result::Result<T, MarkovError>;
