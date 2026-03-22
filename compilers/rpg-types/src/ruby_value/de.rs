use serde::de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
use std::{collections::HashMap, fmt};

use super::{RpgFields, RpgHash, RpgHashKey, RubyValue};

impl<'de> Deserialize<'de> for RubyValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(RpgValueVisitor)
    }
}

struct RpgValueVisitor;

impl<'de> Visitor<'de> for RpgValueVisitor {
    type Value = RubyValue;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a valid RpgValue")
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(RubyValue::Nil)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(RubyValue::Nil)
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(RubyValue::Bool(v))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(RubyValue::Integer(v as i32))
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(RubyValue::Integer(v))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(RubyValue::Float(v))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(RubyValue::String(v.as_bytes().to_vec()))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(RubyValue::String(v.into_bytes()))
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(RubyValue::String(v.to_vec()))
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(RubyValue::String(v))
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut arr = Vec::with_capacity(seq.size_hint().unwrap_or(0));
        while let Some(item) = seq.next_element()? {
            arr.push(item);
        }
        Ok(RubyValue::Array(arr))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut raw_map: HashMap<String, RubyValue> = HashMap::with_capacity(map.size_hint().unwrap_or(0));
        while let Some((key, value)) = map.next_entry()? {
            raw_map.insert(key, value);
        }

        if let Some(RubyValue::Bool(true)) = raw_map.get("__symbol__") {
            if let Some(RubyValue::String(s)) = raw_map.get("value") {
                return Ok(RubyValue::Symbol(String::from_utf8_lossy(s).to_string()));
            }
        }

        if let Some(RubyValue::String(class)) = raw_map.remove("__class__") {
            let class_str = String::from_utf8_lossy(&class).to_string();
            let fields: RpgFields = raw_map.into_iter().map(|(k, v)| (k, v)).collect();
            return Ok(RubyValue::Object { class: class_str, fields });
        }

        if let Some(RubyValue::Bool(true)) = raw_map.get("__userdata__") {
            let class = extract_string(raw_map.get("class"));
            let data = extract_bytes(raw_map.get("data"));
            return Ok(RubyValue::Userdata { class, data });
        }

        if let Some(RubyValue::Bool(true)) = raw_map.get("__instance__") {
            let value = raw_map.get("value").cloned().unwrap_or(RubyValue::Nil);
            let fields = extract_fields_from_hash(raw_map.get("fields"));
            return Ok(RubyValue::Instance { value: Box::new(value), fields });
        }

        if let Some(RubyValue::Bool(true)) = raw_map.get("__regex__") {
            let pattern = extract_bytes(raw_map.get("pattern"));
            let flags = extract_u8(raw_map.get("flags"));
            return Ok(RubyValue::Regex { pattern, flags });
        }

        if let Some(RubyValue::Bool(true)) = raw_map.get("__struct__") {
            let class = extract_string(raw_map.get("class"));
            let mut fields = RpgFields::new();
            for (k, v) in raw_map {
                if k != "__struct__" && k != "class" {
                    fields.insert(k, v);
                }
            }
            return Ok(RubyValue::Struct { class, fields });
        }

        if let Some(RubyValue::Bool(true)) = raw_map.get("__class_ref__") {
            let name = extract_string(raw_map.get("name"));
            return Ok(RubyValue::Class(name));
        }

        if let Some(RubyValue::Bool(true)) = raw_map.get("__module__") {
            let name = extract_string(raw_map.get("name"));
            return Ok(RubyValue::Module(name));
        }

        if let Some(RubyValue::Bool(true)) = raw_map.get("__extended__") {
            let module = extract_string(raw_map.get("module"));
            let value = raw_map.get("value").cloned().unwrap_or(RubyValue::Nil);
            return Ok(RubyValue::Extended { module, value: Box::new(value) });
        }

        if let Some(RubyValue::Bool(true)) = raw_map.get("__user_class__") {
            let class = extract_string(raw_map.get("class"));
            let value = raw_map.get("value").cloned().unwrap_or(RubyValue::Nil);
            return Ok(RubyValue::UserClass { class, value: Box::new(value) });
        }

        if let Some(RubyValue::Bool(true)) = raw_map.get("__user_marshal__") {
            let class = extract_string(raw_map.get("class"));
            let value = raw_map.get("value").cloned().unwrap_or(RubyValue::Nil);
            return Ok(RubyValue::UserMarshal { class, value: Box::new(value) });
        }

        if let Some(RubyValue::Bool(true)) = raw_map.get("__data__") {
            let class = extract_string(raw_map.get("class"));
            let value = raw_map.get("value").cloned().unwrap_or(RubyValue::Nil);
            return Ok(RubyValue::Data { class, value: Box::new(value) });
        }

        let hash: RpgHash = raw_map.into_iter().map(|(k, v)| (RpgHashKey(RubyValue::String(k.into_bytes())), v)).collect();
        Ok(RubyValue::Hash(hash))
    }
}

fn extract_string(value: Option<&RubyValue>) -> String {
    match value {
        Some(RubyValue::String(s)) => String::from_utf8_lossy(s).to_string(),
        Some(RubyValue::Symbol(s)) => s.clone(),
        _ => String::new(),
    }
}

fn extract_bytes(value: Option<&RubyValue>) -> Vec<u8> {
    match value {
        Some(RubyValue::String(s)) => s.clone(),
        _ => Vec::new(),
    }
}

fn extract_u8(value: Option<&RubyValue>) -> u8 {
    match value {
        Some(RubyValue::Integer(i)) => *i as u8,
        _ => 0,
    }
}

fn extract_fields_from_hash(value: Option<&RubyValue>) -> RpgFields {
    match value {
        Some(RubyValue::Hash(h)) => h
            .iter()
            .filter_map(|(k, v)| match &k.0 {
                RubyValue::String(s) => Some((String::from_utf8_lossy(s).to_string(), v.clone())),
                RubyValue::Symbol(s) => Some((s.clone(), v.clone())),
                _ => None,
            })
            .collect(),
        _ => RpgFields::new(),
    }
}

impl<'de> Deserialize<'de> for RpgHashKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        RubyValue::deserialize(deserializer).map(RpgHashKey)
    }
}
