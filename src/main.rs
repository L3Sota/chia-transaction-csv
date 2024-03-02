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

    eprintln!("Parsing {}", args.file_path)
}
