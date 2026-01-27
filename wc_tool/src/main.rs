use std::{
    fs::{self},
    io::{self, Read},
    path::Path,
};

use clap::Parser;

/// Search for a pattern in a file and display lines that contain it
#[derive(Parser, Debug)]
#[command(name = "Wc Tool", version = "1.0", about ="Just a CLI", long_about = None)]
struct CLI {
    #[arg(short, long, help = "counts the number of bytes in a file", action = clap::ArgAction::SetTrue)]
    count: Option<bool>,

    #[arg(short, long, help = "gets the number of lines in a file", action = clap::ArgAction::SetTrue)]
    lines: Option<bool>,

    #[arg(short, long, help = "gets the number of words in a file", action = clap::ArgAction::SetTrue)]
    words: Option<bool>,

    #[arg(short, long, help = "gets the number of chars in a file", action = clap::ArgAction::SetTrue)]
    mchars: Option<bool>,

    // The path of the file to read
    #[arg(help = "path to file to check")]
    file: Option<std::path::PathBuf>,
}

fn main() {
    let args = CLI::parse();

    let contents = match args.file.clone() {
        Some(path) => std::fs::read_to_string(path).expect("Error reading file"),
        None => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer).expect("failed to read stdin");
            buffer
        }
    };

    let any_provided: bool = !args.count.unwrap() & !args.lines.unwrap() & !args.mchars.unwrap() & !args.words.unwrap();
    
    if args.count.unwrap() {
        print!("{:?} ", contents.as_bytes().iter().count());
    }

    if any_provided | args.lines.unwrap() {
        print!("{:?} ", contents.lines().count());
    }

    if any_provided | args.words.unwrap() {
        print!("{:?} ", contents.split_whitespace().count());
    }

    if any_provided | args.mchars.unwrap() {
        print!("{:?} ", contents.chars().count());
    }

    println!("{:?}", args.file);
}

fn _get_file_size_metadata<P: AsRef<Path>>(path: P) -> io::Result<u64> {
    // get the file metadata
    let metadata = fs::metadata(path)?;
    // get the bytes of the data using .len() property
    Ok(metadata.len())
}
