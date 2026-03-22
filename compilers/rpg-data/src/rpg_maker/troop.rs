//! RPG Maker 敌群数据类型

use serde::{Deserialize, Serialize};
use super::event::RpgEventCommand;

/// RPG Maker 敌群数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpgTroop {
    /// 敌群 ID
    #[serde(rename = "@id")]
    pub id: i32,
    /// 敌群名称
    #[serde(rename = "@name")]
    #[serde(default)]
    pub name: String,
    /// 成员列表
    #[serde(rename = "@members")]
    #[serde(default)]
    pub members: Vec<RpgTroopMember>,
    /// 页面列表
    #[serde(rename = "@pages")]
    #[serde(default)]
    pub pages: Vec<RpgTroopPage>,
}

impl Default for RpgTroop {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::new(),
            members: Vec::new(),
            pages: Vec::new(),
        }
    }
}

/// 敌群成员
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpgTroopMember {
    /// 敌人 ID
    #[serde(rename = "@enemy_id")]
    #[serde(default)]
    pub enemy_id: i32,
    /// X 坐标
    #[serde(rename = "@x")]
    #[serde(default)]
    pub x: i32,
    /// Y 坐标
    #[serde(rename = "@y")]
    #[serde(default)]
    pub y: i32,
    /// 是否隐藏
    #[serde(rename = "@hidden")]
    #[serde(default)]
    pub hidden: bool,
    /// 是否不死
    #[serde(rename = "@immortal")]
    #[serde(default)]
    pub immortal: bool,
}

impl Default for RpgTroopMember {
    fn default() -> Self {
        Self {
            enemy_id: 0,
            x: 0,
            y: 0,
            hidden: false,
            immortal: false,
        }
    }
}

/// 敌群页面
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpgTroopPage {
    /// 触发条件
    #[serde(rename = "@condition")]
    #[serde(default)]
    pub condition: RpgTroopPageCondition,
    /// 跨度
    #[serde(rename = "@span")]
    #[serde(default)]
    pub span: i32,
    /// 事件指令列表
    #[serde(rename = "@list")]
    #[serde(default)]
    pub list: Vec<RpgEventCommand>,
}

impl Default for RpgTroopPage {
    fn default() -> Self {
        Self {
            condition: RpgTroopPageCondition::default(),
            span: 0,
            list: vec![RpgEventCommand::default()],
        }
    }
}

/// 敌群页面条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpgTroopPageCondition {
    /// 回合条件是否有效
    #[serde(rename = "@turn_valid")]
    #[serde(default)]
    pub turn_valid: bool,
    /// 敌人条件是否有效
    #[serde(rename = "@enemy_valid")]
    #[serde(default)]
    pub enemy_valid: bool,
    /// 角色条件是否有效
    #[serde(rename = "@actor_valid")]
    #[serde(default)]
    pub actor_valid: bool,
    /// 开关条件是否有效
    #[serde(rename = "@switch_valid")]
    #[serde(default)]
    pub switch_valid: bool,
    /// 回合 A
    #[serde(rename = "@turn_a")]
    #[serde(default)]
    pub turn_a: i32,
    /// 回合 B
    #[serde(rename = "@turn_b")]
    #[serde(default)]
    pub turn_b: i32,
    /// 敌人索引
    #[serde(rename = "@enemy_index")]
    #[serde(default)]
    pub enemy_index: i32,
    /// 敌人 HP
    #[serde(rename = "@enemy_hp")]
    #[serde(default)]
    pub enemy_hp: i32,
    /// 角色 ID
    #[serde(rename = "@actor_id")]
    #[serde(default)]
    pub actor_id: i32,
    /// 角色 HP
    #[serde(rename = "@actor_hp")]
    #[serde(default)]
    pub actor_hp: i32,
    /// 开关 ID
    #[serde(rename = "@switch_id")]
    #[serde(default)]
    pub switch_id: i32,
}

impl Default for RpgTroopPageCondition {
    fn default() -> Self {
        Self {
            turn_valid: false,
            enemy_valid: false,
            actor_valid: false,
            switch_valid: false,
            turn_a: 0,
            turn_b: 0,
            enemy_index: 0,
            enemy_hp: 50,
            actor_id: 1,
            actor_hp: 50,
            switch_id: 1,
        }
    }
}
