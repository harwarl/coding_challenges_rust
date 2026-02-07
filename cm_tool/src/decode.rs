use std::{collections::HashMap, fs::File, io::BufReader, path::PathBuf};

use crate::helpers::{self, load_file};

pub fn decode_and_output_file(output_path: &PathBuf, input_path: &PathBuf) {
    // Open the file
    let reader: BufReader<File> = load_file(&input_path);

    // load and store the chars and code in a hash map
    let codes_char: HashMap<String, char> = helpers::decoder(reader);
}
