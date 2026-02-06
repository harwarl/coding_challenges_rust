use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    hash::Hash,
    io::{BufRead, BufReader, BufWriter, Lines, Write},
    path::{Path, PathBuf},
};

use bitvec::prelude::*;

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
        write!(file, "{}:{}\n", ch, code).expect("Error writing to file");
    }

    write!(file, "encoding\n").expect("Error writing to file");
}

pub fn encoder(reader: BufReader<File>, codes: &HashMap<char, String>, output_path: &PathBuf) {
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&output_path)
        .expect("Failed to open input file");
    let mut writer = BufWriter::new(file);
    // create a bitvec holder
    let mut bv: BitVec<u8, Msb0> = BitVec::<u8, Msb0>::new();

    for line in reader.lines() {
        let line = line.expect("failed to read line");
        push_to_bit(&mut bv, line, codes);
    }

    let packed_bytes = bv.as_raw_slice();
    // write to file
    writer.write_all(packed_bytes).expect("Failed to write");
}

// Converts each lines to bits
pub fn push_to_bit(bv: &mut BitVec<u8, Msb0>, line: String, codes: &HashMap<char, String>) {
    for ch in line.chars() {
        if let Some(bits) = codes.get(&ch) {
            for bit in bits.chars() {
                if bit == '1' {
                    bv.push(true);
                } else {
                    bv.push(false)
                }
            }
        }
    }
}

pub fn decoder(reader: BufReader<File>) -> HashMap<char, String> {
    let lines = reader.lines();
    let mut codes_map: HashMap<char, String> = HashMap::new();
    get_chars_code(lines, &mut codes_map);
    codes_map
}

pub fn get_chars_code(lines: Lines<BufReader<File>>, codes: &mut HashMap<char, String>) {
    for line in lines {
        let line = line.expect("Could not read line");
        if line == "encode\n" { // Break when line gets to "encode"
            break;
        };
        get_codes(&line, codes);
    }
}

fn get_codes(line: &String, codes: &mut HashMap<char, String>) {
    let parts: Vec<&str> = line.split(":").collect();
    if parts.len() == 2 {
        // get the char
        let ch = parts[0].chars().next().unwrap();
        // get the code
        let bit_string = parts[1].to_string();
        codes.insert(ch, bit_string);
    }
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

    #[test]
    fn test_push_to_bits() {
        let line = String::from("abcaba");
        let codes = HashMap::from([
            ('a', "0".to_string()),
            ('b', "10".to_string()),
            ('c', "11".to_string()),
        ]);

        let mut bv = BitVec::<u8, Msb0>::new();
        push_to_bit(&mut bv, line, &codes);
        let packed_bytes = bv.as_raw_slice();

        // abcaba into bits - 01011010 00000000
        // 01011010 binary - 90 (decimals)
        // 00000000 binary - 0 (decimals)
        assert_eq!(packed_bytes[0], 90);
        assert_eq!(packed_bytes[1], 0);
    }

    #[test]
    fn test_get_char_code() {
        let line = "A:1010101".to_string();
        let mut codes = HashMap::<char, String>::new();
        get_codes(&line, &mut codes);
        assert_eq!(codes.get(&'A').unwrap().to_string(), "1010101");
    }
}
