use std::path::PathBuf;

use crate::helpers::load_file;

pub fn decode_and_output_file(output_path: &PathBuf, input_path: &PathBuf) {
    // Open the file
    let reader = load_file(&input_path);

    // load and store the chars and code in a hash map
}
