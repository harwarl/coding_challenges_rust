use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::{self, Path},
};

use clap::Parser;

#[derive(Debug, Clone, Parser)]
pub struct CLI {
    #[arg(help = "name of the file")]
    file: Option<path::PathBuf>,
}

fn main() {
    let args = CLI::parse();

    // check if file exists
    let file_path = match args.file {
        Some(path) => path,
        None => {
            panic!("Error, file name should be passed")
        }
    };

    let char_map: HashMap<char, i32> = load_file_and_get_char_occurence(file_path);

    println!("{:?}", char_map.get(&'t').or(Some(&0)))
}

fn load_file_and_get_char_occurence<P: AsRef<Path>>(path: P) -> HashMap<char, i32> {
    // Open file in path
    let file = File::open(path).expect("Error, file not found");
    // create a new buffer reader
    let reader = BufReader::new(file);
    // char hashMap
    let mut char_map: HashMap<char, i32> = HashMap::new();

    // Read the file line by line
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        // TODO: save the chars into an HashMap
        for char in line.chars() {
            // Store char in hashmap
            if char_map.contains_key(&char) {
                // Increment the value, first get the old value
                *char_map.entry(char).or_insert(0) += 1;
            } else {
                char_map.insert(char, 1);
            }
        }
    }

    char_map
}
