/// Marshal 标签定义
mod tag;
/// Marshal 解码器
mod decode;
/// Marshal 编码器
mod encode;

pub use decode::{from_bytes, DecodeError, DecodeErrorKind};
pub use encode::{to_bytes, EncodeError};
pub use tag::Tag;
