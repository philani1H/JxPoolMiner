use thiserror::Error;

#[derive(Error, Debug)]
pub enum JxError {
    #[error("Device error: {0}")]
    Device(String),
    
    #[error("Mining error: {0}")]
    Mining(String),
    
    #[error("Pool error: {0}")]
    Pool(String),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Other error: {0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, JxError>;
