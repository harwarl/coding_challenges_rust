use std::{collections::HashMap, fs::{File, OpenOptions}, io::{BufReader, Write}, path::PathBuf};

use bitvec::{order::Msb0, vec::BitVec, view::BitView};

use crate::{EncodedData, helpers::{self, load_file, read_file}};

pub fn decode_and_output_file(output_path: &PathBuf, input_path: &PathBuf) {
    // Open the file
    let file_content = read_file(input_path);

    // decode the struct 
    let decoded_struct : EncodedData = rmp_serde::from_slice(&file_content).expect("Failed to decode struct");

    // Get the data
    let codes_map: HashMap<BitVec<u8, Msb0>, char> = decoded_struct.codes.into_iter().map(|(c, s)| {
        let mut bv = BitVec::<u8, Msb0>::new();
        for c in s.chars() {
            bv.push(c == '1');
        }
        (bv, c)
    }).collect();

    let total_bits = decoded_struct.total_bits;
    let bits = decoded_struct.data.view_bits::<Msb0>();

    // Loop to get the main message
    let mut decoded_text = String::new();
    let mut candidate = BitVec::<u8, Msb0>::new();

    for i in 0..total_bits {
        let bit = bits[i];
        candidate.push(bit);

        if let Some(&ch) = codes_map.get(&candidate) {
            decoded_text.push(ch);
            candidate.clear();
        }
    }

    // Write to the output file
    let mut output_file = OpenOptions::new().create(true).write(true).append(true).open(output_path).expect("Error setting output_file");

    output_file.write_all(decoded_text.as_bytes()).expect("error writing to file");
}
