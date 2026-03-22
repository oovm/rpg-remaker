use std::io;
use thiserror::Error;

/// 统一的结果类型
pub type Result<T> = std::result::Result<T, RpgError>;

/// RPG 错误类型
#[derive(Debug, Error)]
pub enum RpgError {
    /// IO 错误
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    /// 编码错误
    #[error("Encode error: {message} (format: {format})")]
    Encode {
        /// 错误消息
        message: String,
        /// 数据格式
        format: &'static str,
    },
    /// 解码错误
    #[error("Decode error: {message} (format: {format})")]
    Decode {
        /// 错误消息
        message: String,
        /// 数据格式
        format: &'static str,
    },
}

impl RpgError {
    /// 创建一个新的解码错误
    pub fn decode(format: &'static str, message: &str) -> Self {
        Self::Decode { message: message.to_string(), format }
    }

    /// 创建一个新的编码错误
    pub fn encode(format: &'static str, message: &str) -> Self {
        Self::Encode { message: message.to_string(), format }
    }
}
