//! RPG Maker 物品数据类型

use serde::{Deserialize, Serialize};
use super::shared::{AudioFile, Occasion, Scope, ParameterType};

/// RPG Maker 物品数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpgItem {
    /// 物品 ID
    #[serde(rename = "@id")]
    pub id: i32,
    /// 物品名称
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
    /// 价格
    #[serde(rename = "@price")]
    #[serde(default)]
    pub price: i32,
    /// 是否消耗
    #[serde(rename = "@consumable")]
    #[serde(default = "default_true")]
    pub consumable: bool,
    /// 参数类型
    #[serde(rename = "@parameter_type")]
    #[serde(default)]
    pub parameter_type: i32,
    /// 参数值
    #[serde(rename = "@parameter_points")]
    #[serde(default)]
    pub parameter_points: i32,
    /// HP 恢复率
    #[serde(rename = "@recover_hp_rate")]
    #[serde(default)]
    pub recover_hp_rate: i32,
    /// HP 恢复值
    #[serde(rename = "@recover_hp")]
    #[serde(default)]
    pub recover_hp: i32,
    /// SP 恢复率
    #[serde(rename = "@recover_sp_rate")]
    #[serde(default)]
    pub recover_sp_rate: i32,
    /// SP 恢复值
    #[serde(rename = "@recover_sp")]
    #[serde(default)]
    pub recover_sp: i32,
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

fn default_true() -> bool { true }

impl Default for RpgItem {
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
            price: 0,
            consumable: true,
            parameter_type: ParameterType::None as i32,
            parameter_points: 0,
            recover_hp_rate: 0,
            recover_hp: 0,
            recover_sp_rate: 0,
            recover_sp: 0,
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
