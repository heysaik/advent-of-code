use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    // Read the input file
    let file_name = "input.txt";
    let file = File::open(&file_name).expect(&format!("Unable to open file: {}", file_name));
    let reader = BufReader::new(file);
    let mut instructions_enabled: bool = true;
    let mut part_one_answer: u32 = 0;
    let mut part_two_answer: u32 = 0;

    for line in reader.lines() {
        match line {
            Ok(input) => {
                // Initially used regex to find all mul instructions for part 1
                // Find all regex matches for "mul(X,Y)" where X and Y are numbers
                // let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
                // let matches = regex.find_iter(&text);
                // for result in matches {
                //     let x: u32 = result.as_str().split(",").nth(0).unwrap().split("mul(").nth(1).unwrap().parse::<u32>().unwrap();
                //     let y: u32 = result.as_str().split(",").nth(1).unwrap().split(")").nth(0).unwrap().parse::<u32>().unwrap();
                //     answer += x * y;
                // }

                // Later on went character by character.
                'outer: for ref mut i in 0..input.len() {
                    if input.get(*i..*i+4) == Some("do()") {
                        instructions_enabled = true;
                        *i += 4;
                    } else if input.get(*i..*i+7) == Some("don't()") {
                        instructions_enabled = false;
                        *i += 7;
                    } else if input.get(*i..*i+4) == Some("mul(") {
                        let mut x: u32 = 0;
                        let mut y: u32 = 0;
            
                        // Find the first number after the 'mul('
                        let mut first_num_index: usize = *i + 4;
                        while input.chars().nth(first_num_index).unwrap() != ',' {
                            if !input.chars().nth(first_num_index).unwrap().is_digit(10) {
                                *i = first_num_index;
                                continue 'outer;
                            }
                            first_num_index += 1;
                        }
                        x = input[*i+4..first_num_index].parse::<u32>().unwrap();
            
                        // Find the next number after the comma
                        let mut second_num_index: usize = first_num_index + 1;
                        while input.chars().nth(second_num_index).unwrap() != ')' {
                            if !input.chars().nth(second_num_index).unwrap().is_digit(10) {
                                *i = second_num_index;
                                continue 'outer;
                            }
                            second_num_index += 1;
                        }
                        y = input[first_num_index+1..second_num_index].parse::<u32>().unwrap();
                        
                        if instructions_enabled {
                            part_two_answer += x * y;
                        }
                        part_one_answer += x * y;
                        *i = second_num_index + 1;
                    }
                }
            }

            Err(err) => {
                eprintln!("Error reading line: {}", err);
            }
        }
    }

    println!("Part 1: {}. Part 2: {}", part_one_answer, part_two_answer);
}