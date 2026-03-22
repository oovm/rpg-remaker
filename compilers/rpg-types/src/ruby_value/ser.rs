use serde::ser::{Serialize, SerializeMap, SerializeSeq, Serializer};

use super::{RpgFields, RpgHash, RpgHashKey, RpgValue};

impl Serialize for RpgValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            RpgValue::Nil => serializer.serialize_none(),
            RpgValue::Bool(b) => serializer.serialize_bool(*b),
            RpgValue::Integer(i) => serializer.serialize_i32(*i),
            RpgValue::Float(f) => serializer.serialize_f64(*f),
            RpgValue::String(s) => {
                let lossy = String::from_utf8_lossy(s);
                serializer.serialize_str(&lossy)
            }
            RpgValue::Symbol(s) => {
                serializer.serialize_newtype_struct("!symbol", &SymbolWrapper(s))
            }
            RpgValue::Array(arr) => {
                let mut seq = serializer.serialize_seq(Some(arr.len()))?;
                for item in arr {
                    seq.serialize_element(item)?;
                }
                seq.end()
            }
            RpgValue::Hash(map) => {
                let mut map_ser = serializer.serialize_map(Some(map.len()))?;
                for (k, v) in map {
                    map_ser.serialize_entry(k, v)?;
                }
                map_ser.end()
            }
            RpgValue::Object { class, fields } => {
                serializer.serialize_newtype_struct(&format!("!{}", class), &ObjectFields(fields))
            }
            RpgValue::Userdata { class, data } => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("class", class)?;
                let lossy = String::from_utf8_lossy(data);
                map.serialize_entry("data", &lossy.as_ref())?;
                map.end()
            }
            RpgValue::Instance { value, fields } => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("value", value)?;
                map.serialize_entry("fields", &ObjectFields(fields))?;
                map.end()
            }
            RpgValue::Regex { pattern, flags } => {
                let mut map = serializer.serialize_map(Some(2))?;
                let lossy = String::from_utf8_lossy(pattern);
                map.serialize_entry("pattern", &lossy.as_ref())?;
                map.serialize_entry("flags", flags)?;
                map.end()
            }
            RpgValue::Struct { class, fields } => {
                serializer.serialize_newtype_struct(&format!("!struct:{}", class), &ObjectFields(fields))
            }
            RpgValue::Class(c) => {
                serializer.serialize_newtype_struct("!class", c)
            }
            RpgValue::Module(m) => {
                serializer.serialize_newtype_struct("!module", m)
            }
            RpgValue::Extended { module, value } => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("module", module)?;
                map.serialize_entry("value", value)?;
                map.end()
            }
            RpgValue::UserClass { class, value } => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("class", class)?;
                map.serialize_entry("value", value)?;
                map.end()
            }
            RpgValue::UserMarshal { class, value } => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("class", class)?;
                map.serialize_entry("value", value)?;
                map.end()
            }
            RpgValue::Data { class, value } => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("class", class)?;
                map.serialize_entry("value", value)?;
                map.end()
            }
        }
    }
}

impl Serialize for RpgHashKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

struct SymbolWrapper<'a>(&'a str);

impl<'a> Serialize for SymbolWrapper<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.0)
    }
}

struct ObjectFields<'a>(&'a RpgFields);

impl<'a> Serialize for ObjectFields<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.0.len()))?;
        for (k, v) in self.0 {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
