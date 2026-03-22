use alox_48::{from_bytes, to_bytes, Value};
use std::io::{Read, Write};
use crate::{Result, SerializableValue};

/// 读取 rxdata 文件并返回解析后的值
/// 同时生成同名的 yaml 文件
pub fn read_rxdata(file_path: &str) -> Result<Value> {
    let mut file = std::fs::File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    
    let value = from_bytes(&buffer)?;
    
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
    let yaml_str = serde_yaml::to_string(&SerializableValue(value))?;
    let mut file = std::fs::File::create(file_path)?;
    file.write_all(yaml_str.as_bytes())?;
    Ok(())
}
