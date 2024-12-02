use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    // Read the input file
    let file_name = "input.txt";
    let file = File::open(&file_name).expect(&format!("Unable to open file: {}", file_name));
    let reader = BufReader::new(file);

    let mut unsafe_reports = Vec::<Vec<i32>>::new();
    let mut safe_count = 0;

    for line in reader.lines() {
        match line {
            Ok(text) => {
                // Parse the levels into a vector of integers
                let levels = text.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
                let is_safe = is_safe(&levels);

                // If the report is safe, increment the safe count
                if is_safe {
                    safe_count += 1;
                } else {
                    unsafe_reports.push(levels);
                }
            }

            Err(err) => {
                eprintln!("Error reading line: {}", err);
            }
        }
    }

    // Part 2
    for report in unsafe_reports {
        // Check if removing a single level from the report would make it safe
        for i in 0..report.len() {
            let mut new_report = report.clone();
            new_report.remove(i);

            if is_safe(&new_report) {
                safe_count += 1;
                break;
            }
        }
    }

    println!("Number of safe reports: {}", safe_count);
}

fn is_safe(levels: &Vec<i32>) -> bool {
    // Check each level
    let mut is_increasing = true;
    let mut is_safe = true;
    for i in 0..levels.len() - 1 {
        // First check if we are increasing or decreasing based on the first two levels
        if i == 0 {
            if levels[i] > levels[i + 1] {
                is_increasing = false;
            }
        }

        // Checking if the next level is not following the increasing or decreasing pattern
        if is_increasing && levels[i] >= levels[i + 1] {
            is_safe = false;
            break;
        }

        if !is_increasing && levels[i] <= levels[i + 1] {
            is_safe = false;
            break;
        }

        // Checking if any two adjacent levels differ by at least one and at most three
        if (levels[i] - levels[i + 1]).abs() < 1 || (levels[i] - levels[i + 1]).abs() > 3 {
            is_safe = false;
            break;
        }
    }

    return is_safe;
}
