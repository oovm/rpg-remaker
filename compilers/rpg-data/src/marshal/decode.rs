use super::tag::Tag;
use rpg_types::{RpgFields, RpgHash, RpgHashKey, RubyValue};

/// Marshal 解码错误
#[derive(Debug, Clone)]
pub struct DecodeError {
    /// 错误类型
    pub kind: DecodeErrorKind,
    /// 错误位置
    pub position: Option<usize>,
}

/// Marshal 解码错误类型
#[derive(Debug, Clone)]
pub enum DecodeErrorKind {
    /// 数据结束
    Eof,
    /// 版本错误
    VersionError([u8; 2]),
    /// 无效标签
    InvalidTag(u8),
    /// 无效 UTF-8
    InvalidUtf8,
    /// 未解析的符号链接
    UnresolvedSymlink(usize),
    /// 未解析的对象链接
    UnresolvedObjectLink(usize),
    /// 循环引用
    CircularReference,
    /// 负长度
    NegativeLength(i32),
    /// 解析浮点数失败
    ParseFloatError,
    /// 其他错误
    Other(String),
}

impl std::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            DecodeErrorKind::Eof => write!(f, "unexpected end of data"),
            DecodeErrorKind::VersionError(v) => write!(f, "invalid marshal version: {:?}, expected [4, 8]", v),
            DecodeErrorKind::InvalidTag(t) => write!(f, "invalid tag: {:02X}", t),
            DecodeErrorKind::InvalidUtf8 => write!(f, "invalid UTF-8 sequence"),
            DecodeErrorKind::UnresolvedSymlink(i) => write!(f, "unresolved symlink: {}", i),
            DecodeErrorKind::UnresolvedObjectLink(i) => write!(f, "unresolved object link: {}", i),
            DecodeErrorKind::CircularReference => write!(f, "circular reference detected"),
            DecodeErrorKind::NegativeLength(n) => write!(f, "negative length: {}", n),
            DecodeErrorKind::ParseFloatError => write!(f, "failed to parse float"),
            DecodeErrorKind::Other(s) => write!(f, "{}", s),
        }
    }
}

impl std::error::Error for DecodeError {}

/// Marshal 解码器
pub struct Decoder<'a> {
    input: &'a [u8],
    position: usize,
    symbol_table: Vec<String>,
    object_table: Vec<usize>,
}

impl<'a> Decoder<'a> {
    /// 创建新的解码器
    pub fn new(input: &'a [u8]) -> Result<Self, DecodeError> {
        if input.len() < 2 {
            return Err(DecodeError { kind: DecodeErrorKind::Eof, position: None });
        }

        if input[0] != 4 || input[1] != 8 {
            return Err(DecodeError { kind: DecodeErrorKind::VersionError([input[0], input[1]]), position: None });
        }

        Ok(Self { input, position: 2, symbol_table: Vec::new(), object_table: Vec::new() })
    }

    /// 解码为 RpgValue
    pub fn decode(&mut self) -> Result<RubyValue, DecodeError> {
        let b = self.read_byte()?;

        // 首先检查是否是有效的标签
        if let Some(tag) = Tag::from_u8(b) {
            // 如果是标签，按标签处理
            if tag.is_object_link_referenceable() {
                self.object_table.push(self.position);
            }
            return self.decode_value(tag);
        }

        // 如果不是标签，检查是否是直接编码的整数
        if b == 0x00 {
            // 直接编码的整数 0
            return Ok(RubyValue::Integer(0));
        }
        else if b >= 0x05 && b <= 0x7F {
            // 正整数: value = b - 5
            let value = (b - 5) as i32;
            return Ok(RubyValue::Integer(value));
        }
        else if b >= 0x80 {
            // 负整数: value = b - 256 - 5
            let value = (b as i8 - 5) as i32;
            return Ok(RubyValue::Integer(value));
        }

        // 既不是标签也不是直接编码的整数，返回错误
        Err(DecodeError { kind: DecodeErrorKind::InvalidTag(b), position: Some(self.position - 1) })
    }

    fn decode_value(&mut self, tag: Tag) -> Result<RubyValue, DecodeError> {
        match tag {
            Tag::Nil => Ok(RubyValue::Nil),
            Tag::True => Ok(RubyValue::Bool(true)),
            Tag::False => Ok(RubyValue::Bool(false)),
            Tag::Integer => {
                let i = self.read_packed_int()?;
                Ok(RubyValue::Integer(i))
            }
            Tag::Bignum => {
                let sign = self.read_byte()?;
                let len = self.read_usize()?;
                let mut result: i64 = 0;
                for i in 0..len {
                    let byte = self.read_byte()? as u64;
                    result |= (byte as i64) << (8 * i);
                }
                if sign == b'-' {
                    result = -result;
                }
                if result >= i32::MIN as i64 && result <= i32::MAX as i64 {
                    Ok(RubyValue::Integer(result as i32))
                }
                else {
                    Ok(RubyValue::Integer(result as i32))
                }
            }
            Tag::Float => {
                let f = self.read_float()?;
                Ok(RubyValue::Float(f))
            }
            Tag::String => {
                let data = self.read_bytes()?;
                Ok(RubyValue::String(data))
            }
            Tag::Symbol => {
                let sym = self.read_symbol()?;
                self.symbol_table.push(sym.clone());
                Ok(RubyValue::Symbol(sym))
            }
            Tag::Symlink => {
                let index = self.read_packed_int()? as usize;
                self.symbol_table.get(index).cloned().map(RubyValue::Symbol).ok_or_else(|| DecodeError {
                    kind: DecodeErrorKind::UnresolvedSymlink(index),
                    position: Some(self.position),
                })
            }
            Tag::Array => {
                let len = self.read_usize()?;
                let mut arr = Vec::with_capacity(len);
                for _ in 0..len {
                    arr.push(self.decode()?);
                }
                Ok(RubyValue::Array(arr))
            }
            Tag::Hash => {
                let len = self.read_usize()?;
                let mut hash = RpgHash::new();
                for _ in 0..len {
                    let key = self.decode()?;
                    let value = self.decode()?;
                    hash.insert(RpgHashKey(key), value);
                }
                Ok(RubyValue::Hash(hash))
            }
            Tag::HashDefault => {
                let len = self.read_usize()?;
                let mut hash = RpgHash::new();
                for _ in 0..len {
                    let key = self.decode()?;
                    let value = self.decode()?;
                    hash.insert(RpgHashKey(key), value);
                }
                let _default = self.decode()?;
                Ok(RubyValue::Hash(hash))
            }
            Tag::Object => {
                let class = self.read_symbol_either()?;
                let len = self.read_usize()?;
                let mut fields = RpgFields::new();
                for _ in 0..len {
                    let key = self.read_symbol_either()?;
                    let value = self.decode()?;
                    fields.insert(key, value);
                }
                Ok(RubyValue::Object { class, fields })
            }
            Tag::Instance => {
                let value = self.decode()?;
                let len = self.read_usize()?;
                let mut fields = RpgFields::new();
                for _ in 0..len {
                    let key = self.read_symbol_either()?;
                    let value = self.decode()?;
                    fields.insert(key, value);
                }
                Ok(RubyValue::Instance { value: Box::new(value), fields })
            }
            Tag::UserDef => {
                let class = self.read_symbol_either()?;
                let data = self.read_bytes()?;
                Ok(RubyValue::Userdata { class, data })
            }
            Tag::Struct => {
                let class = self.read_symbol_either()?;
                let len = self.read_usize()?;
                let mut fields = RpgFields::new();
                for _ in 0..len {
                    let key = self.read_symbol_either()?;
                    let value = self.decode()?;
                    fields.insert(key, value);
                }
                Ok(RubyValue::Struct { class, fields })
            }
            Tag::ClassRef => {
                let name = self.read_string()?;
                Ok(RubyValue::Class(name))
            }
            Tag::ModuleRef => {
                let name = self.read_string()?;
                Ok(RubyValue::Module(name))
            }
            Tag::RawRegexp => {
                let pattern = self.read_bytes()?;
                let flags = self.read_byte()?;
                Ok(RubyValue::Regex { pattern, flags })
            }
            Tag::Extended => {
                let module = self.read_symbol_either()?;
                let value = self.decode()?;
                Ok(RubyValue::Extended { module, value: Box::new(value) })
            }
            Tag::UserClass => {
                let class = self.read_symbol_either()?;
                let value = self.decode()?;
                Ok(RubyValue::UserClass { class, value: Box::new(value) })
            }
            Tag::UserMarshal => {
                let class = self.read_symbol_either()?;
                let value = self.decode()?;
                Ok(RubyValue::UserMarshal { class, value: Box::new(value) })
            }
            Tag::Data => {
                let class = self.read_symbol_either()?;
                let value = self.decode()?;
                Ok(RubyValue::Data { class, value: Box::new(value) })
            }
            Tag::ObjectLink => {
                let index = self.read_usize()?;
                let target_pos = self.object_table.get(index).copied().ok_or_else(|| DecodeError {
                    kind: DecodeErrorKind::UnresolvedObjectLink(index),
                    position: Some(self.position),
                })?;

                let current_pos = self.position;
                self.position = target_pos;
                let value = self.decode()?;
                self.position = current_pos;
                Ok(value)
            }
        }
    }

    fn read_byte(&mut self) -> Result<u8, DecodeError> {
        if self.position >= self.input.len() {
            return Err(DecodeError { kind: DecodeErrorKind::Eof, position: Some(self.position) });
        }
        let b = self.input[self.position];
        self.position += 1;
        Ok(b)
    }

    fn read_tag(&mut self) -> Result<Tag, DecodeError> {
        let b = self.read_byte()?;
        let tag = Tag::from_u8(b);
        if tag.is_none() {
            eprintln!(
                "[DEBUG] Invalid tag 0x{:02X} at position {}, context: {:02X?}",
                b,
                self.position - 1,
                &self.input[self.position.saturating_sub(10)..self.position.min(self.input.len())]
            );
        }
        tag.ok_or_else(|| DecodeError { kind: DecodeErrorKind::InvalidTag(b), position: Some(self.position - 1) })
    }

    fn read_packed_int(&mut self) -> Result<i32, DecodeError> {
        let c = self.read_byte()? as i8;

        match c {
            0 => Ok(0),
            5..=127 => Ok((c - 5) as i32),
            -128..=-5 => Ok((c + 5) as i32),
            1..=4 => {
                let mut x: i32 = 0;
                for i in 0..c as usize {
                    let n = self.read_byte()? as i32;
                    x |= n << (8 * i);
                }
                Ok(x)
            }
            -4..=-1 => {
                let mut x: i32 = -1;
                for i in 0..(-c) as usize {
                    let mask = !(0xFF << (8 * i));
                    let b = self.read_byte()? as i32;
                    let b = b << (8 * i);
                    x = (x & mask) | b;
                }
                Ok(x)
            }
        }
    }

    fn read_usize(&mut self) -> Result<usize, DecodeError> {
        let raw = self.read_packed_int()?;
        if raw < 0 {
            return Err(DecodeError { kind: DecodeErrorKind::NegativeLength(raw), position: Some(self.position) });
        }
        Ok(raw as usize)
    }

    fn read_float(&mut self) -> Result<f64, DecodeError> {
        let bytes = self.read_bytes()?;
        let s = String::from_utf8_lossy(&bytes);

        if let Some(pos) = s.find('\0') {
            let str_part = &s[..pos];
            let float: f64 = str_part
                .parse()
                .map_err(|_| DecodeError { kind: DecodeErrorKind::ParseFloatError, position: Some(self.position) })?;
            Ok(float)
        }
        else {
            s.parse().map_err(|_| DecodeError { kind: DecodeErrorKind::ParseFloatError, position: Some(self.position) })
        }
    }

    fn read_bytes(&mut self) -> Result<Vec<u8>, DecodeError> {
        let len = self.read_usize()?;
        if self.position + len > self.input.len() {
            return Err(DecodeError { kind: DecodeErrorKind::Eof, position: Some(self.position) });
        }
        let bytes = self.input[self.position..self.position + len].to_vec();
        self.position += len;
        Ok(bytes)
    }

    fn read_string(&mut self) -> Result<String, DecodeError> {
        let bytes = self.read_bytes()?;
        String::from_utf8(bytes).map_err(|_| DecodeError { kind: DecodeErrorKind::InvalidUtf8, position: Some(self.position) })
    }

    fn read_symbol(&mut self) -> Result<String, DecodeError> {
        self.read_string()
    }

    fn read_symbol_either(&mut self) -> Result<String, DecodeError> {
        let tag = self.read_tag()?;
        match tag {
            Tag::Symbol => {
                let sym = self.read_symbol()?;
                self.symbol_table.push(sym.clone());
                Ok(sym)
            }
            Tag::Symlink => {
                let index = self.read_packed_int()? as usize;
                self.symbol_table.get(index).cloned().ok_or_else(|| DecodeError {
                    kind: DecodeErrorKind::UnresolvedSymlink(index),
                    position: Some(self.position),
                })
            }
            _ => Err(DecodeError {
                kind: DecodeErrorKind::Other(format!(
                    "expected symbol or symlink, got {:?} (0x{:02X}) at position {}",
                    tag,
                    tag as u8,
                    self.position - 1
                )),
                position: Some(self.position - 1),
            }),
        }
    }
}

/// 从字节解码 RpgValue
/// 自动检测并处理 gzip 压缩数据
pub fn from_bytes(input: &[u8]) -> Result<RubyValue, DecodeError> {
    if input.len() < 2 {
        return Err(DecodeError { kind: DecodeErrorKind::Eof, position: None });
    }

    // 检测 gzip 魔数 (0x1f 0x8b)
    if input[0] == 0x1f && input[1] == 0x8b {
        let mut decoder = flate2::read::GzDecoder::new(input);
        let mut decompressed = Vec::new();
        std::io::Read::read_to_end(&mut decoder, &mut decompressed).map_err(|e| DecodeError {
            kind: DecodeErrorKind::Other(format!("gzip decompression failed: {}", e)),
            position: None,
        })?;
        let mut decoder = Decoder::new(&decompressed)?;
        return decoder.decode();
    }

    let mut decoder = Decoder::new(input)?;
    decoder.decode()
}
