use alox_48::{from_bytes, to_bytes, Value};
use std::io::{Read, Write};
use crate::{Result, RpgError};

/// 读取 rvdata2 文件并返回解析后的值
/// 同时生成同名的 yaml 文件
pub fn read_rvdata2(file_path: &str) -> Result<Value> {
    let mut file = std::fs::File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    
    let value = from_bytes(&buffer)?;
    
    // 生成同名的 yaml 文件
    let yaml_path = file_path.replace(".rvdata2", ".yaml");
    write_yaml(&yaml_path, &value)?;
    
    Ok(value)
}

/// 写入 rvdata2 文件
pub fn write_rvdata2(file_path: &str, value: &Value) -> Result<()> {
    let mut file = std::fs::File::create(file_path)?;
    let data = to_bytes(value)?;
    file.write_all(&data)?;
    Ok(())
}

/// 将值写入 yaml 文件
fn write_yaml(file_path: &str, value: &Value) -> Result<()> {
    let yaml = serde_yaml::to_string(value)?;
    let mut file = std::fs::File::create(file_path)?;
    file.write_all(yaml.as_bytes())?;
    Ok(())
}
