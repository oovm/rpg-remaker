//! RPG Maker 角色数据类型

use crate::{Result, ruby_value::RubyValue};
use serde::{Deserialize, Serialize};

/// RPG Maker 角色数据
///
/// 表示 RPG Maker 游戏中的角色（Actor）数据，
/// 包含角色的基本信息、属性、装备等。
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
    /// 经验值基础值
    #[serde(rename = "@exp_basis")]
    pub exp_basis: i32,
    /// 经验值增长率
    #[serde(rename = "@exp_inflation")]
    pub exp_inflation: i32,
    /// 武器 ID
    #[serde(rename = "@weapon_id")]
    pub weapon_id: i32,
    /// 武器是否固定
    #[serde(rename = "@weapon_fix")]
    pub weapon_fix: bool,
    /// 防具 1 ID
    #[serde(rename = "@armor1_id")]
    pub armor1_id: i32,
    /// 防具 1 是否固定
    #[serde(rename = "@armor1_fix")]
    pub armor1_fix: bool,
    /// 防具 2 ID
    #[serde(rename = "@armor2_id")]
    pub armor2_id: i32,
    /// 防具 2 是否固定
    #[serde(rename = "@armor2_fix")]
    pub armor2_fix: bool,
    /// 防具 3 ID
    #[serde(rename = "@armor3_id")]
    pub armor3_id: i32,
    /// 防具 3 是否固定
    #[serde(rename = "@armor3_fix")]
    pub armor3_fix: bool,
    /// 防具 4 ID
    #[serde(rename = "@armor4_id")]
    pub armor4_id: i32,
    /// 防具 4 是否固定
    #[serde(rename = "@armor4_fix")]
    pub armor4_fix: bool,
    /// 参数表（Table 类型）
    #[serde(rename = "@parameters")]
    pub parameters: Option<RubyValue>,
}

impl RpgActor {
    /// 从 RubyValue 创建 RpgActor
    pub fn from_ruby_value(value: &RubyValue) -> Result<Self> {
        if let RubyValue::Object { class, fields } = value {
            if class != "RPG::Actor" {
                return Err(crate::errors::RpgError::decode("RpgActor", &format!("expected RPG::Actor, got {}", class)));
            }

            let temp_value = RubyValue::Object { class: "__temp".to_string(), fields: fields.clone() };

            let yaml =
                serde_yaml::to_string(&temp_value).map_err(|e| crate::errors::RpgError::decode("RpgActor", &e.to_string()))?;

            let actor: Self =
                serde_yaml::from_str(&yaml).map_err(|e| crate::errors::RpgError::decode("RpgActor", &e.to_string()))?;

            Ok(actor)
        }
        else {
            Err(crate::errors::RpgError::decode("RpgActor", "expected RubyValue::Object"))
        }
    }

    /// 转换为 RubyValue
    pub fn to_ruby_value(&self) -> RubyValue {
        let yaml = serde_yaml::to_string(self).expect("Failed to serialize RpgActor to YAML");

        let temp_value: RubyValue = serde_yaml::from_str(&yaml).expect("Failed to deserialize YAML to RubyValue");

        if let RubyValue::Object { fields, .. } = temp_value {
            RubyValue::Object { class: "RPG::Actor".to_string(), fields }
        }
        else {
            panic!("Expected Object structure");
        }
    }
}
