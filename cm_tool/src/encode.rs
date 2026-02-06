use std::{collections::HashMap, fs::File, io::BufReader, path::PathBuf};

use crate::{generate_huffman_codes::generate_huffman_codes, helpers, huffman::huffman_tree};

pub fn encode_and_output_file(output_path: &PathBuf, input_path: &PathBuf) {
    // Open the file
    let file = File::open(&input_path).expect("Failed to open input file");
    let reader = BufReader::new(file);

    // Get the char ocurrences
    let char_map: HashMap<char, u64> = helpers::get_char_occurence(reader);

    // get the huffman tree
    let huffman_tree = huffman_tree::<char>(char_map);

    // generate the huffman codes
    let char_codes = generate_huffman_codes(huffman_tree);

    // Write header char codes to file
    helpers::write_header_to_file(output_path, &char_codes);

    // Write the main content to file (encoded with huffman codes)
    // Open the file
    let file = File::open(&input_path).expect("Failed to open input file");
    let reader = BufReader::new(file);

    helpers::encoder(reader, &char_codes, output_path);
}
