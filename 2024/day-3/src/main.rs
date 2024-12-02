use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file_name = "input.txt";
    let file = File::open(&file_name).expect(&format!("Unable to open file: {}", file_name));
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(text) => {
                println!("{}", text);
            }

            Err(e) => {
                eprintln!("Error reading line: {}", e);
            }
        }
    }
}
