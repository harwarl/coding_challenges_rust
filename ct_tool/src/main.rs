
use std::{fs::File, io::{self, BufReader, Read, read_to_string}, path::PathBuf};

use clap::Parser;
#[derive(Debug, Parser)]
#[command(name = "Cut Tool", version, about = "A tool for cutting things", long_about = None)]
struct Args {
    /// Fields to cut (1 - based)
    #[arg(short, long)]
    fields: usize,

    /// path of file to read ,if not provided, it will read from stdin
    #[arg(help = "path to file to check")]
    file: Option<std::path::PathBuf>,
}

fn main() {
    let args = Args::parse();

    let file_contents = match args.file {
        Some(path) => {
            // Read in file as string
            std::fs::read_to_string(path).expect("Error reading file")
        },
        None => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer).expect("Failed to read line");
            buffer
        }
    };

    println!("Hello, world!");
}
