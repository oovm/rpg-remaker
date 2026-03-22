//! RPG Maker 技能数据类型

use serde::{Deserialize, Serialize};
use super::shared::{AudioFile, Occasion, Scope};

/// RPG Maker 技能数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpgSkill {
    /// 技能 ID
    #[serde(rename = "@id")]
    pub id: i32,
    /// 技能名称
    #[serde(rename = "@name")]
    pub name: String,
    /// 图标文件名
    #[serde(rename = "@icon_name")]
    pub icon_name: String,
    /// 描述
    #[serde(rename = "@description")]
    #[serde(default)]
    pub description: String,
    /// 作用范围
    #[serde(rename = "@scope")]
    #[serde(default)]
    pub scope: i32,
    /// 使用场合
    #[serde(rename = "@occasion")]
    #[serde(default)]
    pub occasion: i32,
    /// 动画 1 ID
    #[serde(rename = "@animation1_id")]
    #[serde(default)]
    pub animation1_id: i32,
    /// 动画 2 ID
    #[serde(rename = "@animation2_id")]
    #[serde(default)]
    pub animation2_id: i32,
    /// 菜单音效
    #[serde(rename = "@menu_se")]
    #[serde(default)]
    pub menu_se: AudioFile,
    /// 公共事件 ID
    #[serde(rename = "@common_event_id")]
    #[serde(default)]
    pub common_event_id: i32,
    /// SP 消耗
    #[serde(rename = "@sp_cost")]
    #[serde(default)]
    pub sp_cost: i32,
    /// 威力
    #[serde(rename = "@power")]
    #[serde(default)]
    pub power: i32,
    /// 攻击力影响
    #[serde(rename = "@atk_f")]
    #[serde(default)]
    pub atk_f: i32,
    /// 回避率影响
    #[serde(rename = "@eva_f")]
    #[serde(default)]
    pub eva_f: i32,
    /// 力量影响
    #[serde(rename = "@str_f")]
    #[serde(default)]
    pub str_f: i32,
    /// 灵巧影响
    #[serde(rename = "@dex_f")]
    #[serde(default)]
    pub dex_f: i32,
    /// 速度影响
    #[serde(rename = "@agi_f")]
    #[serde(default)]
    pub agi_f: i32,
    /// 魔力影响
    #[serde(rename = "@int_f")]
    #[serde(default)]
    pub int_f: i32,
    /// 命中率
    #[serde(rename = "@hit")]
    #[serde(default)]
    pub hit: i32,
    /// 物理防御影响
    #[serde(rename = "@pdef_f")]
    #[serde(default)]
    pub pdef_f: i32,
    /// 魔法防御影响
    #[serde(rename = "@mdef_f")]
    #[serde(default)]
    pub mdef_f: i32,
    /// 分散度
    #[serde(rename = "@variance")]
    #[serde(default)]
    pub variance: i32,
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

impl Default for RpgSkill {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::new(),
            icon_name: String::new(),
            description: String::new(),
            scope: Scope::None as i32,
            occasion: Occasion::Always as i32,
            animation1_id: 0,
            animation2_id: 0,
            menu_se: AudioFile::default(),
            common_event_id: 0,
            sp_cost: 0,
            power: 0,
            atk_f: 0,
            eva_f: 0,
            str_f: 0,
            dex_f: 0,
            agi_f: 0,
            int_f: 0,
            hit: 100,
            pdef_f: 0,
            mdef_f: 0,
            variance: 15,
            element_set: Vec::new(),
            plus_state_set: Vec::new(),
            minus_state_set: Vec::new(),
        }
    }
}
