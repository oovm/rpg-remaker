//! RPG Maker 敌人数据类型

use serde::{Deserialize, Serialize};
use super::shared::{ActionKind, BasicAction};

/// RPG Maker 敌人数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpgEnemy {
    /// 敌人 ID
    #[serde(rename = "@id")]
    pub id: i32,
    /// 敌人名称
    #[serde(rename = "@name")]
    pub name: String,
    /// 战斗图形文件名
    #[serde(rename = "@battler_name")]
    #[serde(default)]
    pub battler_name: String,
    /// 战斗图形色相
    #[serde(rename = "@battler_hue")]
    #[serde(default)]
    pub battler_hue: i32,
    /// 最大 HP
    #[serde(rename = "@maxhp")]
    #[serde(default)]
    pub maxhp: i32,
    /// 最大 SP
    #[serde(rename = "@maxsp")]
    #[serde(default)]
    pub maxsp: i32,
    /// 力量
    #[serde(rename = "@str")]
    #[serde(default)]
    pub str: i32,
    /// 灵巧
    #[serde(rename = "@dex")]
    #[serde(default)]
    pub dex: i32,
    /// 速度
    #[serde(rename = "@agi")]
    #[serde(default)]
    pub agi: i32,
    /// 魔力
    #[serde(rename = "@int")]
    #[serde(default)]
    pub int: i32,
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
    /// 回避率
    #[serde(rename = "@eva")]
    #[serde(default)]
    pub eva: i32,
    /// 动画 1 ID
    #[serde(rename = "@animation1_id")]
    #[serde(default)]
    pub animation1_id: i32,
    /// 动画 2 ID
    #[serde(rename = "@animation2_id")]
    #[serde(default)]
    pub animation2_id: i32,
    /// 元素抗性表（Table 类型）
    #[serde(rename = "@element_ranks")]
    pub element_ranks: Option<String>,
    /// 状态抗性表（Table 类型）
    #[serde(rename = "@state_ranks")]
    pub state_ranks: Option<String>,
    /// 行动列表
    #[serde(rename = "@actions")]
    #[serde(default)]
    pub actions: Vec<RpgEnemyAction>,
    /// 经验值
    #[serde(rename = "@exp")]
    #[serde(default)]
    pub exp: i32,
    /// 金币
    #[serde(rename = "@gold")]
    #[serde(default)]
    pub gold: i32,
    /// 掉落物品 ID
    #[serde(rename = "@item_id")]
    #[serde(default)]
    pub item_id: i32,
    /// 掉落武器 ID
    #[serde(rename = "@weapon_id")]
    #[serde(default)]
    pub weapon_id: i32,
    /// 掉落防具 ID
    #[serde(rename = "@armor_id")]
    #[serde(default)]
    pub armor_id: i32,
    /// 掉落概率
    #[serde(rename = "@treasure_prob")]
    #[serde(default)]
    pub treasure_prob: i32,
}

impl Default for RpgEnemy {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::new(),
            battler_name: String::new(),
            battler_hue: 0,
            maxhp: 100,
            maxsp: 100,
            str: 10,
            dex: 10,
            agi: 10,
            int: 10,
            atk: 10,
            pdef: 10,
            mdef: 10,
            eva: 0,
            animation1_id: 0,
            animation2_id: 0,
            element_ranks: None,
            state_ranks: None,
            actions: Vec::new(),
            exp: 0,
            gold: 0,
            item_id: 0,
            weapon_id: 0,
            armor_id: 0,
            treasure_prob: 100,
        }
    }
}

/// 敌人行动
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpgEnemyAction {
    /// 行动类型
    #[serde(rename = "@kind")]
    #[serde(default)]
    pub kind: i32,
    /// 基础行动
    #[serde(rename = "@basic")]
    #[serde(default)]
    pub basic: i32,
    /// 技能 ID
    #[serde(rename = "@skill_id")]
    #[serde(default)]
    pub skill_id: i32,
    /// 条件回合 A
    #[serde(rename = "@condition_turn_a")]
    #[serde(default)]
    pub condition_turn_a: i32,
    /// 条件回合 B
    #[serde(rename = "@condition_turn_b")]
    #[serde(default)]
    pub condition_turn_b: i32,
    /// 条件 HP
    #[serde(rename = "@condition_hp")]
    #[serde(default)]
    pub condition_hp: i32,
    /// 条件等级
    #[serde(rename = "@condition_level")]
    #[serde(default)]
    pub condition_level: i32,
    /// 条件开关 ID
    #[serde(rename = "@condition_switch_id")]
    #[serde(default)]
    pub condition_switch_id: i32,
    /// 优先级
    #[serde(rename = "@rating")]
    #[serde(default)]
    pub rating: i32,
}

impl Default for RpgEnemyAction {
    fn default() -> Self {
        Self {
            kind: ActionKind::Basic as i32,
            basic: BasicAction::Attack as i32,
            skill_id: 1,
            condition_turn_a: 0,
            condition_turn_b: 1,
            condition_hp: 100,
            condition_level: 1,
            condition_switch_id: 0,
            rating: 5,
        }
    }
}
