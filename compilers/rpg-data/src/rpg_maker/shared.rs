//! RPG Maker 共享类型定义

use serde::{Deserialize, Serialize};

/// 音频文件
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AudioFile {
    /// 文件名
    #[serde(rename = "@name")]
    pub name: String,
    /// 音量
    #[serde(rename = "@volume")]
    pub volume: i32,
    /// 音调
    #[serde(rename = "@pitch")]
    pub pitch: i32,
}

/// 作用范围
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(into = "i32", try_from = "i32")]
#[repr(i32)]
pub enum Scope {
    #[default]
    None = 0,
    OneEnemy = 1,
    AllEnemies = 2,
    OneAlly = 3,
    AllAllies = 4,
    OneAllyHp0 = 5,
    AllAlliesHp0 = 6,
    User = 7,
}

impl From<Scope> for i32 {
    fn from(value: Scope) -> Self {
        value as i32
    }
}

impl TryFrom<i32> for Scope {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::OneEnemy),
            2 => Ok(Self::AllEnemies),
            3 => Ok(Self::OneAlly),
            4 => Ok(Self::AllAllies),
            5 => Ok(Self::OneAllyHp0),
            6 => Ok(Self::AllAlliesHp0),
            7 => Ok(Self::User),
            _ => Err(format!("invalid Scope value: {}", value)),
        }
    }
}

/// 使用场合
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(into = "i32", try_from = "i32")]
#[repr(i32)]
pub enum Occasion {
    #[default]
    Always = 0,
    OnlyBattle = 1,
    OnlyMenu = 2,
    Never = 3,
}

impl From<Occasion> for i32 {
    fn from(value: Occasion) -> Self {
        value as i32
    }
}

impl TryFrom<i32> for Occasion {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Always),
            1 => Ok(Self::OnlyBattle),
            2 => Ok(Self::OnlyMenu),
            3 => Ok(Self::Never),
            _ => Err(format!("invalid Occasion value: {}", value)),
        }
    }
}

/// 参数类型
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(into = "i32", try_from = "i32")]
#[repr(i32)]
pub enum ParameterType {
    #[default]
    None = 0,
    MaxHP = 1,
    MaxSP = 2,
    Str = 3,
    Dex = 4,
    Agi = 5,
    Int = 6,
}

impl From<ParameterType> for i32 {
    fn from(value: ParameterType) -> Self {
        value as i32
    }
}

impl TryFrom<i32> for ParameterType {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::MaxHP),
            2 => Ok(Self::MaxSP),
            3 => Ok(Self::Str),
            4 => Ok(Self::Dex),
            5 => Ok(Self::Agi),
            6 => Ok(Self::Int),
            _ => Err(format!("invalid ParameterType value: {}", value)),
        }
    }
}

/// 混合模式
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(into = "i32", try_from = "i32")]
#[repr(i32)]
pub enum BlendMode {
    #[default]
    Normal = 0,
    Add = 1,
    Subtract = 2,
}

impl From<BlendMode> for i32 {
    fn from(value: BlendMode) -> Self {
        value as i32
    }
}

impl TryFrom<i32> for BlendMode {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Normal),
            1 => Ok(Self::Add),
            2 => Ok(Self::Subtract),
            _ => Err(format!("invalid BlendMode value: {}", value)),
        }
    }
}

/// 移动类型
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(into = "i32", try_from = "i32")]
#[repr(i32)]
pub enum MoveType {
    #[default]
    Fixed = 0,
    Random = 1,
    Approach = 2,
    Custom = 3,
}

impl From<MoveType> for i32 {
    fn from(value: MoveType) -> Self {
        value as i32
    }
}

impl TryFrom<i32> for MoveType {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Fixed),
            1 => Ok(Self::Random),
            2 => Ok(Self::Approach),
            3 => Ok(Self::Custom),
            _ => Err(format!("invalid MoveType value: {}", value)),
        }
    }
}

/// 移动速度
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(into = "i32", try_from = "i32")]
#[repr(i32)]
pub enum MoveSpeed {
    Slowest = 1,
    Slower = 2,
    Slow = 3,
    #[default]
    Fast = 4,
    Faster = 5,
    Fastest = 6,
}

impl From<MoveSpeed> for i32 {
    fn from(value: MoveSpeed) -> Self {
        value as i32
    }
}

impl TryFrom<i32> for MoveSpeed {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Slowest),
            2 => Ok(Self::Slower),
            3 => Ok(Self::Slow),
            4 => Ok(Self::Fast),
            5 => Ok(Self::Faster),
            6 => Ok(Self::Fastest),
            _ => Err(format!("invalid MoveSpeed value: {}", value)),
        }
    }
}

/// 移动频率
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(into = "i32", try_from = "i32")]
#[repr(i32)]
pub enum MoveFrequency {
    Lowest = 1,
    Lower = 2,
    Low = 3,
    #[default]
    High = 4,
    Higher = 5,
    Highest = 6,
}

impl From<MoveFrequency> for i32 {
    fn from(value: MoveFrequency) -> Self {
        value as i32
    }
}

impl TryFrom<i32> for MoveFrequency {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Lowest),
            2 => Ok(Self::Lower),
            3 => Ok(Self::Low),
            4 => Ok(Self::High),
            5 => Ok(Self::Higher),
            6 => Ok(Self::Highest),
            _ => Err(format!("invalid MoveFrequency value: {}", value)),
        }
    }
}

/// 事件触发
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(into = "i32", try_from = "i32")]
#[repr(i32)]
pub enum EventTrigger {
    #[default]
    ActionButton = 0,
    PlayerTouch = 1,
    EventTouch = 2,
    Autorun = 3,
    Parallel = 4,
}

impl From<EventTrigger> for i32 {
    fn from(value: EventTrigger) -> Self {
        value as i32
    }
}

impl TryFrom<i32> for EventTrigger {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::ActionButton),
            1 => Ok(Self::PlayerTouch),
            2 => Ok(Self::EventTouch),
            3 => Ok(Self::Autorun),
            4 => Ok(Self::Parallel),
            _ => Err(format!("invalid EventTrigger value: {}", value)),
        }
    }
}

/// 限制类型
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(into = "i32", try_from = "i32")]
#[repr(i32)]
pub enum Restriction {
    #[default]
    None = 0,
    NoMagic = 1,
    AttackEnemies = 2,
    AttackAllies = 3,
    NoMove = 4,
}

impl From<Restriction> for i32 {
    fn from(value: Restriction) -> Self {
        value as i32
    }
}

impl TryFrom<i32> for Restriction {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::NoMagic),
            2 => Ok(Self::AttackEnemies),
            3 => Ok(Self::AttackAllies),
            4 => Ok(Self::NoMove),
            _ => Err(format!("invalid Restriction value: {}", value)),
        }
    }
}

/// 职业位置
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(into = "i32", try_from = "i32")]
#[repr(i32)]
pub enum Position {
    #[default]
    Front = 0,
    Middle = 1,
    Rear = 2,
}

impl From<Position> for i32 {
    fn from(value: Position) -> Self {
        value as i32
    }
}

impl TryFrom<i32> for Position {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Front),
            1 => Ok(Self::Middle),
            2 => Ok(Self::Rear),
            _ => Err(format!("invalid Position value: {}", value)),
        }
    }
}

/// 防具类型
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(into = "i32", try_from = "i32")]
#[repr(i32)]
pub enum ArmorKind {
    #[default]
    Shield = 0,
    Helmet = 1,
    BodyArmor = 2,
    Accessory = 3,
}

impl From<ArmorKind> for i32 {
    fn from(value: ArmorKind) -> Self {
        value as i32
    }
}

impl TryFrom<i32> for ArmorKind {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Shield),
            1 => Ok(Self::Helmet),
            2 => Ok(Self::BodyArmor),
            3 => Ok(Self::Accessory),
            _ => Err(format!("invalid ArmorKind value: {}", value)),
        }
    }
}

/// 动画位置
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(into = "i32", try_from = "i32")]
#[repr(i32)]
pub enum AnimationPosition {
    Top = 0,
    #[default]
    Middle = 1,
    Bottom = 2,
    Screen = 3,
}

impl From<AnimationPosition> for i32 {
    fn from(value: AnimationPosition) -> Self {
        value as i32
    }
}

impl TryFrom<i32> for AnimationPosition {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Top),
            1 => Ok(Self::Middle),
            2 => Ok(Self::Bottom),
            3 => Ok(Self::Screen),
            _ => Err(format!("invalid AnimationPosition value: {}", value)),
        }
    }
}

/// 敌人行动类型
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(into = "i32", try_from = "i32")]
#[repr(i32)]
pub enum ActionKind {
    #[default]
    Basic = 0,
    Skill = 1,
}

impl From<ActionKind> for i32 {
    fn from(value: ActionKind) -> Self {
        value as i32
    }
}

impl TryFrom<i32> for ActionKind {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Basic),
            1 => Ok(Self::Skill),
            _ => Err(format!("invalid ActionKind value: {}", value)),
        }
    }
}

/// 敌人基础行动
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(into = "i32", try_from = "i32")]
#[repr(i32)]
pub enum BasicAction {
    #[default]
    Attack = 0,
    Defend = 1,
    Escape = 2,
    DoNothing = 3,
}

impl From<BasicAction> for i32 {
    fn from(value: BasicAction) -> Self {
        value as i32
    }
}

impl TryFrom<i32> for BasicAction {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Attack),
            1 => Ok(Self::Defend),
            2 => Ok(Self::Escape),
            3 => Ok(Self::DoNothing),
            _ => Err(format!("invalid BasicAction value: {}", value)),
        }
    }
}
