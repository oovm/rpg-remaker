use clap::Parser;
use rpg_data::{rvdata2, rxdata};
use std::fs;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, clap::Subcommand)]
enum Command {
    /// Decode rvdata2 or rxdata files
    Decode {
        /// Path to the file or directory
        path: String,
    },
}

fn main() {
    let args = Args::parse();
    
    match args.command {
        Command::Decode { path } => {
            let path = Path::new(&path);
            if path.is_dir() {
                // Process all rvdata2 and rxdata files in the directory
                process_directory(path);
            } else if path.is_file() {
                // Process a single file
                process_file(path);
            } else {
                eprintln!("Error: {} is not a file or directory", path.display());
            }
        }
    }
}

/// Process all rvdata2 and rxdata files in a directory
fn process_directory(dir: &Path) {
    for entry in fs::read_dir(dir).expect("Failed to read directory") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();
        if path.is_dir() {
            // Recursively process subdirectories
            process_directory(&path);
        } else if path.is_file() {
            process_file(&path);
        }
    }
}

/// Process a single file if it's a rvdata2 or rxdata file
fn process_file(path: &Path) {
    let extension = path.extension().and_then(|ext| ext.to_str());
    match extension {
        Some("rvdata2") => {
            println!("Processing rvdata2 file: {}", path.display());
            match rvdata2::read_rvdata2(path.to_str().unwrap()) {
                Ok(_) => println!("Successfully decoded {}", path.display()),
                Err(e) => eprintln!("Error decoding {}: {}", path.display(), e),
            }
        }
        Some("rxdata") => {
            println!("Processing rxdata file: {}", path.display());
            match rxdata::read_rxdata(path.to_str().unwrap()) {
                Ok(_) => println!("Successfully decoded {}", path.display()),
                Err(e) => eprintln!("Error decoding {}: {}", path.display(), e),
            }
        }
        _ => {
            // Skip files with other extensions
        }
    }
}
