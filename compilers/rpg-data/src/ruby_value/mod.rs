use alox_48::{Instance, Object, RbArray, RbFields, RbHash, RbString, RbStruct, Symbol, Userdata, Value};
use rpg_types::RpgValue;
use std::collections::HashMap;

impl From<&Value> for RpgValue {
    fn from(value: &Value) -> Self {
        match value {
            Value::Nil => RpgValue::Nil,
            Value::Bool(b) => RpgValue::Bool(*b),
            Value::Integer(i) => RpgValue::Integer(*i),
            Value::Float(f) => RpgValue::Float(*f),
            Value::String(s) => RpgValue::String(s.data.clone()),
            Value::Symbol(s) => RpgValue::Symbol(s.to_string()),
            Value::Array(arr) => RpgValue::Array(arr.iter().map(RpgValue::from).collect()),
            Value::Hash(hash) => {
                let mut map = HashMap::new();
                for (k, v) in hash {
                    map.insert(RpgValue::from(k), RpgValue::from(v));
                }
                RpgValue::Hash(map)
            }
            Value::Userdata(u) => RpgValue::Userdata {
                class: u.class.to_string(),
                data: u.data.clone(),
            },
            Value::Object(o) => RpgValue::Object {
                class: o.class.to_string(),
                fields: convert_fields(&o.fields),
            },
            Value::Instance(i) => RpgValue::Instance {
                value: Box::new(RpgValue::from(i.value.as_ref())),
                fields: convert_fields(&i.fields),
            },
            Value::Regex { data, flags } => RpgValue::Regex {
                pattern: data.data.clone(),
                flags: *flags,
            },
            Value::RbStruct(s) => RpgValue::Struct {
                class: s.class.to_string(),
                fields: convert_fields(&s.fields),
            },
            Value::Class(c) => RpgValue::Class(c.to_string()),
            Value::Module(m) => RpgValue::Module(m.to_string()),
            Value::Extended { module, value } => RpgValue::Extended {
                module: module.to_string(),
                value: Box::new(RpgValue::from(value.as_ref())),
            },
            Value::UserClass { class, value } => RpgValue::UserClass {
                class: class.to_string(),
                value: Box::new(RpgValue::from(value.as_ref())),
            },
            Value::UserMarshal { class, value } => RpgValue::UserMarshal {
                class: class.to_string(),
                value: Box::new(RpgValue::from(value.as_ref())),
            },
            Value::Data { class, value } => RpgValue::Data {
                class: class.to_string(),
                value: Box::new(RpgValue::from(value.as_ref())),
            },
        }
    }
}

impl From<&RpgValue> for Value {
    fn from(value: &RpgValue) -> Self {
        match value {
            RpgValue::Nil => Value::Nil,
            RpgValue::Bool(b) => Value::Bool(*b),
            RpgValue::Integer(i) => Value::Integer(*i),
            RpgValue::Float(f) => Value::Float(*f),
            RpgValue::String(s) => Value::String(RbString { data: s.clone() }),
            RpgValue::Symbol(s) => Value::Symbol(Symbol::from(s.as_str())),
            RpgValue::Array(arr) => {
                let values: RbArray = arr.iter().map(Value::from).collect();
                Value::Array(values)
            }
            RpgValue::Hash(map) => {
                let mut hash = RbHash::new();
                for (k, v) in map {
                    hash.insert(Value::from(k), Value::from(v));
                }
                Value::Hash(hash)
            }
            RpgValue::Object { class, fields } => Value::Object(Object {
                class: Symbol::from(class.as_str()),
                fields: convert_to_rb_fields(fields),
            }),
            RpgValue::Userdata { class, data } => Value::Userdata(Userdata {
                class: Symbol::from(class.as_str()),
                data: data.clone(),
            }),
            RpgValue::Instance { value, fields } => Value::Instance(Instance {
                value: Box::new(Value::from(value.as_ref())),
                fields: convert_to_rb_fields(fields),
            }),
            RpgValue::Regex { pattern, flags } => Value::Regex {
                data: RbString { data: pattern.clone() },
                flags: *flags,
            },
            RpgValue::Struct { class, fields } => Value::RbStruct(RbStruct {
                class: Symbol::from(class.as_str()),
                fields: convert_to_rb_fields(fields),
            }),
            RpgValue::Class(c) => Value::Class(Symbol::from(c.as_str())),
            RpgValue::Module(m) => Value::Module(Symbol::from(m.as_str())),
            RpgValue::Extended { module, value } => Value::Extended {
                module: Symbol::from(module.as_str()),
                value: Box::new(Value::from(value.as_ref())),
            },
            RpgValue::UserClass { class, value } => Value::UserClass {
                class: Symbol::from(class.as_str()),
                value: Box::new(Value::from(value.as_ref())),
            },
            RpgValue::UserMarshal { class, value } => Value::UserMarshal {
                class: Symbol::from(class.as_str()),
                value: Box::new(Value::from(value.as_ref())),
            },
            RpgValue::Data { class, value } => Value::Data {
                class: Symbol::from(class.as_str()),
                value: Box::new(Value::from(value.as_ref())),
            },
        }
    }
}

fn convert_fields(fields: &RbFields) -> HashMap<String, RpgValue> {
    let mut map = HashMap::new();
    for (k, v) in fields {
        map.insert(k.to_string(), RpgValue::from(v));
    }
    map
}

fn convert_to_rb_fields(fields: &HashMap<String, RpgValue>) -> RbFields {
    let mut rb_fields = RbFields::new();
    for (k, v) in fields {
        rb_fields.insert(Symbol::from(k.as_str()), Value::from(v));
    }
    rb_fields
}

pub fn value_to_rpg(value: &Value) -> RpgValue {
    RpgValue::from(value)
}

pub fn rpg_to_value(value: &RpgValue) -> Value {
    Value::from(value)
}
