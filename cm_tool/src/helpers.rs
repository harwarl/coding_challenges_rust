use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Write},
    path::{Path, PathBuf},
};

use bitvec::prelude::*;

struct BitWriter {
    buffer: u8,
    count: u8,
}

impl BitWriter {}

pub fn load_file<P: AsRef<Path>>(path: P) -> BufReader<File> {
    // Open file in path
    let file = File::open(path).expect("Error, file not found");
    // create a new buffer reader
    BufReader::new(file)
}

pub fn get_char_occurence(reader: BufReader<File>) -> HashMap<char, u64> {
    // char hashMap
    let mut char_map: HashMap<char, u64> = HashMap::new();
    // Read the file line by line
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        // save the chars into an HashMap
        for ch in line.chars() {
            // Store char in hashmap
            *char_map.entry(ch).or_insert(0) += 1;
        }
    }
    char_map
}

pub fn write_header_to_file<P: AsRef<Path>>(output: P, codes: &HashMap<char, String>) {
    // Create the file
    // Create a header section
    // Demarcate the header and the body with a new line with Text
    // Write the char and code in the header section
    let mut file = File::create(output).expect("Error Creating file");

    for (ch, code) in codes {
        // Write the code to the file
        write!(file, "{} -> {}\n", ch, code).expect("Error writing to file");
    }

    write!(file, "huffman encoding starts").expect("Error writing to file");
}

pub fn encoder(reader: BufReader<File>, codes: &HashMap<char, String>, output_path: &PathBuf) {
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&output_path)
        .expect("Failed to open input file");
    let mut writer = BufWriter::new(file);
    // create a bitvec holder
    let mut compressed_bits = BitVec::<u8, Msb0>::new();

    for line in reader.lines() {
        let line = line.expect("failed to read line");
        // re add the new line striped by .lines()
        // let line_with_nl = line + "\n";

        for ch in line.chars() {
            let code_str = codes
                .get(&ch)
                .unwrap_or_else(|| panic!("char '{}' not in Huffman Tree", ch));

            // convert code_str to actual bits
            for bit_char in code_str.chars() {
                compressed_bits.push(bit_char == '1');
            }
        }
    }
    // writer
    //     .write_all(compressed_bits.as_raw_slice())
    //     .expect("Failed to write to file");
    // writer.flush().expect("Failed to flush writer");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_to_file() {
        let mut char_map: HashMap<char, String> = HashMap::new();
        char_map.insert('a', "00".to_string());

        write_header_to_file("test_output.txt", &char_map);

        // read the file from the disk and assert contents
        let file = File::open("test_output.txt").expect("Error Opening file");
        let buf_reader = BufReader::new(file);
        let mut lines = buf_reader.lines();

        assert_eq!(lines.next().unwrap().unwrap(), "a -> 00");
    }
}
