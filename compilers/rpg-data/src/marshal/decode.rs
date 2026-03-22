use super::tag::Tag;
use rpg_types::{RpgFields, RpgHash, RpgHashKey, RpgValue};

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
    pub fn decode(&mut self) -> Result<RpgValue, DecodeError> {
        let tag = self.read_tag()?;

        if tag.is_object_link_referenceable() {
            self.object_table.push(self.position);
        }

        self.decode_value(tag)
    }

    fn decode_value(&mut self, tag: Tag) -> Result<RpgValue, DecodeError> {
        match tag {
            Tag::Nil => Ok(RpgValue::Nil),
            Tag::True => Ok(RpgValue::Bool(true)),
            Tag::False => Ok(RpgValue::Bool(false)),
            Tag::Integer => {
                let i = self.read_packed_int()?;
                Ok(RpgValue::Integer(i))
            }
            Tag::Float => {
                let f = self.read_float()?;
                Ok(RpgValue::Float(f))
            }
            Tag::String => {
                let data = self.read_bytes()?;
                Ok(RpgValue::String(data))
            }
            Tag::Symbol => {
                let sym = self.read_symbol()?;
                self.symbol_table.push(sym.clone());
                Ok(RpgValue::Symbol(sym))
            }
            Tag::Symlink => {
                let index = self.read_packed_int()? as usize;
                self.symbol_table.get(index).cloned().map(RpgValue::Symbol).ok_or_else(|| DecodeError {
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
                Ok(RpgValue::Array(arr))
            }
            Tag::Hash => {
                let len = self.read_usize()?;
                let mut hash = RpgHash::new();
                for _ in 0..len {
                    let key = self.decode()?;
                    let value = self.decode()?;
                    hash.insert(RpgHashKey(key), value);
                }
                Ok(RpgValue::Hash(hash))
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
                Ok(RpgValue::Hash(hash))
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
                Ok(RpgValue::Object { class, fields })
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
                Ok(RpgValue::Instance { value: Box::new(value), fields })
            }
            Tag::UserDef => {
                let class = self.read_symbol_either()?;
                let data = self.read_bytes()?;
                Ok(RpgValue::Userdata { class, data })
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
                Ok(RpgValue::Struct { class, fields })
            }
            Tag::ClassRef => {
                let name = self.read_string()?;
                Ok(RpgValue::Class(name))
            }
            Tag::ModuleRef => {
                let name = self.read_string()?;
                Ok(RpgValue::Module(name))
            }
            Tag::RawRegexp => {
                let pattern = self.read_bytes()?;
                let flags = self.read_byte()?;
                Ok(RpgValue::Regex { pattern, flags })
            }
            Tag::Extended => {
                let module = self.read_symbol_either()?;
                let value = self.decode()?;
                Ok(RpgValue::Extended { module, value: Box::new(value) })
            }
            Tag::UserClass => {
                let class = self.read_symbol_either()?;
                let value = self.decode()?;
                Ok(RpgValue::UserClass { class, value: Box::new(value) })
            }
            Tag::UserMarshal => {
                let class = self.read_symbol_either()?;
                let value = self.decode()?;
                Ok(RpgValue::UserMarshal { class, value: Box::new(value) })
            }
            Tag::Data => {
                let class = self.read_symbol_either()?;
                let value = self.decode()?;
                Ok(RpgValue::Data { class, value: Box::new(value) })
            }
            Tag::ObjectLink => {
                let index = self.read_usize()?;
                let target_pos = self.object_table.get(index).copied().ok_or_else(|| DecodeError {
                    kind: DecodeErrorKind::UnresolvedObjectLink(index),
                    position: Some(self.position),
                })?;

                let current_pos = self.position;
                self.position = target_pos;
                let tag = self.read_tag()?;
                let value = self.decode_value(tag)?;
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
        Tag::from_u8(b).ok_or_else(|| DecodeError { kind: DecodeErrorKind::InvalidTag(b), position: Some(self.position - 1) })
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
            _ => Err(DecodeError { kind: DecodeErrorKind::InvalidTag(tag as u8), position: Some(self.position - 1) }),
        }
    }
}

/// 从字节解码 RpgValue
/// 自动检测并处理 gzip 压缩数据
pub fn from_bytes(input: &[u8]) -> Result<RpgValue, DecodeError> {
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
