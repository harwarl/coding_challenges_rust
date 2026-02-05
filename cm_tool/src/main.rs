use clap::{Parser, ValueEnum};
use std::{collections::HashMap, fs::File, io::BufReader, path::PathBuf, time};

mod decode;
mod encode;
mod generate_huffman_codes;
mod helpers;
mod huffman;

#[derive(Debug, Clone, Parser)]
#[command(author, version, about, long_about=None)]
pub struct Args {
    #[arg(value_enum)]
    action: Action,
    #[arg(help = "input file path")]
    input: PathBuf,
    #[arg(help = "new output file path")]
    output: PathBuf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Action {
    Encode,
    Decode,
}

// Sample commands
// cargo run -- compress test.txt output.txt
// cargo run -- extract output.txt output.txt

fn main() {
    let args = Args::parse();

    match args.action {
        Action::Encode => {
            let timer: time::Instant = time::Instant::now();
            // load file
            let buf_reader: BufReader<File> = helpers::load_file(args.input);
            // Get the time elapsed to read the file
            let time = timer.elapsed();
            println!("Read source file with in times {time:?}");

            // Start encoding while it's timed
            let timer = time::Instant::now();
            encode::encode_and_output_file(args.output, buf_reader);
            let time = timer.elapsed();
            
            println!("File encoded in {time:?}")
        }
        Action::Decode => {}
    }
}
