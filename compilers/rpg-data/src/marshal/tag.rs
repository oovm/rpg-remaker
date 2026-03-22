/// Marshal 格式类型标记
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tag {
    /// 空值
    Nil = b'0',
    /// 布尔真
    True = b'T',
    /// 布尔假
    False = b'F',
    /// 整数
    Integer = b'i',
    /// 大整数 (Bignum)
    Bignum = b'l',
    /// 浮点数
    Float = b'f',
    /// 字符串
    String = b'"',
    /// 数组
    Array = b'[',
    /// 哈希
    Hash = b'{',
    /// 带默认值的哈希
    HashDefault = b'}',
    /// 符号
    Symbol = b':',
    /// 符号链接
    Symlink = b';',
    /// 实例
    Instance = b'I',
    /// 正则表达式
    RawRegexp = b'/',
    /// 类引用
    ClassRef = b'c',
    /// 模块引用
    ModuleRef = b'm',
    /// 对象
    Object = b'o',
    /// 对象链接
    ObjectLink = b'@',
    /// 用户定义数据
    UserDef = b'u',
    /// 结构体
    Struct = b'S',
    /// 用户类
    UserClass = b'C',
    /// 扩展模块
    Extended = b'e',
    /// 用户 Marshal
    UserMarshal = b'U',
    /// 数据
    Data = b'd',
}

impl Tag {
    /// 从字节转换为 Tag
    pub fn from_u8(value: u8) -> Option<Tag> {
        match value {
            b'0' => Some(Tag::Nil),
            b'T' => Some(Tag::True),
            b'F' => Some(Tag::False),
            b'i' => Some(Tag::Integer),
            b'l' => Some(Tag::Bignum),
            b'f' => Some(Tag::Float),
            b'"' => Some(Tag::String),
            b'[' => Some(Tag::Array),
            b'{' => Some(Tag::Hash),
            b'}' => Some(Tag::HashDefault),
            b':' => Some(Tag::Symbol),
            b';' => Some(Tag::Symlink),
            b'I' => Some(Tag::Instance),
            b'/' => Some(Tag::RawRegexp),
            b'c' => Some(Tag::ClassRef),
            b'm' => Some(Tag::ModuleRef),
            b'o' => Some(Tag::Object),
            b'@' => Some(Tag::ObjectLink),
            b'u' => Some(Tag::UserDef),
            b'S' => Some(Tag::Struct),
            b'C' => Some(Tag::UserClass),
            b'e' => Some(Tag::Extended),
            b'U' => Some(Tag::UserMarshal),
            b'd' => Some(Tag::Data),
            _ => None,
        }
    }

    /// 检查是否可以被对象链接引用
    pub fn is_object_link_referenceable(self) -> bool {
        !matches!(
            self,
            Self::Nil
                | Self::True
                | Self::False
                | Self::Integer
                | Self::Bignum
                | Self::Symbol
                | Self::Symlink
                | Self::ObjectLink
        )
    }
}

impl From<Tag> for u8 {
    fn from(value: Tag) -> Self {
        value as _
    }
}
