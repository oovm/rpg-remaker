#![warn(missing_docs)]

/// 统一的 RPG 错误类型
#[derive(Debug, thiserror::Error)]
pub enum RpgError {
    /// IO 错误
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    /// 解码错误
    #[error("Decode error: {0}")]
    Decode(#[from] alox_48::DeError),
    /// 编码错误
    #[error("Encode error: {0}")]
    Encode(#[from] alox_48::SerError),
    /// YAML 错误
    #[error("YAML error: {0}")]
    Yaml(#[from] serde_yaml::Error),
    /// 其他错误
    #[error("Other error: {0}")]
    Other(String),
}

/// 统一的结果类型
pub type Result<T> = std::result::Result<T, RpgError>;

pub mod rvdata2;
