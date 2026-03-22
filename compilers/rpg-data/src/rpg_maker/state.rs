//! RPG Maker 状态数据类型

use serde::{Deserialize, Serialize};
use super::shared::Restriction;

/// RPG Maker 状态数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpgState {
    /// 状态 ID
    #[serde(rename = "@id")]
    pub id: i32,
    /// 状态名称
    #[serde(rename = "@name")]
    pub name: String,
    /// 动画 ID
    #[serde(rename = "@animation_id")]
    #[serde(default)]
    pub animation_id: i32,
    /// 限制类型
    #[serde(rename = "@restriction")]
    #[serde(default)]
    pub restriction: i32,
    /// 是否无抵抗
    #[serde(rename = "@nonresistance")]
    #[serde(default)]
    pub nonresistance: bool,
    /// 是否 HP 为 0
    #[serde(rename = "@zero_hp")]
    #[serde(default)]
    pub zero_hp: bool,
    /// 是否无法获得经验
    #[serde(rename = "@cant_get_exp")]
    #[serde(default)]
    pub cant_get_exp: bool,
    /// 是否无法回避
    #[serde(rename = "@cant_evade")]
    #[serde(default)]
    pub cant_evade: bool,
    /// 是否滑落伤害
    #[serde(rename = "@slip_damage")]
    #[serde(default)]
    pub slip_damage: bool,
    /// 评级
    #[serde(rename = "@rating")]
    #[serde(default)]
    pub rating: i32,
    /// 命中率
    #[serde(rename = "@hit_rate")]
    #[serde(default)]
    pub hit_rate: i32,
    /// 最大 HP 倍率
    #[serde(rename = "@maxhp_rate")]
    #[serde(default)]
    pub maxhp_rate: i32,
    /// 最大 SP 倍率
    #[serde(rename = "@maxsp_rate")]
    #[serde(default)]
    pub maxsp_rate: i32,
    /// 力量倍率
    #[serde(rename = "@str_rate")]
    #[serde(default)]
    pub str_rate: i32,
    /// 灵巧倍率
    #[serde(rename = "@dex_rate")]
    #[serde(default)]
    pub dex_rate: i32,
    /// 速度倍率
    #[serde(rename = "@agi_rate")]
    #[serde(default)]
    pub agi_rate: i32,
    /// 魔力倍率
    #[serde(rename = "@int_rate")]
    #[serde(default)]
    pub int_rate: i32,
    /// 攻击力倍率
    #[serde(rename = "@atk_rate")]
    #[serde(default)]
    pub atk_rate: i32,
    /// 物理防御倍率
    #[serde(rename = "@pdef_rate")]
    #[serde(default)]
    pub pdef_rate: i32,
    /// 魔法防御倍率
    #[serde(rename = "@mdef_rate")]
    #[serde(default)]
    pub mdef_rate: i32,
    /// 回避率
    #[serde(rename = "@eva")]
    #[serde(default)]
    pub eva: i32,
    /// 是否仅战斗中有效
    #[serde(rename = "@battle_only")]
    #[serde(default)]
    pub battle_only: bool,
    /// 持续回合
    #[serde(rename = "@hold_turn")]
    #[serde(default)]
    pub hold_turn: i32,
    /// 自动解除概率
    #[serde(rename = "@auto_release_prob")]
    #[serde(default)]
    pub auto_release_prob: i32,
    /// 受击解除概率
    #[serde(rename = "@shock_release_prob")]
    #[serde(default)]
    pub shock_release_prob: i32,
    /// 防御元素集合
    #[serde(rename = "@guard_element_set")]
    #[serde(default)]
    pub guard_element_set: Vec<i32>,
    /// 附加状态集合
    #[serde(rename = "@plus_state_set")]
    #[serde(default)]
    pub plus_state_set: Vec<i32>,
    /// 解除状态集合
    #[serde(rename = "@minus_state_set")]
    #[serde(default)]
    pub minus_state_set: Vec<i32>,
}

impl Default for RpgState {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::new(),
            animation_id: 0,
            restriction: Restriction::None as i32,
            nonresistance: false,
            zero_hp: false,
            cant_get_exp: false,
            cant_evade: false,
            slip_damage: false,
            rating: 5,
            hit_rate: 100,
            maxhp_rate: 100,
            maxsp_rate: 100,
            str_rate: 100,
            dex_rate: 100,
            agi_rate: 100,
            int_rate: 100,
            atk_rate: 100,
            pdef_rate: 100,
            mdef_rate: 100,
            eva: 0,
            battle_only: true,
            hold_turn: 0,
            auto_release_prob: 0,
            shock_release_prob: 0,
            guard_element_set: Vec::new(),
            plus_state_set: Vec::new(),
            minus_state_set: Vec::new(),
        }
    }
}
