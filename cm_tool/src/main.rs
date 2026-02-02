use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::{self, Path, PathBuf},
};

use clap::Parser;

#[derive(Debug, Clone, Parser)]
pub struct Args {
    #[arg(help = "name of the file")]
    input: PathBuf,
}

fn main() {
    let args = Args::parse();

    let char_map: HashMap<char, u32> = load_file_and_get_char_occurence(args.input);

    // Convert HashMap to Array so We can sort
    println!("{:?}", char_map.iter().count());

    // convert hashmap to an array of tuples and sort the tuples
    let mut char_vec : Vec<(&char, &u32)> = char_map.iter().collect();
    sort_vec(&mut char_vec);
}

fn sort_vec(vec_array: &mut Vec<(&char, &u32)>){
    vec_array.sort_by(|a, b| a.1.cmp(b.1));
}



fn load_file_and_get_char_occurence<P: AsRef<Path>>(path: P) -> HashMap<char, u32> {
    // Open file in path
    let file = File::open(path).expect("Error, file not found");
    // create a new buffer reader
    let reader = BufReader::new(file);
    // char hashMap
    let mut char_map: HashMap<char, u32> = HashMap::new();

    // Read the file line by line
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        add_and_update_char(line, &mut char_map);
    }
    char_map
}

fn add_and_update_char(line: String, char_map: &mut HashMap<char, u32>)  {
    // save the chars into an HashMap
    for ch in line.chars() {
        // Store char in hashmap
        *char_map.entry(ch).or_insert(0) += 1;
    }
}

fn build_tree_with_char_frequency() {
    // Organize the hash by weight.
    // remove the first two trees, (ones with the lowest weight)
    // join the two trees to create a new tree whose root has the two trees as children, and weight is the sum of the weight of the two tress
}

// Unit Tests
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_store_chars_to_hash_map() {
        let mut char_map: HashMap<char, u32> = HashMap::new();
        let line = String::from("This is a new line");
        add_and_update_char(line, &mut char_map);

        // assert that number of "i"s is 3
        assert_eq!(*char_map.get(&'i').unwrap(), 3);
        assert_eq!(*char_map.get(&'T').unwrap(), 1);
        assert_eq!(*char_map.get(&'s').unwrap(), 2);
    }
}

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
