use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn main() {
    let file_name = "input.txt";
    let file = File::open(&file_name).expect(&format!("Unable to open file: {}", file_name));
    let reader = BufReader::new(file);
    let mut answer = 0;

    for line in reader.lines() {
        match line {
            Ok(text) => {
                // Find all regex matches for "mul(X,Y)" where X and Y are numbers
                let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
                let matches = regex.find_iter(&text);
                for result in matches {
                    let x = result.as_str().split(",").nth(0).unwrap().split("mul(").nth(1).unwrap().parse::<u32>().unwrap();
                    let y = result.as_str().split(",").nth(1).unwrap().split(")").nth(0).unwrap().parse::<u32>().unwrap();
                    answer += (x * y);
                }
            }

            Err(e) => {
                eprintln!("Error reading line: {}", e);
            }
        }
    }

    println!("The result adding all the multiplication values is: {answer}");
}
