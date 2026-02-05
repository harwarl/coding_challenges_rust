use clap::{Parser, ValueEnum};
use std::{collections::HashMap, fs::File, io::BufReader, path::PathBuf, time};

mod decode;
mod encode;
mod helpers;
mod huffman;
mod prefix_table;

#[derive(Debug, Clone, Parser)]
#[command(author, version, about, long_about=None)]
pub struct Args {
    #[arg(value_enum)]
    action: Action,
    #[arg(help = "input file path")]
    input: PathBuf,
    #[arg(help = "new output file path")]
    output: PathBuf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Action {
    Encode,
    Decode,
}

// Sample commands
// cargo run -- compress test.txt output.txt
// cargo run -- extract output.txt output.txt

fn main() {
    let args = Args::parse();

    match args.action {
        Action::Encode => {
            let timer: time::Instant = time::Instant::now();
            // load file
            let buf_reader: BufReader<File> = helpers::load_file(args.input);
            // Get the time elapsed to read the file
            let time = timer.elapsed();
            println!("Read source file with in times {time:?}");

            println!("Getting array of occurences...");
            // get the chars and frequency
            let char_map: HashMap<char, i32> = helpers::get_char_occurence(buf_reader);

            // get the huffman tree
            let huffman = huffman::huffman_tree::<char>(char_map);

            println!("{:?}", huffman);

            // Start encoding while it's timed
            let timer = time::Instant::now();
            let enoded = encode::encode();
            let time = timer.elapsed();
            println!("File encoded in {time:?}")
        }
        Action::Decode => {}
    }

    // Convert HashMap to Array so We can sort
    // println!("{:?}", char_map.iter().count());

    // // convert hashmap to an array of tuples and sort the tuples
    // let mut char_vec: Vec<(&char, &u32)> = char_map.iter().collect();
    // sort_vec(&mut char_vec);
}

fn sort_vec(vec_array: &mut Vec<(&char, &u32)>) {
    vec_array.sort_by(|a, b| a.1.cmp(b.1));
}

// Unit Tests
// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn should_store_chars_to_hash_map() {
//         let mut char_map: HashMap<char, u32> = HashMap::new();
//         let line = String::from("This is a new line");
//         add_and_update_char(line, &mut char_map);

//         // assert that number of "i"s is 3
//         assert_eq!(*char_map.get(&'i').unwrap(), 3);
//         assert_eq!(*char_map.get(&'T').unwrap(), 1);
//         assert_eq!(*char_map.get(&'s').unwrap(), 2);
//     }
// }

// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// pub fn add_2(a: u64) -> u64 {
//     a + 2
// }

// #[derive(Debug)]
// struct  Rectangle {
//     width: u32,
//     height: u32
// }

// impl Rectangle {
//     fn area(&mut self) -> u32 {
//         self.width * self.height
//     }

//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }

// #[derive(Debug)]
// pub struct Guess {
//     value: i32
// }

// impl Guess {
//     pub fn new(value: i32) -> Guess {
//         if value < 1  {
//             panic!("Guess value must be greater than or equal to 1, got {value}");
//         }
//         else if value > 100 {
//             panic!("Guess value must be less than or equal to 100, got {value}");
//         }
//         Guess { value }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_adds_two (){
//         let result = add_2(3);
//         assert_eq!(result, 5);
//     }

//     #[test]
//     fn exploration() {
//         let result = add(2, 3);
//         assert_eq!(result, 5, "This should fail because '{result}'");
//     }

//     #[test]
//     #[should_panic]
//     fn will_fail() {
//         panic!("Make this test fail");
//     }

//     #[test]
//     fn larger_can_hold_smaler() {
//         let larger = Rectangle {
//             width: 8,
//             height: 7
//         };

//         let smaller = Rectangle {
//             width: 5,
//             height: 1
//         };

//         assert!(larger.can_hold(&smaller));
//     }

//     #[test]
//     fn smaller_cannor_hold_larger () {
//         let larger = Rectangle {
//             width: 8,
//             height: 7
//         };

//         let smaller = Rectangle {
//             width: 5,
//             height: 1
//         };

//         assert!(!smaller.can_hold(&larger));
//     }

//     #[test]
//     #[should_panic]
//     fn greater_than_100 () {
//         Guess::new(200);
//     }

//     #[test]
//     #[ignore = "for now"]
//     fn it_works() -> Result<(), String> {
//         let result = add(2, 2);

//         if result == 4 {
//             Ok(())
//         } else {
//             Err(String::from("two plus two does not equal four"))
//         }
//     }
// }
