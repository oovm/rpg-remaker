//! RPG Maker 脚本数据类型

use serde::{Deserialize, Serialize};

/// RPG Maker 脚本
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpgScript {
    /// 脚本 ID
    pub id: u32,
    /// 脚本名称
    pub name: String,
    /// 脚本内容（压缩后的 base64 或原始文本）
    pub text: String,
}

impl Default for RpgScript {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::new(),
            text: String::new(),
        }
    }
}
