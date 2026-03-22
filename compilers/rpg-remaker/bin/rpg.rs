use clap::Parser;
use rpg_data::{read_rvdata2, read_rxdata};
use std::path::Path;
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
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Decode { path } => {
            decode_path(&path);
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
