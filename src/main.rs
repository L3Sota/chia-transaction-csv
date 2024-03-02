use std::{fs::File, io::Read};

use clap::Parser;

/// Simple program to parse output from `chia wallet get_transactions`
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File to parse
    #[arg(short, long)]
    file_path: String,
}

fn main() {
    let args = Args::parse();

    eprintln!("Parsing {}", args.file_path);

    let mut file = File::open(args.file_path).expect("failed to open file");
    let mut content = {
        let cap = file.metadata().expect("failed to obtain file metadata").len();
        String::with_capacity(cap.try_into().unwrap_or(0))
    };
    file.read_to_string(&mut content).expect("failed to read file to string");

    // TODO
    println!("{}", content)
}
