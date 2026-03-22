use std::hash::{Hash, Hasher};

mod de;
mod ser;

/// RPG Maker 数据格式中的值类型
///
/// 表示 Ruby Marshal 格式中所有可能的值类型，
/// 支持标准 serde 序列化/反序列化，可转换为 YAML、JSON 等格式。
#[derive(Debug, Clone)]
pub enum RubyValue {
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
    Array(Vec<RubyValue>),

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
        value: Box<RubyValue>,
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
        value: Box<RubyValue>,
    },

    /// 用户定义的子类
    UserClass {
        /// 子类名
        class: String,
        /// 内部值
        value: Box<RubyValue>,
    },

    /// 用户 Marshal 对象
    UserMarshal {
        /// 类名
        class: String,
        /// 序列化后的值
        value: Box<RubyValue>,
    },

    /// 数据对象，用于 C 扩展
    Data {
        /// 类名
        class: String,
        /// 数据值
        value: Box<RubyValue>,
    },
}

/// RpgValue 的哈希表类型
pub type RpgHash = std::collections::HashMap<RpgHashKey, RubyValue>;

/// RpgValue 的字段映射类型
pub type RpgFields = std::collections::HashMap<String, RubyValue>;

/// 可用作哈希键的 RpgValue 包装类型
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RpgHashKey(pub RubyValue);

impl PartialEq for RubyValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RubyValue::Nil, RubyValue::Nil) => true,
            (RubyValue::Bool(a), RubyValue::Bool(b)) => a == b,
            (RubyValue::Integer(a), RubyValue::Integer(b)) => a == b,
            (RubyValue::Float(a), RubyValue::Float(b)) => a.to_bits() == b.to_bits(),
            (RubyValue::String(a), RubyValue::String(b)) => a == b,
            (RubyValue::Symbol(a), RubyValue::Symbol(b)) => a == b,
            (RubyValue::Array(a), RubyValue::Array(b)) => a == b,
            (RubyValue::Hash(a), RubyValue::Hash(b)) => a == b,
            (RubyValue::Object { class: c1, fields: f1 }, RubyValue::Object { class: c2, fields: f2 }) => c1 == c2 && f1 == f2,
            (RubyValue::Userdata { class: c1, data: d1 }, RubyValue::Userdata { class: c2, data: d2 }) => c1 == c2 && d1 == d2,
            (RubyValue::Instance { value: v1, fields: f1 }, RubyValue::Instance { value: v2, fields: f2 }) => {
                v1 == v2 && f1 == f2
            }
            (RubyValue::Regex { pattern: p1, flags: f1 }, RubyValue::Regex { pattern: p2, flags: f2 }) => p1 == p2 && f1 == f2,
            (RubyValue::Struct { class: c1, fields: f1 }, RubyValue::Struct { class: c2, fields: f2 }) => c1 == c2 && f1 == f2,
            (RubyValue::Class(a), RubyValue::Class(b)) => a == b,
            (RubyValue::Module(a), RubyValue::Module(b)) => a == b,
            (RubyValue::Extended { module: m1, value: v1 }, RubyValue::Extended { module: m2, value: v2 }) => {
                m1 == m2 && v1 == v2
            }
            (RubyValue::UserClass { class: c1, value: v1 }, RubyValue::UserClass { class: c2, value: v2 }) => {
                c1 == c2 && v1 == v2
            }
            (RubyValue::UserMarshal { class: c1, value: v1 }, RubyValue::UserMarshal { class: c2, value: v2 }) => {
                c1 == c2 && v1 == v2
            }
            (RubyValue::Data { class: c1, value: v1 }, RubyValue::Data { class: c2, value: v2 }) => c1 == c2 && v1 == v2,
            _ => false,
        }
    }
}

impl Eq for RubyValue {}

impl Hash for RubyValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        match self {
            RubyValue::Nil => {}
            RubyValue::Bool(b) => b.hash(state),
            RubyValue::Integer(i) => i.hash(state),
            RubyValue::Float(f) => f.to_bits().hash(state),
            RubyValue::String(s) => s.hash(state),
            RubyValue::Symbol(s) => s.hash(state),
            RubyValue::Array(a) => a.hash(state),
            RubyValue::Hash(h) => {
                let mut entries: Vec<_> = h.iter().collect();
                entries.sort_by(|a, b| format!("{:?}", a.0).cmp(&format!("{:?}", b.0)));
                for (k, v) in entries {
                    k.hash(state);
                    v.hash(state);
                }
            }
            RubyValue::Object { class, fields } => {
                class.hash(state);
                hash_fields(fields, state);
            }
            RubyValue::Userdata { class, data } => {
                class.hash(state);
                data.hash(state);
            }
            RubyValue::Instance { value, fields } => {
                value.hash(state);
                hash_fields(fields, state);
            }
            RubyValue::Regex { pattern, flags } => {
                pattern.hash(state);
                flags.hash(state);
            }
            RubyValue::Struct { class, fields } => {
                class.hash(state);
                hash_fields(fields, state);
            }
            RubyValue::Class(c) => c.hash(state),
            RubyValue::Module(m) => m.hash(state),
            RubyValue::Extended { module, value } => {
                module.hash(state);
                value.hash(state);
            }
            RubyValue::UserClass { class, value } => {
                class.hash(state);
                value.hash(state);
            }
            RubyValue::UserMarshal { class, value } => {
                class.hash(state);
                value.hash(state);
            }
            RubyValue::Data { class, value } => {
                class.hash(state);
                value.hash(state);
            }
        }
    }
}

fn hash_fields<H: Hasher>(fields: &RpgFields, state: &mut H) {
    let mut entries: Vec<_> = fields.iter().collect();
    entries.sort_by_key(|(k, _)| *k);
    for (k, v) in entries {
        k.hash(state);
        v.hash(state);
    }
}

impl Default for RubyValue {
    fn default() -> Self {
        RubyValue::Nil
    }
}

impl RubyValue {
    /// 创建一个 Nil 值
    pub fn nil() -> Self {
        RubyValue::Nil
    }

    /// 创建一个布尔值
    pub fn bool(v: bool) -> Self {
        RubyValue::Bool(v)
    }

    /// 创建一个整数值
    pub fn integer(v: i32) -> Self {
        RubyValue::Integer(v)
    }

    /// 创建一个浮点数值
    pub fn float(v: f64) -> Self {
        RubyValue::Float(v)
    }

    /// 创建一个字符串值
    pub fn string(v: impl Into<Vec<u8>>) -> Self {
        RubyValue::String(v.into())
    }

    /// 创建一个符号值
    pub fn symbol(v: impl Into<String>) -> Self {
        RubyValue::Symbol(v.into())
    }

    /// 创建一个数组值
    pub fn array(v: Vec<RubyValue>) -> Self {
        RubyValue::Array(v)
    }

    /// 创建一个哈希值
    pub fn hash(v: RpgHash) -> Self {
        RubyValue::Hash(v)
    }

    /// 创建一个对象值
    pub fn object(class: impl Into<String>, fields: RpgFields) -> Self {
        RubyValue::Object { class: class.into(), fields }
    }

    /// 检查是否为 Nil
    pub fn is_nil(&self) -> bool {
        matches!(self, RubyValue::Nil)
    }

    /// 检查是否为布尔值
    pub fn is_bool(&self) -> bool {
        matches!(self, RubyValue::Bool(_))
    }

    /// 检查是否为整数
    pub fn is_integer(&self) -> bool {
        matches!(self, RubyValue::Integer(_))
    }

    /// 检查是否为浮点数
    pub fn is_float(&self) -> bool {
        matches!(self, RubyValue::Float(_))
    }

    /// 检查是否为字符串
    pub fn is_string(&self) -> bool {
        matches!(self, RubyValue::String(_))
    }

    /// 检查是否为符号
    pub fn is_symbol(&self) -> bool {
        matches!(self, RubyValue::Symbol(_))
    }

    /// 检查是否为数组
    pub fn is_array(&self) -> bool {
        matches!(self, RubyValue::Array(_))
    }

    /// 检查是否为哈希
    pub fn is_hash(&self) -> bool {
        matches!(self, RubyValue::Hash(_))
    }

    /// 检查是否为对象
    pub fn is_object(&self) -> bool {
        matches!(self, RubyValue::Object { .. })
    }

    /// 将 RpgValue 转换为字符串表示
    pub fn to_string_lossy(&self) -> String {
        match self {
            RubyValue::Nil => "nil".to_string(),
            RubyValue::Bool(b) => b.to_string(),
            RubyValue::Integer(i) => i.to_string(),
            RubyValue::Float(f) => f.to_string(),
            RubyValue::String(s) => String::from_utf8_lossy(s).to_string(),
            RubyValue::Symbol(s) => s.clone(),
            RubyValue::Array(_) => "[...]".to_string(),
            RubyValue::Hash(_) => "{...}".to_string(),
            RubyValue::Object { class, .. } => format!("<Object: {}>", class),
            RubyValue::Userdata { class, .. } => format!("<Userdata: {}>", class),
            RubyValue::Instance { .. } => "<Instance>".to_string(),
            RubyValue::Regex { pattern, .. } => String::from_utf8_lossy(pattern).to_string(),
            RubyValue::Struct { class, .. } => format!("<Struct: {}>", class),
            RubyValue::Class(c) => c.clone(),
            RubyValue::Module(m) => m.clone(),
            RubyValue::Extended { module, .. } => format!("<Extended: {}>", module),
            RubyValue::UserClass { class, .. } => format!("<UserClass: {}>", class),
            RubyValue::UserMarshal { class, .. } => format!("<UserMarshal: {}>", class),
            RubyValue::Data { class, .. } => format!("<Data: {}>", class),
        }
    }
}
