//! RPG Maker 动画数据类型

use serde::{Deserialize, Serialize};
use super::shared::{AudioFile, AnimationPosition};

/// RPG Maker 动画数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpgAnimation {
    /// 动画 ID
    #[serde(rename = "@id")]
    pub id: i32,
    /// 动画名称
    #[serde(rename = "@name")]
    #[serde(default)]
    pub name: String,
    /// 动画文件名
    #[serde(rename = "@animation_name")]
    #[serde(default)]
    pub animation_name: String,
    /// 动画色相
    #[serde(rename = "@animation_hue")]
    #[serde(default)]
    pub animation_hue: i32,
    /// 动画位置
    #[serde(rename = "@position")]
    #[serde(default)]
    pub position: i32,
    /// 帧数
    #[serde(rename = "@frame_max")]
    #[serde(default)]
    pub frame_max: i32,
    /// 帧列表
    #[serde(rename = "@frames")]
    #[serde(default)]
    pub frames: Vec<RpgAnimationFrame>,
    /// 时机列表
    #[serde(rename = "@timings")]
    #[serde(default)]
    pub timings: Vec<RpgAnimationTiming>,
}

impl Default for RpgAnimation {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::new(),
            animation_name: String::new(),
            animation_hue: 0,
            position: AnimationPosition::Middle as i32,
            frame_max: 1,
            frames: Vec::new(),
            timings: Vec::new(),
        }
    }
}

/// 动画帧
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpgAnimationFrame {
    /// 单元格数量
    #[serde(rename = "@cell_max")]
    #[serde(default)]
    pub cell_max: i32,
    /// 单元格数据（Table 类型）
    #[serde(rename = "@cell_data")]
    pub cell_data: Option<String>,
}

impl Default for RpgAnimationFrame {
    fn default() -> Self {
        Self {
            cell_max: 0,
            cell_data: None,
        }
    }
}

/// 动画时机
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpgAnimationTiming {
    /// 帧索引
    #[serde(rename = "@frame")]
    #[serde(default)]
    pub frame: i32,
    /// 音效
    #[serde(rename = "@se")]
    #[serde(default)]
    pub se: AudioFile,
    /// 闪烁范围
    #[serde(rename = "@flash_scope")]
    #[serde(default)]
    pub flash_scope: i32,
    /// 闪烁颜色（Color 类型）
    #[serde(rename = "@flash_color")]
    pub flash_color: Option<String>,
    /// 闪烁持续时间
    #[serde(rename = "@flash_duration")]
    #[serde(default)]
    pub flash_duration: i32,
    /// 条件
    #[serde(rename = "@condition")]
    #[serde(default)]
    pub condition: i32,
}

impl Default for RpgAnimationTiming {
    fn default() -> Self {
        Self {
            frame: 0,
            se: AudioFile::default(),
            flash_scope: AnimationFlashScope::None as i32,
            flash_color: None,
            flash_duration: 5,
            condition: AnimationCondition::None as i32,
        }
    }
}

/// 动画闪烁范围
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(into = "i32", try_from = "i32")]
#[repr(i32)]
pub enum AnimationFlashScope {
    #[default]
    None = 0,
    Target = 1,
    Screen = 2,
    HideTarget = 3,
}

impl From<AnimationFlashScope> for i32 {
    fn from(value: AnimationFlashScope) -> Self {
        value as i32
    }
}

impl TryFrom<i32> for AnimationFlashScope {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Target),
            2 => Ok(Self::Screen),
            3 => Ok(Self::HideTarget),
            _ => Err(format!("invalid AnimationFlashScope value: {}", value)),
        }
    }
}

/// 动画条件
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(into = "i32", try_from = "i32")]
#[repr(i32)]
pub enum AnimationCondition {
    #[default]
    None = 0,
    Hit = 1,
    Miss = 2,
}

impl From<AnimationCondition> for i32 {
    fn from(value: AnimationCondition) -> Self {
        value as i32
    }
}

impl TryFrom<i32> for AnimationCondition {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Hit),
            2 => Ok(Self::Miss),
            _ => Err(format!("invalid AnimationCondition value: {}", value)),
        }
    }
}
