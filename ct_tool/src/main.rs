use std::io::{self, Read};

use clap::Parser;
#[derive(Debug, Parser)]
#[command(name = "Cut Tool", version, about = "A tool for cutting things", long_about = None)]
struct Args {
    /// Fields to cut (1 - based)
    #[arg(short, long)]
    field: String,

    /// Delimiter to use instead of whitespace
    #[arg(short, long)]
    delimiter: Option<char>,

    /// path of file to read ,if not provided, it will read from stdin
    #[arg(help = "path to file to check")]
    file: Option<std::path::PathBuf>,
}

fn main() {
    let args = Args::parse();

    let file_contents = match args.file {
        Some(path) => {
            // Read in file as string
            std::fs::read_to_string(path).expect("Error reading file")
        }
        None => {
            let mut buffer = String::new();
            io::stdin()
                .read_to_string(&mut buffer)
                .expect("Failed to read line");
            buffer
        }
    };

    match args.field {
        field => {
            // clean the field options and get a vector of field numbers
            let field_vec = clean_field_options(field);
            let result = match args.delimiter {
                Some(delimiter) => {
                    cut_field_with_delimiter(file_contents, field_vec, &delimiter)
                }
                None => cut_field(file_contents, field_vec),
            };
            println!("{}", result);
        }
    }
}

// cleans the field options and returns a vector of usize represeting the field numbers
// i.e -f1,2  prints col 1 and 2, -f1-3 prints col 1,2,3 and -f"1 2" prints col 1 and 2
fn clean_field_options(field: String) -> Vec<usize> {
    let mut result = Vec::new();

    // case 1,2 - split by comma and parse
    if field.contains(",") {
        // split the field and add to result
        for f in field.split(",") {
            match f.parse::<usize>() {
                Ok(num) => result.push(num),
                Err(_) => println!("Invalid field number: {}", f),
            }
        }
    } else {
        // split by whitespace and add to result
        for f in field.split_whitespace() {
            match f.parse::<usize>() {
                Ok(num) => result.push(num),
                Err(_) => println!("Invalid field number: {}", f),
            }
        }
    }

    result
}

fn cut_field(file_contents: String, field: Vec<usize>) -> String {
    let mut result = String::new();

    for line in file_contents.lines() {
        let rows: Vec<&str> = line.split_whitespace().collect();
        // iterate through the field vector
        if rows.len() >= field.len() {
            for f in &field {
                if *f <= rows.len() {
                    result.push_str(rows[*f - 1]);
                    result.push('\t');
                }
            }
            result.push('\n');
        }
    }

    result
}

fn cut_field_with_delimiter(file_contents: String, field: Vec<usize>, delimiter: &char) -> String  {
    let mut result = String::new();
    // Print out the selected field from each line
    for line in file_contents.lines() {
        let rows: Vec<&str> = line.split(*delimiter).collect();
        // iterate through the field vector
        if rows.len() >= field.len() {
            let mut printed = 0;
            let total = field.len() ;

            for f in &field {
                if *f <= rows.len() {
                    printed += 1;
                    result.push_str(rows[*f - 1]);
                    if printed < total {
                        result.push(*delimiter);
                    } else {
                        printed = 0;
                    }
                }
            }
            result.push('\n');
        }
    }

    result
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_field_options() {
        assert_eq!(clean_field_options("1 2".to_string()), vec![1, 2]);
        assert_eq!(clean_field_options("1".to_string()), vec![1]);
        assert_eq!(clean_field_options("1,2,3".to_string()), vec![1, 2, 3]);
        assert_eq!(clean_field_options("1 2".to_string()), vec![1, 2]);
        assert_eq!(clean_field_options("1 3".to_string()), vec![1, 3]);
    }
}
