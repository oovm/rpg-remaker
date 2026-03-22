use clap::Parser;
use rpg_data::{read_rvdata2, read_rxdata, write_rvdata2, write_rxdata};
use rpg_types::{Result, RubyValue};
use std::{fs::File, io::Read, path::Path};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, clap::Subcommand)]
enum Command {
    /// 解码 rvdata2 或 rxdata 文件
    Decode {
        /// 文件或目录路径
        path: String,
    },
    /// 编码 YAML 文件为 rvdata2 或 rxdata 格式
    Encode {
        /// 文件或目录路径
        path: String,
    },
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Decode { path } => {
            decode_path(&path);
        }
        Command::Encode { path } => {
            encode_path(&path);
        }
    }
}

/// 解码指定路径下的所有 rvdata2 和 rxdata 文件
fn decode_path(path: &str) {
    let path = Path::new(path);

    if path.is_file() {
        process_file(path);
        return;
    }

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            process_file(entry.path());
        }
    }
}

/// 处理单个文件
fn process_file(path: &Path) {
    let extension = path.extension().and_then(|ext| ext.to_str());
    let result = match extension {
        Some("rvdata2") => read_rvdata2(path.to_str().unwrap()),
        Some("rxdata") => read_rxdata(path.to_str().unwrap()),
        _ => return,
    };
    if let Err(e) = result {
        eprintln!("解码 {} 失败: {}", path.display(), e);
    }
}

/// 编码指定路径下的所有 YAML 文件
fn encode_path(path: &str) {
    let path = Path::new(path);

    if path.is_file() {
        encode_file(path);
        return;
    }

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            encode_file(entry.path());
        }
    }
}

/// 处理单个 YAML 文件的编码
fn encode_file(path: &Path) {
    let extension = path.extension().and_then(|ext| ext.to_str());
    if extension != Some("yaml") {
        return;
    }

    let result = read_yaml(path.to_str().unwrap()).and_then(|value| {
        let file_stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
        let output_path = match file_stem {
            "Actor" | "Class" | "Skill" | "Item" | "Weapon" | "Armor" | "Enemy" | "Troop" | "State" | "Animation"
            | "CommonEvent" | "System" | "Map" => path.to_str().unwrap().replace(".yaml", ".rvdata2"),
            _ => path.to_str().unwrap().replace(".yaml", ".rvdata2"),
        };
        write_rvdata2(&output_path, &value)
    });

    if let Err(e) = result {
        eprintln!("编码 {} 失败: {}", path.display(), e);
    }
    else {
        println!("成功编码 {}", path.display());
    }
}

/// 读取 YAML 文件并解析为 RubyValue
fn read_yaml(file_path: &str) -> Result<RubyValue> {
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    serde_yaml::from_str(&content).map_err(|e| rpg_types::RpgError::decode("yaml", &e.to_string()))
}
