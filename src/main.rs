use clap::Parser;
use std::io::Result;

mod args;
mod utils;
mod process;

use args::Args;
use process::process_mcpr;

fn main() -> Result<()> {
    let args = Args::parse();

    println!("Input: {:?}", args.input);
    println!("Output: {:?}", args.output);
    println!("Codes: {:?}", args.codes);

    // Parse the comma-separated codes. Hexadecimal (0x65) or decimal formats are accepted.
    let filter_ids: Result<Vec<u32>> = args.codes
        .split(',')
        .map(|s| utils::parse_packet_code(s.trim()))
        .collect();
    let filter_ids = filter_ids?;

    println!("Filtering the following packet IDs: {:?}", filter_ids);

    process_mcpr(&args.input, &args.output, &filter_ids)?;
    println!("Modified replay created: {:?}", args.output);
    Ok(())
}
