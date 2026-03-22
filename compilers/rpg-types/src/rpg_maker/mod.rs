//! RPG Maker 游戏数据类型
//!
//! 提供 RPG Maker XP/VX/VX Ace/MV/MZ 游戏中常见的数据类型定义，
//! 以及这些类型与 RubyValue 之间的转换功能。

use crate::{Result, ruby_value::RubyValue};

mod actor;
pub use actor::RpgActor;

#[derive(Debug, Clone)]
pub struct ActorTable {
    pub actors: Vec<Option<RpgActor>>,
}

/// RPG Maker 游戏数据
///
/// 表示 RPG Maker 游戏中的各种数据类型，如 Actors、Items、Maps 等。
#[derive(Debug, Clone)]
pub enum RpgMakerData {
    /// 角色数据数组
    Actors(Box<ActorTable>),
    /// 类型未知的一般表
    Custom(Box<RubyValue>),
}

impl RpgMakerData {
    /// 从 RubyValue 创建 RpgMakerData
    pub fn from_ruby_value(value: &RubyValue, data_type: &str) -> Result<Self> {
        match data_type {
            "Actors" => Self::actors_from_ruby_value(value),
            _ => Err(crate::errors::RpgError::decode("RpgMakerData", &format!("unsupported data type: {}", data_type))),
        }
    }

    /// 转换为 RubyValue
    pub fn to_ruby_value(&self) -> RubyValue {
        match self {
            RpgMakerData::Actors(actors) => Self::actors_to_ruby_value(&actors.actors),
            RpgMakerData::Custom(value) => *value.clone(),
        }
    }

    /// 从 RubyValue 创建 Actors 数据
    fn actors_from_ruby_value(value: &RubyValue) -> Result<Self> {
        if let RubyValue::Array(arr) = value {
            let mut actors = Vec::with_capacity(arr.len());
            for item in arr {
                match item {
                    RubyValue::Nil => actors.push(None),
                    _ => {
                        let actor = RpgActor::from_ruby_value(item)?;
                        actors.push(Some(actor));
                    }
                }
            }
            Ok(RpgMakerData::Actors(Box::new(ActorTable { actors })))
        }
        else {
            Err(crate::errors::RpgError::decode("RpgMakerData", "expected Array for Actors data"))
        }
    }

    /// 将 Actors 数据转换为 RubyValue
    fn actors_to_ruby_value(actors: &[Option<RpgActor>]) -> RubyValue {
        let arr: Vec<RubyValue> = actors
            .iter()
            .map(|opt| match opt {
                None => RubyValue::Nil,
                Some(actor) => actor.to_ruby_value(),
            })
            .collect();
        RubyValue::Array(arr)
    }
}
