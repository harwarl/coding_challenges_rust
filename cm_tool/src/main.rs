use bitvec::vec::BitVec;
use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};
use std::{ collections::HashMap, path::PathBuf, time};

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


#[derive(Serialize, Deserialize)]
pub struct EncodedData {
    // Huffman code map
    codes: HashMap<char, String>,
    // exact number of bits
    total_bits: usize,
    // Packed Binary bytes
    data: Vec<u8>
}

// Sample commands
// cargo run -- encode test.txt output.txt
// cargo run -- decode output.txt output.txt

fn main() {
    let args = Args::parse();

    match args.action {
        Action::Encode => {
            // Start encoding while it's timed
            let timer = time::Instant::now();
            encode::encode_and_output_file(&args.output, &args.input);
            let time = timer.elapsed();
            println!("File encoded in {time:?}")
        }
        Action::Decode => {
            // Start decoding
            let timer = time::Instant::now();
            decode::decode_and_output_file(&args.output, &args.input);
            let time = timer.elapsed();
            println!("File decoded in {time:?}");
        }
    }
}
