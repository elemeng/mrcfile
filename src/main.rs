use clap::Parser;
use std::fs::File;
//use mrcfile::{image_processing::contrast::adjust_contrast};
use mrcfile::header::parser::MrcHeader;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the MRC file
    file_path: String,
}

fn main() {
    let args = Args::parse();
    match mrc::parser::parse_header(&args.file) {
        Ok(header) => println!("{:?}", header),
        Err(e) => eprintln!("Error parsing file: {}", e),
    }
}
