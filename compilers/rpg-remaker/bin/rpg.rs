use clap::Parser;
use rpg_data::rvdata2;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, clap::Subcommand)]
enum Command {
    /// Decode rvdata2 file
    Decode {
        /// Path to the rvdata2 file
        path: String,
    },
}

fn main() {
    let args = Args::parse();
    
    match args.command {
        Command::Decode { path } => {
            match rvdata2::read_rvdata2(&path) {
                Ok(value) => println!("{:?}", value),
                Err(e) => eprintln!("Error reading rvdata2 file: {}", e),
            }
        }
    }
}
