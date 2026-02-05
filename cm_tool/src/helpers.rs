use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn load_file<P: AsRef<Path>>(path: P) -> BufReader<File> {
    // Open file in path
    let file = File::open(path).expect("Error, file not found");
    // create a new buffer reader
    BufReader::new(file)
}

pub fn get_char_occurence(reader: BufReader<File>) -> HashMap<char, i32> {
    // char hashMap
    let mut char_map: HashMap<char, i32> = HashMap::new();
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
