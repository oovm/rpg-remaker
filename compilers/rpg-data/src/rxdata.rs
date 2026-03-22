use crate::ruby_value::value_to_rpg;
use alox_48::{Value, from_bytes, to_bytes};
use flate2::read::GzDecoder;
use rpg_types::{Result, RpgError, RpgValue};
use std::io::{Read, Write};

/// gzip 压缩文件的魔术字节
const GZIP_MAGIC: [u8; 2] = [0x1F, 0x8B];

/// 检测数据是否为 gzip 压缩
fn is_gzip_compressed(data: &[u8]) -> bool {
    data.len() >= 2 && data[0..2] == GZIP_MAGIC
}

/// 解压 gzip 数据
fn decompress_gzip(data: &[u8]) -> Result<Vec<u8>> {
    let mut decoder = GzDecoder::new(data);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed).map_err(|e| RpgError::decode("gzip", &e.to_string()))?;
    Ok(decompressed)
}

/// 读取 rxdata 文件并返回解析后的 RpgValue
/// 同时生成同名的 yaml 文件
/// 自动检测并处理 gzip 压缩的存档文件
pub fn read_rxdata(file_path: &str) -> Result<RpgValue> {
    let mut file = std::fs::File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let data = if is_gzip_compressed(&buffer) { decompress_gzip(&buffer)? } else { buffer };

    let value: Value = from_bytes(&data).map_err(|e| RpgError::decode("rxdata", &e.to_string()))?;

    let yaml_path = file_path.replace(".rxdata", ".yaml");
    let rpg_value = value_to_rpg(&value);
    write_yaml(&yaml_path, &rpg_value)?;

    Ok(rpg_value)
}

/// 写入 rxdata 文件
pub fn write_rxdata(file_path: &str, value: &RpgValue) -> Result<()> {
    let alox_value = Value::from(value);
    let mut file = std::fs::File::create(file_path)?;
    let data = to_bytes(&alox_value).map_err(|e| RpgError::encode("rxdata", &e.to_string()))?;
    file.write_all(&data)?;
    Ok(())
}

/// 将值写入 yaml 文件
fn write_yaml(file_path: &str, value: &RpgValue) -> Result<()> {
    let yaml_str = serde_yaml::to_string(value).map_err(|e| RpgError::encode("yaml", &e.to_string()))?;
    let mut file = std::fs::File::create(file_path)?;
    file.write_all(yaml_str.as_bytes())?;
    Ok(())
}
