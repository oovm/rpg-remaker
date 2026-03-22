use crate::{Result, RgssError};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

pub fn read_file(path: &Path) -> Result<String> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

pub fn write_file(path: &Path, content: &str) -> Result<()>
{
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn parse_script_name(script: &str) -> Option<String> {
    let lines = script.lines();
    for line in lines {
        if line.starts_with("#") {
            let name = line.trim_start_matches("#").trim();
            if !name.is_empty() {
                return Some(name.to_string());
            }
        }
    }
    None
}

pub fn validate_script(script: &str) -> bool {
    // 简单的脚本验证
    // 实际实现中可能需要更复杂的验证逻辑
    !script.is_empty()
}
