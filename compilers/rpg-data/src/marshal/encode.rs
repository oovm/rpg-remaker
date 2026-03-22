use indexmap::IndexSet;

use super::tag::Tag;
use rpg_types::RpgValue;

/// Marshal 编码错误
#[derive(Debug, Clone)]
pub struct EncodeError {
    /// 错误消息
    pub message: String,
}

impl std::fmt::Display for EncodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "encode error: {}", self.message)
    }
}

impl std::error::Error for EncodeError {}

/// Marshal 编码器
pub struct Encoder {
    output: Vec<u8>,
    symbol_table: IndexSet<String>,
}

impl Default for Encoder {
    fn default() -> Self {
        Self { output: vec![4, 8], symbol_table: IndexSet::new() }
    }
}

impl Encoder {
    /// 创建新的编码器
    pub fn new() -> Self {
        Self::default()
    }

    /// 编码 RpgValue
    pub fn encode(&mut self, value: &RpgValue) -> Result<Vec<u8>, EncodeError> {
        self.encode_value(value)?;
        Ok(std::mem::take(&mut self.output))
    }

    fn encode_value(&mut self, value: &RpgValue) -> Result<(), EncodeError> {
        match value {
            RpgValue::Nil => {
                self.write_tag(Tag::Nil);
            }
            RpgValue::Bool(b) => {
                self.write_tag(if *b { Tag::True } else { Tag::False });
            }
            RpgValue::Integer(i) => {
                self.write_tag(Tag::Integer);
                self.write_packed_int(*i);
            }
            RpgValue::Float(f) => {
                self.write_tag(Tag::Float);
                let s = f.to_string();
                self.write_bytes_len(s.as_bytes());
            }
            RpgValue::String(s) => {
                self.write_tag(Tag::String);
                self.write_bytes_len(s);
            }
            RpgValue::Symbol(s) => {
                self.write_symbol(s);
            }
            RpgValue::Array(arr) => {
                self.write_tag(Tag::Array);
                self.write_packed_int(arr.len() as i32);
                for item in arr {
                    self.encode_value(item)?;
                }
            }
            RpgValue::Hash(hash) => {
                self.write_tag(Tag::Hash);
                self.write_packed_int(hash.len() as i32);
                for (k, v) in hash {
                    self.encode_value(&k.0)?;
                    self.encode_value(v)?;
                }
            }
            RpgValue::Object { class, fields } => {
                self.write_tag(Tag::Object);
                self.write_symbol(class);
                self.write_packed_int(fields.len() as i32);
                for (k, v) in fields {
                    self.write_symbol(k);
                    self.encode_value(v)?;
                }
            }
            RpgValue::Userdata { class, data } => {
                self.write_tag(Tag::UserDef);
                self.write_symbol(class);
                self.write_bytes_len(data);
            }
            RpgValue::Instance { value, fields } => {
                self.write_tag(Tag::Instance);
                self.encode_value(value)?;
                self.write_packed_int(fields.len() as i32);
                for (k, v) in fields {
                    self.write_symbol(k);
                    self.encode_value(v)?;
                }
            }
            RpgValue::Regex { pattern, flags } => {
                self.write_tag(Tag::RawRegexp);
                self.write_bytes_len(pattern);
                self.write_byte(*flags);
            }
            RpgValue::Struct { class, fields } => {
                self.write_tag(Tag::Struct);
                self.write_symbol(class);
                self.write_packed_int(fields.len() as i32);
                for (k, v) in fields {
                    self.write_symbol(k);
                    self.encode_value(v)?;
                }
            }
            RpgValue::Class(c) => {
                self.write_tag(Tag::ClassRef);
                self.write_bytes_len(c.as_bytes());
            }
            RpgValue::Module(m) => {
                self.write_tag(Tag::ModuleRef);
                self.write_bytes_len(m.as_bytes());
            }
            RpgValue::Extended { module, value } => {
                self.write_tag(Tag::Extended);
                self.write_symbol(module);
                self.encode_value(value)?;
            }
            RpgValue::UserClass { class, value } => {
                self.write_tag(Tag::UserClass);
                self.write_symbol(class);
                self.encode_value(value)?;
            }
            RpgValue::UserMarshal { class, value } => {
                self.write_tag(Tag::UserMarshal);
                self.write_symbol(class);
                self.encode_value(value)?;
            }
            RpgValue::Data { class, value } => {
                self.write_tag(Tag::Data);
                self.write_symbol(class);
                self.encode_value(value)?;
            }
        }
        Ok(())
    }

    fn write_byte(&mut self, b: u8) {
        self.output.push(b);
    }

    fn write_tag(&mut self, tag: Tag) {
        self.output.push(tag as u8);
    }

    fn write_packed_int(&mut self, v: i32) {
        let v = v as i64;
        match v {
            0 => self.write_byte(0),
            1..=122 => self.write_byte(v as u8 + 5),
            -122..=0 => self.write_byte((256 + v - 5) as u8),
            mut v => {
                let mut bytes = vec![];
                for _ in 0..4 {
                    let b = (v & 0xFF) as u8;
                    bytes.push(b);
                    v >>= 8;
                    if v == 0 || v == -1 {
                        break;
                    }
                }
                let len_byte = if v < 0 { (256 - bytes.len()) as u8 } else { bytes.len() as u8 };
                self.write_byte(len_byte);
                for b in bytes {
                    self.write_byte(b);
                }
            }
        }
    }

    fn write_symbol(&mut self, sym: &str) {
        if let Some(idx) = self.symbol_table.get_index_of(sym) {
            self.write_tag(Tag::Symlink);
            self.write_packed_int(idx as i32);
        }
        else {
            self.symbol_table.insert(sym.to_string());
            self.write_tag(Tag::Symbol);
            self.write_bytes_len(sym.as_bytes());
        }
    }

    fn write_bytes_len(&mut self, bytes: &[u8]) {
        self.write_packed_int(bytes.len() as i32);
        self.output.extend_from_slice(bytes);
    }
}

/// 将 RpgValue 编码为字节
pub fn to_bytes(value: &RpgValue) -> Result<Vec<u8>, EncodeError> {
    let mut encoder = Encoder::new();
    encoder.encode(value)
}
