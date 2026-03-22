#![warn(missing_docs)]

use alox_48::Value;
use serde::ser::{Serialize, SerializeMap, Serializer};

/// 可序列化的 Value 包装类型
pub struct SerializableValue<'a>(pub &'a Value);

impl<'a> Serialize for SerializableValue<'a> {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.0 {
            Value::Nil => serializer.serialize_none(),
            Value::Bool(b) => serializer.serialize_bool(*b),
            Value::Integer(i) => serializer.serialize_i64(*i as i64),
            Value::Float(f) => serializer.serialize_f64(*f),
            Value::String(s) => {
                let s = String::from_utf8_lossy(&s.data);
                serializer.serialize_str(&s)
            }
            Value::Symbol(sym) => serializer.serialize_str(&sym.to_string()),
            Value::Array(arr) => {
                let items: Vec<SerializableValue> = arr.iter().map(SerializableValue).collect();
                items.serialize(serializer)
            }
            Value::Hash(map) => {
                let mut map_serializer = serializer.serialize_map(Some(map.len()))?;
                for (k, v) in map {
                    map_serializer.serialize_entry(&SerializableValue(k), &SerializableValue(v))?;
                }
                map_serializer.end()
            }
            Value::Userdata(userdata) => {
                serializer.serialize_str(&format!("<Userdata: {} data={:?}>", userdata.class, userdata.data))
            }
            Value::Object(obj) => serializer.serialize_str(&format!("<Object: {}>", obj.class)),
            Value::Instance(inst) => {
                serializer.serialize_str(&format!("<Instance: {:?} fields={:?}>", inst.value, inst.fields))
            }
            Value::Regex { data, flags } => serializer.serialize_str(&format!("<Regex: /{}/{}/>", data, flags)),
            Value::RbStruct(st) => serializer.serialize_str(&format!("<Struct: {}>", st.class)),
            Value::Class(cls) => serializer.serialize_str(&format!("<Class: {}>", cls)),
            Value::Module(modu) => serializer.serialize_str(&format!("<Module: {}>", modu)),
            Value::Extended { module, value } => {
                serializer.serialize_str(&format!("<Extended: module={:?} value={:?}>", module, value))
            }
            Value::UserClass { class, value } => {
                serializer.serialize_str(&format!("<UserClass: class={:?} value={:?}>", class, value))
            }
            Value::UserMarshal { class, value } => {
                serializer.serialize_str(&format!("<UserMarshal: class={:?} value={:?}>", class, value))
            }
            Value::Data { class, value } => serializer.serialize_str(&format!("<Data: class={:?} value={:?}>", class, value)),
        }
    }
}

pub mod rvdata2;
pub mod rxdata;
