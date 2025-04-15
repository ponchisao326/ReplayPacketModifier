use clap::Parser;
use std::path::PathBuf;

/// Application to filter specific packets in replays (.mcpr)
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Input file (.mcpr)
    #[arg(short, long)]
    pub input: PathBuf,

    /// Output file (.mcpr)
    #[arg(short, long)]
    pub output: PathBuf,

    /// List of packet codes to filter (comma-separated, e.g., "0x65,0x03")
    #[arg(short, long)]
    pub codes: String,
}
