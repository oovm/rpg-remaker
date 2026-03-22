#![warn(missing_docs)]
#![doc = include_str!("readme.md")]

/// Marshal 编解码模块
mod marshal;
/// rvdata2 文件处理模块
pub mod rvdata2;
/// rxdata 文件处理模块
pub mod rxdata;
/// 加密资源文件处理模块
pub mod asset;
/// RPG Maker 游戏数据类型模块
pub mod rpg_maker;

pub use marshal::{DecodeError, EncodeError, from_bytes, to_bytes};
pub use rvdata2::{read_rvdata2, write_rvdata2};
pub use rxdata::{read_rxdata, write_rxdata};
pub use asset::{
    read_asset, decrypt_asset_data,
    Decrypter, AssetType, EncryptedExtension
};
