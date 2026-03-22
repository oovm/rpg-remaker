//! RPG Maker 职业数据类型

use serde::{Deserialize, Serialize};
use super::shared::Position;

/// RPG Maker 职业数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpgClass {
    /// 职业 ID
    #[serde(rename = "@id")]
    pub id: i32,
    /// 职业名称
    #[serde(rename = "@name")]
    pub name: String,
    /// 战斗位置
    #[serde(rename = "@position")]
    #[serde(default)]
    pub position: i32,
    /// 可装备武器列表
    #[serde(rename = "@weapon_set")]
    #[serde(default)]
    pub weapon_set: Vec<i32>,
    /// 可装备防具列表
    #[serde(rename = "@armor_set")]
    #[serde(default)]
    pub armor_set: Vec<i32>,
    /// 元素抗性表（Table 类型）
    #[serde(rename = "@element_ranks")]
    pub element_ranks: Option<String>,
    /// 状态抗性表（Table 类型）
    #[serde(rename = "@state_ranks")]
    pub state_ranks: Option<String>,
    /// 学习技能列表
    #[serde(rename = "@learnings")]
    #[serde(default)]
    pub learnings: Vec<RpgLearning>,
}

impl Default for RpgClass {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::new(),
            position: Position::Front as i32,
            weapon_set: Vec::new(),
            armor_set: Vec::new(),
            element_ranks: None,
            state_ranks: None,
            learnings: Vec::new(),
        }
    }
}

/// 技能学习
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpgLearning {
    /// 学习等级
    #[serde(rename = "@level")]
    pub level: i32,
    /// 技能 ID
    #[serde(rename = "@skill_id")]
    pub skill_id: i32,
}

impl Default for RpgLearning {
    fn default() -> Self {
        Self { level: 1, skill_id: 0 }
    }
}
