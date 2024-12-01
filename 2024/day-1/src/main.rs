use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    // Initialize variables
    let mut total_distance = 0;
    let mut total_similarity_score = 0;

    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();
    let mut right_frequency_map: HashMap<i32, i32> = HashMap::new();

    // Read the input file and parse the left and right lists
    let file_name = "input.txt";
    let file = File::open(&file_name).expect(&format!("Unable to open file: {}", file_name));
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(text) => {
                let (left, right) = text.split_once("   ").unwrap();
                left_list.push(left.parse::<i32>().unwrap());
                right_list.push(right.parse::<i32>().unwrap());
        
                // Count the frequency of each number in the right list
                *right_frequency_map.entry(right.parse::<i32>().unwrap()).or_insert(0) += 1;
            }

            Err(err) => {
                eprintln!("Error reading line: {}", err);
            }
        }
    }

    // Sort the left and right lists
    left_list.sort();
    right_list.sort();

    // Calculate the total distance and similarity score
    for i in 0..left_list.len() {
        total_distance += (left_list[i] - right_list[i]).abs();
        total_similarity_score += left_list[i] * right_frequency_map.get(&left_list[i]).unwrap_or(&0);
    }

    // Print the total distance and similarity score
    println!("total_distance: {}", total_distance);
    println!("total_similarity_score: {}", total_similarity_score);
}
