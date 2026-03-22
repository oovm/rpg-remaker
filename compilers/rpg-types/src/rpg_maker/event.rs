//! RPG Maker 事件数据类型

use serde::{Deserialize, Serialize};
use super::shared::{BlendMode, MoveType, MoveSpeed, MoveFrequency, EventTrigger};

/// RPG Maker 事件数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpgEvent {
    /// 事件 ID
    #[serde(rename = "@id")]
    pub id: i32,
    /// 事件名称
    #[serde(rename = "@name")]
    #[serde(default)]
    pub name: String,
    /// X 坐标
    #[serde(rename = "@x")]
    #[serde(default)]
    pub x: i32,
    /// Y 坐标
    #[serde(rename = "@y")]
    #[serde(default)]
    pub y: i32,
    /// 页面列表
    #[serde(rename = "@pages")]
    #[serde(default)]
    pub pages: Vec<RpgEventPage>,
}

impl Default for RpgEvent {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::new(),
            x: 0,
            y: 0,
            pages: Vec::new(),
        }
    }
}

/// 公共事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpgCommonEvent {
    /// 公共事件 ID
    #[serde(rename = "@id")]
    pub id: i32,
    /// 公共事件名称
    #[serde(rename = "@name")]
    #[serde(default)]
    pub name: String,
    /// 触发方式
    #[serde(rename = "@trigger")]
    #[serde(default)]
    pub trigger: i32,
    /// 开关 ID
    #[serde(rename = "@switch_id")]
    #[serde(default)]
    pub switch_id: i32,
    /// 事件指令列表
    #[serde(rename = "@list")]
    #[serde(default)]
    pub list: Vec<RpgEventCommand>,
}

impl Default for RpgCommonEvent {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::new(),
            trigger: 0,
            switch_id: 0,
            list: vec![RpgEventCommand::default()],
        }
    }
}

/// 事件页面
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpgEventPage {
    /// 触发条件
    #[serde(rename = "@condition")]
    #[serde(default)]
    pub condition: RpgEventCondition,
    /// 图形
    #[serde(rename = "@graphic")]
    #[serde(default)]
    pub graphic: RpgEventGraphic,
    /// 移动类型
    #[serde(rename = "@move_type")]
    #[serde(default)]
    pub move_type: i32,
    /// 移动速度
    #[serde(rename = "@move_speed")]
    #[serde(default)]
    pub move_speed: i32,
    /// 移动频率
    #[serde(rename = "@move_frequency")]
    #[serde(default)]
    pub move_frequency: i32,
    /// 移动路线
    #[serde(rename = "@move_route")]
    #[serde(default)]
    pub move_route: RpgMoveRoute,
    /// 步行动画
    #[serde(rename = "@walk_anime")]
    #[serde(default = "default_true")]
    pub walk_anime: bool,
    /// 踏步动画
    #[serde(rename = "@step_anime")]
    #[serde(default)]
    pub step_anime: bool,
    /// 方向固定
    #[serde(rename = "@direction_fix")]
    #[serde(default)]
    pub direction_fix: bool,
    /// 穿透
    #[serde(rename = "@through")]
    #[serde(default)]
    pub through: bool,
    /// 始终置顶
    #[serde(rename = "@always_on_top")]
    #[serde(default)]
    pub always_on_top: bool,
    /// 触发方式
    #[serde(rename = "@trigger")]
    #[serde(default)]
    pub trigger: i32,
    /// 事件指令列表
    #[serde(rename = "@list")]
    #[serde(default)]
    pub list: Vec<RpgEventCommand>,
}

fn default_true() -> bool { true }

impl Default for RpgEventPage {
    fn default() -> Self {
        Self {
            condition: RpgEventCondition::default(),
            graphic: RpgEventGraphic::default(),
            move_type: MoveType::Fixed as i32,
            move_speed: MoveSpeed::Fast as i32,
            move_frequency: MoveFrequency::High as i32,
            move_route: RpgMoveRoute::default(),
            walk_anime: true,
            step_anime: false,
            direction_fix: false,
            through: false,
            always_on_top: false,
            trigger: EventTrigger::ActionButton as i32,
            list: vec![RpgEventCommand::default()],
        }
    }
}

/// 事件触发条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpgEventCondition {
    /// 开关 1 是否有效
    #[serde(rename = "@switch1_valid")]
    #[serde(default)]
    pub switch1_valid: bool,
    /// 开关 2 是否有效
    #[serde(rename = "@switch2_valid")]
    #[serde(default)]
    pub switch2_valid: bool,
    /// 变量是否有效
    #[serde(rename = "@variable_valid")]
    #[serde(default)]
    pub variable_valid: bool,
    /// 自开关是否有效
    #[serde(rename = "@self_switch_valid")]
    #[serde(default)]
    pub self_switch_valid: bool,
    /// 开关 1 ID
    #[serde(rename = "@switch1_id")]
    #[serde(default)]
    pub switch1_id: i32,
    /// 开关 2 ID
    #[serde(rename = "@switch2_id")]
    #[serde(default)]
    pub switch2_id: i32,
    /// 变量 ID
    #[serde(rename = "@variable_id")]
    #[serde(default)]
    pub variable_id: i32,
    /// 变量值
    #[serde(rename = "@variable_value")]
    #[serde(default)]
    pub variable_value: i32,
    /// 自开关通道
    #[serde(rename = "@self_switch_ch")]
    #[serde(default = "default_self_switch")]
    pub self_switch_ch: String,
}

fn default_self_switch() -> String { "A".to_string() }

impl Default for RpgEventCondition {
    fn default() -> Self {
        Self {
            switch1_valid: false,
            switch2_valid: false,
            variable_valid: false,
            self_switch_valid: false,
            switch1_id: 1,
            switch2_id: 1,
            variable_id: 1,
            variable_value: 0,
            self_switch_ch: default_self_switch(),
        }
    }
}

/// 事件图形
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpgEventGraphic {
    /// 图块 ID
    #[serde(rename = "@tile_id")]
    #[serde(default)]
    pub tile_id: i32,
    /// 角色图形文件名
    #[serde(rename = "@character_name")]
    #[serde(default)]
    pub character_name: String,
    /// 角色图形色相
    #[serde(rename = "@character_hue")]
    #[serde(default)]
    pub character_hue: i32,
    /// 方向
    #[serde(rename = "@direction")]
    #[serde(default)]
    pub direction: i32,
    /// 图案
    #[serde(rename = "@pattern")]
    #[serde(default)]
    pub pattern: i32,
    /// 透明度
    #[serde(rename = "@opacity")]
    #[serde(default = "default_opacity")]
    pub opacity: i32,
    /// 混合模式
    #[serde(rename = "@blend_type")]
    #[serde(default)]
    pub blend_type: i32,
}

fn default_opacity() -> i32 { 255 }

impl Default for RpgEventGraphic {
    fn default() -> Self {
        Self {
            tile_id: 0,
            character_name: String::new(),
            character_hue: 0,
            direction: 2,
            pattern: 0,
            opacity: 255,
            blend_type: BlendMode::Normal as i32,
        }
    }
}

/// 移动路线
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RpgMoveRoute {
    /// 是否重复
    #[serde(rename = "@repeat")]
    #[serde(default = "default_true")]
    pub repeat: bool,
    /// 是否可跳过
    #[serde(rename = "@skippable")]
    #[serde(default)]
    pub skippable: bool,
    /// 移动指令列表
    #[serde(rename = "@list")]
    #[serde(default)]
    pub list: Vec<RpgMoveCommand>,
}

impl Default for RpgMoveRoute {
    fn default() -> Self {
        Self {
            repeat: true,
            skippable: false,
            list: vec![RpgMoveCommand::default()],
        }
    }
}

/// 移动指令
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RpgMoveCommand {
    /// 指令代码
    #[serde(rename = "@code")]
    #[serde(default)]
    pub code: u16,
    /// 参数列表
    #[serde(rename = "@parameters")]
    #[serde(default)]
    pub parameters: Vec<serde_json::Value>,
}

impl Default for RpgMoveCommand {
    fn default() -> Self {
        Self {
            code: 0,
            parameters: Vec::new(),
        }
    }
}

/// 事件指令
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpgEventCommand {
    /// 指令代码
    #[serde(rename = "@code")]
    #[serde(default)]
    pub code: u16,
    /// 缩进
    #[serde(rename = "@indent")]
    #[serde(default)]
    pub indent: i32,
    /// 参数列表
    #[serde(rename = "@parameters")]
    #[serde(default)]
    pub parameters: Vec<serde_json::Value>,
}

impl Default for RpgEventCommand {
    fn default() -> Self {
        Self {
            code: 0,
            indent: 0,
            parameters: Vec::new(),
        }
    }
}
