mod parser;
mod tokenizer;
mod types;

use std::io::{self};

use clap::Parser;
#[derive(Parser, Debug)]
struct CLI {
    #[arg(help = "This is the json file path")]
    file: Option<std::path::PathBuf>,
}

fn main() {
    let args = CLI::parse();

    // Get the content from the cli
    let json_stringify = match args.file {
        Some(path) => std::fs::read_to_string(path).expect("3 - Failed to read file"),
        None => {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("3 - Failed to read file");
            input
        }
    };

    // Tokenize the stringify json
    let tokens = tokenizer::tokenizer(json_stringify.trim());

    // parse the tokens to json
    parser::parser(tokens);
}
