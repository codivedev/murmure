use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum AppError {
    #[error("Audio error: {0}")]
    AudioError(String),
    
    #[error("Groq API error: {0}")]
    GroqApiError(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Input error: {0}")]
    InputError(String),
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("IO error: {0}")]
    IoError(String),
}

pub type Result<T> = std::result::Result<T, AppError>;