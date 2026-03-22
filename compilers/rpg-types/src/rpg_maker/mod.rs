//! RPG Maker 游戏数据类型
//!
//! 提供 RPG Maker XP/VX/VX Ace/MV/MZ 游戏中常见的数据类型定义。
//! 这些类型使用 serde 进行序列化/反序列化，支持 YAML 和 JSON 格式。

mod actor;
mod animation;
mod armor;
mod class_;
mod enemy;
mod event;
mod item;
mod map;
mod mapinfo;
mod script;
mod shared;
mod skill;
mod state;
mod system;
mod tileset;
mod troop;
mod weapon;

pub use actor::RpgActor;
pub use animation::{RpgAnimation, RpgAnimationFrame, RpgAnimationTiming, AnimationFlashScope, AnimationCondition};
pub use armor::RpgArmor;
pub use class_::{RpgClass, RpgLearning};
pub use enemy::{RpgEnemy, RpgEnemyAction};
pub use event::{RpgEvent, RpgEventPage, RpgEventCondition, RpgEventGraphic, RpgEventCommand, RpgCommonEvent, RpgMoveRoute, RpgMoveCommand};
pub use item::RpgItem;
pub use map::RpgMap;
pub use mapinfo::RpgMapInfo;
pub use script::RpgScript;
pub use shared::{
    AudioFile, Scope, Occasion, ParameterType, BlendMode, MoveType, MoveSpeed, MoveFrequency,
    EventTrigger, Restriction, Position, ArmorKind, AnimationPosition, ActionKind, BasicAction,
};
pub use skill::RpgSkill;
pub use state::RpgState;
pub use system::{RpgSystem, RpgWords};
pub use tileset::RpgTileset;
pub use troop::{RpgTroop, RpgTroopMember, RpgTroopPage, RpgTroopPageCondition};
pub use weapon::RpgWeapon;
