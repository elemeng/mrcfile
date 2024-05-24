use clap::Parser;
use std::fs::File;
use mrcfile::{image_processing::contrast::adjust_contrast};
use mrcfile::header::parser::MrcHeader;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the MRC file
    file_path: String,
}



fn main() {
    let matches = App::new("MRC Toolkit")
        .version("1.0")
        .about("Tools for processing MRC files")
        .subcommand(
            SubCommand::with_name("info")
                .about("Displays header information")
                .arg(Arg::with_name("FILE").help("Input MRC file").required(true)),
        )
        .subcommand(
            SubCommand::with_name("adjust-contrast")
                .about("Adjusts image contrast")
                .arg(Arg::with_name("FILE").help("Input MRC file").required(true))
                .arg(Arg::with_name("CONTRAST").help("Contrast value").required(true)),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("info") {
        let filename = matches.value_of("FILE").unwrap();
        let mut file = File::open(filename).expect("Failed to open file");
        let header = MrcHeader::from_reader(&mut file).expect("Failed to read header");
        println!("{:?}", header);
    }

    if let Some(matches) = matches.subcommand_matches("adjust-contrast") {
        let filename = matches.value_of("FILE").unwrap();
        let contrast: f32 = matches.value_of("CONTRAST").unwrap().parse().expect("Invalid contrast value");
        let mut file = File::open(filename).expect("Failed to open file");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).expect("Failed to read file");
        adjust_contrast(&mut buffer, contrast);
        // Save the adjusted image back or display it
    }
}
