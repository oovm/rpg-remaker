#![deny(missing_debug_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]

/// 错误处理模块
pub mod errors;
/// Ruby 值模块
pub mod ruby_value;

pub use crate::errors::{Result, RpgError};
pub use crate::ruby_value::RpgValue;
