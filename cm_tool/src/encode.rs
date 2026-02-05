use std::{collections::HashMap, fs::File, io::BufReader, path::PathBuf};

use crate::{generate_huffman_codes::generate_huffman_codes, helpers, huffman::huffman_tree};

pub fn encode_and_output_file(output: PathBuf, buffer: BufReader<File>) {
    // Get the char ocurrences
    let char_map: HashMap<char, u64> = helpers::get_char_occurence(buffer);

    // get the huffman tree
    let huffman_tree = huffman_tree::<char>(char_map);

    // generate the huffman codes
    let char_codes = generate_huffman_codes(huffman_tree);

    // Write to file
    helpers::write_to_file(output, &char_codes);
}
