use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

mod ser;
mod de;

/// RPG Maker 数据格式中的值类型
///
/// 表示 Ruby Marshal 格式中所有可能的值类型，
/// 支持标准 serde 序列化/反序列化，可转换为 YAML、JSON 等格式。
#[derive(Debug, Clone)]
pub enum RpgValue {
    /// 空值，对应 Ruby 的 nil
    Nil,

    /// 布尔值
    Bool(bool),

    /// 整数值
    Integer(i32),

    /// 浮点数值
    Float(f64),

    /// 字符串，存储原始字节数据
    String(Vec<u8>),

    /// 符号，用于表示 Ruby 中的标识符
    Symbol(String),

    /// 数组
    Array(Vec<RpgValue>),

    /// 哈希表
    Hash(RpgHash),

    /// RPG 对象，包含类名和字段
    /// 序列化为 YAML 标签格式，如 `!RPG::Map`
    Object {
        /// 类名
        class: String,
        /// 字段映射，键为字段名（通常以 @ 开头）
        fields: RpgFields,
    },

    /// 用户数据，通过 _dump 方法序列化的对象
    Userdata {
        /// 类名
        class: String,
        /// 原始数据
        data: Vec<u8>,
    },

    /// 实例，带有实例变量的值
    Instance {
        /// 内部值
        value: Box<RpgValue>,
        /// 实例变量
        fields: RpgFields,
    },

    /// 正则表达式
    Regex {
        /// 正则表达式内容
        pattern: Vec<u8>,
        /// 标志位
        flags: u8,
    },

    /// Ruby 结构体
    Struct {
        /// 结构体类名
        class: String,
        /// 成员字段
        fields: RpgFields,
    },

    /// Ruby 类引用
    Class(String),

    /// Ruby 模块引用
    Module(String),

    /// 扩展模块的对象
    Extended {
        /// 扩展的模块名
        module: String,
        /// 被扩展的值
        value: Box<RpgValue>,
    },

    /// 用户定义的子类
    UserClass {
        /// 子类名
        class: String,
        /// 内部值
        value: Box<RpgValue>,
    },

    /// 用户 Marshal 对象
    UserMarshal {
        /// 类名
        class: String,
        /// 序列化后的值
        value: Box<RpgValue>,
    },

    /// 数据对象，用于 C 扩展
    Data {
        /// 类名
        class: String,
        /// 数据值
        value: Box<RpgValue>,
    },
}

/// RpgValue 的哈希表类型
pub type RpgHash = HashMap<RpgHashKey, RpgValue>;

/// RpgValue 的字段映射类型
pub type RpgFields = HashMap<String, RpgValue>;

/// 可用作哈希键的 RpgValue 包装类型
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RpgHashKey(pub RpgValue);

impl PartialEq for RpgValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RpgValue::Nil, RpgValue::Nil) => true,
            (RpgValue::Bool(a), RpgValue::Bool(b)) => a == b,
            (RpgValue::Integer(a), RpgValue::Integer(b)) => a == b,
            (RpgValue::Float(a), RpgValue::Float(b)) => a.to_bits() == b.to_bits(),
            (RpgValue::String(a), RpgValue::String(b)) => a == b,
            (RpgValue::Symbol(a), RpgValue::Symbol(b)) => a == b,
            (RpgValue::Array(a), RpgValue::Array(b)) => a == b,
            (RpgValue::Hash(a), RpgValue::Hash(b)) => a == b,
            (
                RpgValue::Object { class: c1, fields: f1 },
                RpgValue::Object { class: c2, fields: f2 },
            ) => c1 == c2 && f1 == f2,
            (
                RpgValue::Userdata { class: c1, data: d1 },
                RpgValue::Userdata { class: c2, data: d2 },
            ) => c1 == c2 && d1 == d2,
            (
                RpgValue::Instance { value: v1, fields: f1 },
                RpgValue::Instance { value: v2, fields: f2 },
            ) => v1 == v2 && f1 == f2,
            (
                RpgValue::Regex { pattern: p1, flags: f1 },
                RpgValue::Regex { pattern: p2, flags: f2 },
            ) => p1 == p2 && f1 == f2,
            (
                RpgValue::Struct { class: c1, fields: f1 },
                RpgValue::Struct { class: c2, fields: f2 },
            ) => c1 == c2 && f1 == f2,
            (RpgValue::Class(a), RpgValue::Class(b)) => a == b,
            (RpgValue::Module(a), RpgValue::Module(b)) => a == b,
            (
                RpgValue::Extended { module: m1, value: v1 },
                RpgValue::Extended { module: m2, value: v2 },
            ) => m1 == m2 && v1 == v2,
            (
                RpgValue::UserClass { class: c1, value: v1 },
                RpgValue::UserClass { class: c2, value: v2 },
            ) => c1 == c2 && v1 == v2,
            (
                RpgValue::UserMarshal { class: c1, value: v1 },
                RpgValue::UserMarshal { class: c2, value: v2 },
            ) => c1 == c2 && v1 == v2,
            (
                RpgValue::Data { class: c1, value: v1 },
                RpgValue::Data { class: c2, value: v2 },
            ) => c1 == c2 && v1 == v2,
            _ => false,
        }
    }
}

impl Eq for RpgValue {}

impl Hash for RpgValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        match self {
            RpgValue::Nil => {}
            RpgValue::Bool(b) => b.hash(state),
            RpgValue::Integer(i) => i.hash(state),
            RpgValue::Float(f) => f.to_bits().hash(state),
            RpgValue::String(s) => s.hash(state),
            RpgValue::Symbol(s) => s.hash(state),
            RpgValue::Array(a) => a.hash(state),
            RpgValue::Hash(h) => {
                let mut entries: Vec<_> = h.iter().collect();
                entries.sort_by(|a, b| format!("{:?}", a.0).cmp(&format!("{:?}", b.0)));
                for (k, v) in entries {
                    k.hash(state);
                    v.hash(state);
                }
            }
            RpgValue::Object { class, fields } => {
                class.hash(state);
                fields.hash(state);
            }
            RpgValue::Userdata { class, data } => {
                class.hash(state);
                data.hash(state);
            }
            RpgValue::Instance { value, fields } => {
                value.hash(state);
                fields.hash(state);
            }
            RpgValue::Regex { pattern, flags } => {
                pattern.hash(state);
                flags.hash(state);
            }
            RpgValue::Struct { class, fields } => {
                class.hash(state);
                fields.hash(state);
            }
            RpgValue::Class(c) => c.hash(state),
            RpgValue::Module(m) => m.hash(state),
            RpgValue::Extended { module, value } => {
                module.hash(state);
                value.hash(state);
            }
            RpgValue::UserClass { class, value } => {
                class.hash(state);
                value.hash(state);
            }
            RpgValue::UserMarshal { class, value } => {
                class.hash(state);
                value.hash(state);
            }
            RpgValue::Data { class, value } => {
                class.hash(state);
                value.hash(state);
            }
        }
    }
}

impl Default for RpgValue {
    fn default() -> Self {
        RpgValue::Nil
    }
}

impl RpgValue {
    /// 创建一个 Nil 值
    pub fn nil() -> Self {
        RpgValue::Nil
    }

    /// 创建一个布尔值
    pub fn bool(v: bool) -> Self {
        RpgValue::Bool(v)
    }

    /// 创建一个整数值
    pub fn integer(v: i32) -> Self {
        RpgValue::Integer(v)
    }

    /// 创建一个浮点数值
    pub fn float(v: f64) -> Self {
        RpgValue::Float(v)
    }

    /// 创建一个字符串值
    pub fn string(v: impl Into<Vec<u8>>) -> Self {
        RpgValue::String(v.into())
    }

    /// 创建一个符号值
    pub fn symbol(v: impl Into<String>) -> Self {
        RpgValue::Symbol(v.into())
    }

    /// 创建一个数组值
    pub fn array(v: Vec<RpgValue>) -> Self {
        RpgValue::Array(v)
    }

    /// 创建一个哈希值
    pub fn hash(v: RpgHash) -> Self {
        RpgValue::Hash(v)
    }

    /// 创建一个对象值
    pub fn object(class: impl Into<String>, fields: RpgFields) -> Self {
        RpgValue::Object {
            class: class.into(),
            fields,
        }
    }

    /// 检查是否为 Nil
    pub fn is_nil(&self) -> bool {
        matches!(self, RpgValue::Nil)
    }

    /// 检查是否为布尔值
    pub fn is_bool(&self) -> bool {
        matches!(self, RpgValue::Bool(_))
    }

    /// 检查是否为整数
    pub fn is_integer(&self) -> bool {
        matches!(self, RpgValue::Integer(_))
    }

    /// 检查是否为浮点数
    pub fn is_float(&self) -> bool {
        matches!(self, RpgValue::Float(_))
    }

    /// 检查是否为字符串
    pub fn is_string(&self) -> bool {
        matches!(self, RpgValue::String(_))
    }

    /// 检查是否为符号
    pub fn is_symbol(&self) -> bool {
        matches!(self, RpgValue::Symbol(_))
    }

    /// 检查是否为数组
    pub fn is_array(&self) -> bool {
        matches!(self, RpgValue::Array(_))
    }

    /// 检查是否为哈希
    pub fn is_hash(&self) -> bool {
        matches!(self, RpgValue::Hash(_))
    }

    /// 检查是否为对象
    pub fn is_object(&self) -> bool {
        matches!(self, RpgValue::Object { .. })
    }
}
