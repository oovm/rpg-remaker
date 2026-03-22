use serde::ser::{Serialize, SerializeMap, SerializeSeq, Serializer};

use super::{RpgFields, RpgHashKey, RubyValue};

impl Serialize for RubyValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            RubyValue::Nil => serializer.serialize_none(),
            RubyValue::Bool(b) => serializer.serialize_bool(*b),
            RubyValue::Integer(i) => serializer.serialize_i32(*i),
            RubyValue::Float(f) => serializer.serialize_f64(*f),
            RubyValue::String(s) => {
                let lossy = String::from_utf8_lossy(s);
                serializer.serialize_str(&lossy)
            }
            RubyValue::Symbol(s) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("__symbol__", &true)?;
                map.serialize_entry("value", s)?;
                map.end()
            }
            RubyValue::Array(arr) => {
                let mut seq = serializer.serialize_seq(Some(arr.len()))?;
                for item in arr {
                    seq.serialize_element(item)?;
                }
                seq.end()
            }
            RubyValue::Hash(map) => {
                let mut map_ser = serializer.serialize_map(Some(map.len()))?;
                for (k, v) in map {
                    map_ser.serialize_entry(k, v)?;
                }
                map_ser.end()
            }
            RubyValue::Object { class, fields } => {
                let mut map = serializer.serialize_map(Some(fields.len() + 1))?;
                map.serialize_entry("__class__", class)?;
                for (k, v) in fields {
                    map.serialize_entry(k, v)?;
                }
                map.end()
            }
            RubyValue::Userdata { class, data } => {
                let mut map = serializer.serialize_map(Some(3))?;
                map.serialize_entry("__userdata__", &true)?;
                map.serialize_entry("class", class)?;
                let lossy = String::from_utf8_lossy(data);
                map.serialize_entry("data", &lossy.as_ref())?;
                map.end()
            }
            RubyValue::Instance { value, fields } => {
                let mut map = serializer.serialize_map(Some(3))?;
                map.serialize_entry("__instance__", &true)?;
                map.serialize_entry("value", value)?;
                map.serialize_entry("fields", &ObjectFields(fields))?;
                map.end()
            }
            RubyValue::Regex { pattern, flags } => {
                let mut map = serializer.serialize_map(Some(3))?;
                map.serialize_entry("__regex__", &true)?;
                let lossy = String::from_utf8_lossy(pattern);
                map.serialize_entry("pattern", &lossy.as_ref())?;
                map.serialize_entry("flags", flags)?;
                map.end()
            }
            RubyValue::Struct { class, fields } => {
                let mut map = serializer.serialize_map(Some(fields.len() + 2))?;
                map.serialize_entry("__struct__", &true)?;
                map.serialize_entry("class", class)?;
                for (k, v) in fields {
                    map.serialize_entry(k, v)?;
                }
                map.end()
            }
            RubyValue::Class(c) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("__class_ref__", &true)?;
                map.serialize_entry("name", c)?;
                map.end()
            }
            RubyValue::Module(m) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("__module__", &true)?;
                map.serialize_entry("name", m)?;
                map.end()
            }
            RubyValue::Extended { module, value } => {
                let mut map = serializer.serialize_map(Some(3))?;
                map.serialize_entry("__extended__", &true)?;
                map.serialize_entry("module", module)?;
                map.serialize_entry("value", value)?;
                map.end()
            }
            RubyValue::UserClass { class, value } => {
                let mut map = serializer.serialize_map(Some(3))?;
                map.serialize_entry("__user_class__", &true)?;
                map.serialize_entry("class", class)?;
                map.serialize_entry("value", value)?;
                map.end()
            }
            RubyValue::UserMarshal { class, value } => {
                let mut map = serializer.serialize_map(Some(3))?;
                map.serialize_entry("__user_marshal__", &true)?;
                map.serialize_entry("class", class)?;
                map.serialize_entry("value", value)?;
                map.end()
            }
            RubyValue::Data { class, value } => {
                let mut map = serializer.serialize_map(Some(3))?;
                map.serialize_entry("__data__", &true)?;
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
