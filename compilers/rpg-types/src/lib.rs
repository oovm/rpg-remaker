#![deny(missing_debug_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]

/// 错误处理模块
pub mod errors;
/// Marshal 编解码模块
pub mod marshal;
/// Ruby 值模块
pub mod ruby_value;

pub use crate::{
    errors::{Result, RpgError},
    marshal::{from_bytes, to_bytes, DecodeError, EncodeError},
    ruby_value::{RpgFields, RpgHash, RpgHashKey, RpgValue},
};
