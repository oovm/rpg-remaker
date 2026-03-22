use std::io;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, RpgError>;

#[derive(Debug, Error)]
pub enum RpgError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Encode error: {message} (format: {format})")]
    Encode { message: String, format: &'static str },
    #[error("Decode error: {message} (format: {format})")]
    Decode { message: String, format: &'static str },
}

impl RpgError {
    /// 创建一个新的解码错误
    pub fn decode(format: &'static str, message: &str) -> Self {
        Self::Decode { 
            message: message.to_string(), 
            format 
        }
    }
    
    /// 创建一个新的编码错误
    pub fn ncode(format: &'static str, message: &str) -> Self {
        Self::Encode { 
            message: message.to_string(), 
            format 
        }
    }
}
