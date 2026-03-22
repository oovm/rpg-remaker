use alox_48::{from_bytes, to_bytes, Value};
use crate::Result;
use serde::Serializer;
use serde_yaml;


/// 读取 rxdata 文件并返回解析后的值
/// 同时生成同名的 yaml 文件
pub fn read_rxdata(file_path: &str) -> Result<Value> {
    let mut file = std::fs::File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    
    let value = from_bytes(&buffer)?;
    
    // 生成同名的 yaml 文件
    let yaml_path = file_path.replace(".rxdata", ".yaml");
    write_yaml(&yaml_path, &value)?;
    
    Ok(value)
}

/// 写入 rxdata 文件
pub fn write_rxdata(file_path: &str, value: &Value) -> Result<()> {
    let mut file = std::fs::File::create(file_path)?;
    let data = to_bytes(value)?;
    file.write_all(&data)?;
    Ok(())
}

/// 将值写入 yaml 文件
fn write_yaml(file_path: &str, value: &Value) -> Result<()> {
    let yaml_str = value_to_yaml(value);
    let mut file = std::fs::File::create(file_path)?;
    file.write_all(yaml_str.as_bytes())?;
    Ok(())
}

/// 将 alox_48::Value 转换为 YAML 字符串
fn value_to_yaml(value: &Value) -> String {
    match value {
        Value::Nil => "null".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Integer(i) => i.to_string(),
        Value::Float(f) => f.to_string(),
        Value::String(s) => format!("\"{}\"", s),
        Value::Symbol(sym) => format!(":{}", sym),
        Value::Array(arr) => {
            let items: Vec<String> = arr.iter().map(value_to_yaml).collect();
            format!("[{}]", items.join(", "))
        }
        Value::Hash(map) => {
            let entries: Vec<String> = map.iter().map(|(k, v)| {
                format!("{}: {}", value_to_yaml(k), value_to_yaml(v))
            }).collect();
            format!("{{{}}}", entries.join(", "))
        }
        Value::Userdata(userdata) => format!(
            "<Userdata: {} data={:?}>",
            userdata.class,
            userdata.data
        ),
        Value::Object(obj) => format!("<Object: {}>", obj.class),
        Value::Instance(inst) => format!(
            "<Instance: {:?} fields={:?}>",
            inst.value,
            inst.fields
        ),
        Value::Regex { data, flags } => format!("<Regex: /{}/{}>", data, flags),
        Value::RbStruct(st) => format!("<Struct: {}>", st.class),
        Value::Class(cls) => format!("<Class: {}>", cls),
        Value::Module(modu) => format!("<Module: {}>", modu),
        Value::Extended { module, value } => format!(
            "<Extended: module={:?} value={:?}>",
            module,
            value
        ),
        Value::UserClass { class, value } => format!(
            "<UserClass: class={:?} value={:?}>",
            class,
            value
        ),
        Value::UserMarshal { class, value } => format!(
            "<UserMarshal: class={:?} value={:?}>",
            class,
            value
        ),
        Value::Data { class, value } => format!(
            "<Data: class={:?} value={:?}>",
            class,
            value
        ),
    }
}
