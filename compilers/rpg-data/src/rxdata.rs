use crate::marshal::{from_bytes, to_bytes};
use rpg_types::{Result, RpgError, RpgValue};
use std::io::{Read, Write};

/// 读取 rxdata 文件并返回解析后的 RpgValue
/// 同时生成同名的 yaml 文件
pub fn read_rxdata(file_path: &str) -> Result<RpgValue> {
    let mut file = std::fs::File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let value = from_bytes(&buffer).map_err(|e| RpgError::decode("rxdata", &e.to_string()))?;

    let yaml_path = file_path.replace(".rxdata", ".yaml");
    write_yaml(&yaml_path, &value)?;

    Ok(value)
}

/// 写入 rxdata 文件
pub fn write_rxdata(file_path: &str, value: &RpgValue) -> Result<()> {
    let data = to_bytes(value).map_err(|e| RpgError::encode("rxdata", &e.to_string()))?;
    let mut file = std::fs::File::create(file_path)?;
    file.write_all(&data)?;
    Ok(())
}

fn write_yaml(file_path: &str, value: &RpgValue) -> Result<()> {
    let yaml_str = serde_yaml::to_string(value).map_err(|e| RpgError::encode("yaml", &e.to_string()))?;
    let mut file = std::fs::File::create(file_path)?;
    file.write_all(yaml_str.as_bytes())?;
    Ok(())
}
