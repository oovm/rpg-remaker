#![warn(missing_docs)]
#![doc = include_str!("../readme.md")]

/// Marshal 编解码模块
mod marshal;
/// rvdata2 文件处理模块
pub mod rvdata2;
/// rxdata 文件处理模块
pub mod rxdata;

pub use marshal::{DecodeError, EncodeError, from_bytes, to_bytes};
pub use rvdata2::{read_rvdata2, write_rvdata2};
pub use rxdata::{read_rxdata, write_rxdata};
