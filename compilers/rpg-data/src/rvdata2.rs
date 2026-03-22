use crate::marshal::{from_bytes, to_bytes};
use rpg_types::{Result, RpgError, RubyValue};
use std::io::{Read, Write};

/// 读取 rvdata2 文件并返回解析后的 RpgValue
/// 同时生成同名的 yaml 文件
pub fn read_rvdata2(file_path: &str) -> Result<RubyValue> {
    let mut file = std::fs::File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let value = from_bytes(&buffer).map_err(|e| RpgError::decode("rvdata2", &e.to_string()))?;

    let yaml_path = file_path.replace(".rvdata2", ".yaml");
    write_yaml(&yaml_path, &value)?;

    Ok(value)
}

/// 写入 rvdata2 文件
pub fn write_rvdata2(file_path: &str, value: &RubyValue) -> Result<()> {
    let data = to_bytes(value).map_err(|e| RpgError::encode("rvdata2", &e.to_string()))?;
    let mut file = std::fs::File::create(file_path)?;
    file.write_all(&data)?;
    Ok(())
}

fn write_yaml(file_path: &str, value: &RubyValue) -> Result<()> {
    let yaml_str = serde_yaml::to_string(value).map_err(|e| RpgError::encode("yaml", &e.to_string()))?;
    let mut file = std::fs::File::create(file_path)?;
    file.write_all(yaml_str.as_bytes())?;
    Ok(())
}
