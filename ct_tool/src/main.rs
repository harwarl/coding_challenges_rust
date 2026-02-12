
use std::{io::{self, Read}};

use clap::Parser;
#[derive(Debug, Parser)]
#[command(name = "Cut Tool", version, about = "A tool for cutting things", long_about = None)]
struct Args {
    /// Fields to cut (1 - based)
    #[arg(short, long)]
    field: usize,

    /// Delimiter to use instead of whitespace
    #[arg(short, long)]
    delimiter: Option<String>,

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

    match args.field {
        0 => println!("Field number must be greater than 0"),
        field => {
            let delimiter = match args.delimiter {
                Some(delimiter) => {
                    delimiter
                },
                None => {
                    " ".to_string()
                }
            };
            let result = cut_field_with_delimiter(file_contents, field, &delimiter);
            println!("{}", result);
        }
    }
}


pub fn cut_field_with_delimiter(file_contents: String, field: usize, delimiter: &String) -> String  {
    let mut result = String::new();

    // Print out the selected field from each line
    for line in file_contents.lines() {
        let rows: Vec<&str> = line.split(delimiter).collect();
        if rows.len() >= field {
            result.push_str(rows[field - 1]);
            result.push('\n');
        }
    }

    result
}