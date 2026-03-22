/// Marshal 解码器
mod decode;
/// Marshal 编码器
mod encode;
/// Marshal 标签定义
mod tag;

pub use decode::{DecodeError, from_bytes};
pub use encode::{EncodeError, to_bytes};
