//! RPG Maker 防具数据类型

use serde::{Deserialize, Serialize};
use super::shared::ArmorKind;

/// RPG Maker 防具数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpgArmor {
    /// 防具 ID
    #[serde(rename = "@id")]
    pub id: i32,
    /// 防具名称
    #[serde(rename = "@name")]
    pub name: String,
    /// 图标文件名
    #[serde(rename = "@icon_name")]
    pub icon_name: String,
    /// 描述
    #[serde(rename = "@description")]
    #[serde(default)]
    pub description: String,
    /// 类型
    #[serde(rename = "@kind")]
    #[serde(default)]
    pub kind: i32,
    /// 自动状态 ID
    #[serde(rename = "@auto_state_id")]
    #[serde(default)]
    pub auto_state_id: i32,
    /// 价格
    #[serde(rename = "@price")]
    #[serde(default)]
    pub price: i32,
    /// 物理防御
    #[serde(rename = "@pdef")]
    #[serde(default)]
    pub pdef: i32,
    /// 魔法防御
    #[serde(rename = "@mdef")]
    #[serde(default)]
    pub mdef: i32,
    /// 回避率
    #[serde(rename = "@eva")]
    #[serde(default)]
    pub eva: i32,
    /// 力量加成
    #[serde(rename = "@str_plus")]
    #[serde(default)]
    pub str_plus: i32,
    /// 灵巧加成
    #[serde(rename = "@dex_plus")]
    #[serde(default)]
    pub dex_plus: i32,
    /// 速度加成
    #[serde(rename = "@agi_plus")]
    #[serde(default)]
    pub agi_plus: i32,
    /// 魔力加成
    #[serde(rename = "@int_plus")]
    #[serde(default)]
    pub int_plus: i32,
    /// 防御元素集合
    #[serde(rename = "@guard_element_set")]
    #[serde(default)]
    pub guard_element_set: Vec<i32>,
    /// 防御状态集合
    #[serde(rename = "@guard_state_set")]
    #[serde(default)]
    pub guard_state_set: Vec<i32>,
}

impl Default for RpgArmor {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::new(),
            icon_name: String::new(),
            description: String::new(),
            kind: ArmorKind::Shield as i32,
            auto_state_id: 0,
            price: 0,
            pdef: 0,
            mdef: 0,
            eva: 0,
            str_plus: 0,
            dex_plus: 0,
            agi_plus: 0,
            int_plus: 0,
            guard_element_set: Vec::new(),
            guard_state_set: Vec::new(),
        }
    }
}
