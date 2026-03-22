//! RPG Maker 角色数据类型

use serde::{Deserialize, Serialize};

/// RPG Maker 角色数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpgActor {
    /// 角色 ID
    #[serde(rename = "@id")]
    pub id: i32,
    /// 角色名称
    #[serde(rename = "@name")]
    pub name: String,
    /// 角色类 ID
    #[serde(rename = "@class_id")]
    pub class_id: i32,
    /// 初始等级
    #[serde(rename = "@initial_level")]
    pub initial_level: i32,
    /// 最终等级
    #[serde(rename = "@final_level")]
    pub final_level: i32,
    /// 经验值基础值
    #[serde(rename = "@exp_basis")]
    pub exp_basis: i32,
    /// 经验值增长率
    #[serde(rename = "@exp_inflation")]
    pub exp_inflation: i32,
    /// 角色图形文件名
    #[serde(rename = "@character_name")]
    pub character_name: String,
    /// 角色图形色相
    #[serde(rename = "@character_hue")]
    pub character_hue: i32,
    /// 战斗图形文件名
    #[serde(rename = "@battler_name")]
    pub battler_name: String,
    /// 战斗图形色相
    #[serde(rename = "@battler_hue")]
    pub battler_hue: i32,
    /// 参数表（Table 类型，base64 编码）
    #[serde(rename = "@parameters")]
    pub parameters: Option<String>,
    /// 武器 ID
    #[serde(rename = "@weapon_id")]
    pub weapon_id: i32,
    /// 防具 1 ID
    #[serde(rename = "@armor1_id")]
    pub armor1_id: i32,
    /// 防具 2 ID
    #[serde(rename = "@armor2_id")]
    pub armor2_id: i32,
    /// 防具 3 ID
    #[serde(rename = "@armor3_id")]
    pub armor3_id: i32,
    /// 防具 4 ID
    #[serde(rename = "@armor4_id")]
    pub armor4_id: i32,
    /// 武器是否固定
    #[serde(rename = "@weapon_fix")]
    #[serde(default)]
    pub weapon_fix: bool,
    /// 防具 1 是否固定
    #[serde(rename = "@armor1_fix")]
    #[serde(default)]
    pub armor1_fix: bool,
    /// 防具 2 是否固定
    #[serde(rename = "@armor2_fix")]
    #[serde(default)]
    pub armor2_fix: bool,
    /// 防具 3 是否固定
    #[serde(rename = "@armor3_fix")]
    #[serde(default)]
    pub armor3_fix: bool,
    /// 防具 4 是否固定
    #[serde(rename = "@armor4_fix")]
    #[serde(default)]
    pub armor4_fix: bool,
}

impl Default for RpgActor {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::new(),
            class_id: 1,
            initial_level: 1,
            final_level: 99,
            exp_basis: 25,
            exp_inflation: 35,
            character_name: String::new(),
            character_hue: 0,
            battler_name: String::new(),
            battler_hue: 0,
            parameters: None,
            weapon_id: 0,
            armor1_id: 0,
            armor2_id: 0,
            armor3_id: 0,
            armor4_id: 0,
            weapon_fix: false,
            armor1_fix: false,
            armor2_fix: false,
            armor3_fix: false,
            armor4_fix: false,
        }
    }
}
