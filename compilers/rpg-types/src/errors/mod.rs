use std::io;
use std::fmt;

/// 统一的结果类型
pub type Result<T> = std::result::Result<T, RpgError>;

/// RPG 错误类型
#[derive(Debug)]
pub enum RpgError {
    /// IO 错误
    Io(io::Error),
    /// 编码错误
    Encode {
        /// 错误消息
        message: String,
        /// 数据格式
        format: &'static str,
    },
    /// 解码错误
    Decode {
        /// 错误消息
        message: String,
        /// 数据格式
        format: &'static str,
    },
}

impl fmt::Display for RpgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RpgError::Io(e) => write!(f, "IO error: {}", e),
            RpgError::Encode { message, format } => write!(f, "Encode error: {} (format: {})", message, format),
            RpgError::Decode { message, format } => write!(f, "Decode error: {} (format: {})", message, format),
        }
    }
}

impl std::error::Error for RpgError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            RpgError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for RpgError {
    fn from(e: io::Error) -> Self {
        RpgError::Io(e)
    }
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
