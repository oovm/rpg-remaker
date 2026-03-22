//! RPG Maker 武器数据类型

use serde::{Deserialize, Serialize};

/// RPG Maker 武器数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpgWeapon {
    /// 武器 ID
    #[serde(rename = "@id")]
    pub id: i32,
    /// 武器名称
    #[serde(rename = "@name")]
    pub name: String,
    /// 图标文件名
    #[serde(rename = "@icon_name")]
    pub icon_name: String,
    /// 描述
    #[serde(rename = "@description")]
    #[serde(default)]
    pub description: String,
    /// 动画 1 ID
    #[serde(rename = "@animation1_id")]
    #[serde(default)]
    pub animation1_id: i32,
    /// 动画 2 ID
    #[serde(rename = "@animation2_id")]
    #[serde(default)]
    pub animation2_id: i32,
    /// 价格
    #[serde(rename = "@price")]
    #[serde(default)]
    pub price: i32,
    /// 攻击力
    #[serde(rename = "@atk")]
    #[serde(default)]
    pub atk: i32,
    /// 物理防御
    #[serde(rename = "@pdef")]
    #[serde(default)]
    pub pdef: i32,
    /// 魔法防御
    #[serde(rename = "@mdef")]
    #[serde(default)]
    pub mdef: i32,
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
    /// 元素集合
    #[serde(rename = "@element_set")]
    #[serde(default)]
    pub element_set: Vec<i32>,
    /// 附加状态集合
    #[serde(rename = "@plus_state_set")]
    #[serde(default)]
    pub plus_state_set: Vec<i32>,
    /// 解除状态集合
    #[serde(rename = "@minus_state_set")]
    #[serde(default)]
    pub minus_state_set: Vec<i32>,
}

impl Default for RpgWeapon {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::new(),
            icon_name: String::new(),
            description: String::new(),
            animation1_id: 0,
            animation2_id: 0,
            price: 0,
            atk: 0,
            pdef: 0,
            mdef: 0,
            str_plus: 0,
            dex_plus: 0,
            agi_plus: 0,
            int_plus: 0,
            element_set: Vec::new(),
            plus_state_set: Vec::new(),
            minus_state_set: Vec::new(),
        }
    }
}
