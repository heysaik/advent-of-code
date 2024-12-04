fn main() {
    let input: String = std::fs::read_to_string("input.txt").unwrap();
    let rows: Vec<&str> = input.split("\n").collect::<Vec<&str>>();
    let cols: Vec<String> = rows[0].chars().map(|c| c.to_string()).collect::<Vec<String>>();
    let matrix: Vec<Vec<String>> = rows.iter().map(|s| s.chars().map(|c| c.to_string()).collect()).collect::<Vec<Vec<String>>>();
    let mut part1_count: i32 = 0;
    let mut part2_count: i32 = 0;

    let mut current_row: usize = 0;
    let mut current_col: usize = 0;

    // Check horizontally (front and back with "XMAS" or "SAMX")
    while current_row < rows.len() {
        current_col = 0;
        while current_col < cols.len() - "XMAS".len() + 1 {
            if matrix[current_row][current_col..(current_col + "XMAS".len())].join("") == "XMAS" || matrix[current_row][current_col..(current_col + "XMAS".len())].join("") == "SAMX" {
                part1_count += 1;
            }
            current_col += 1;
        }
        current_row += 1;
    }

    println!("Horizontal count: {}", part1_count);

    // Check vertically ("XMAS" or "SAMX")
    current_row = 0;
    while current_row < rows.len() - "XMAS".len() + 1 {
        println!("Current row: {}", current_row);
        current_col = 0;
        while current_col < cols.len() {
            println!("Current col: {}", current_col);
            if (matrix[current_row][current_col] == "X" && matrix[current_row + 1][current_col] == "M" && matrix[current_row + 2][current_col] == "A" && matrix[current_row + 3][current_col] == "S") ||
               (matrix[current_row][current_col] == "S" && matrix[current_row + 1][current_col] == "A" && matrix[current_row + 2][current_col] == "M" && matrix[current_row + 3][current_col] == "X") {
                part1_count += 1;
            }
            current_col += 1;
        }
        current_row += 1;
    }

    println!("Vertical count: {}", part1_count);

    // Check diagonally (right and down "XMAS" or left and up "SAMX")
    current_col = 0;
    while current_col < cols.len() - "XMAS".len() + 1 {
        current_row = 0;
        while current_row < matrix.len() && current_row + "XMAS".len() <= matrix.len() {
            if (matrix[current_row][current_col] == "X" && matrix[current_row + 1][current_col + 1] == "M" && matrix[current_row + 2][current_col + 2] == "A" && matrix[current_row + 3][current_col + 3] == "S") ||
               (matrix[current_row][current_col] == "S" && matrix[current_row + 1][current_col + 1] == "A" && matrix[current_row + 2][current_col + 2] == "M" && matrix[current_row + 3][current_col + 3] == "X") {
                part1_count += 1;
            }
            current_row += 1;
        }
        current_col += 1;
    }

    println!("Diagonal count (right and down): {}", part1_count);

    // Check diagonally (left and down "XMAS" or right and up "SAMX")
    current_col = cols.len() - 1;
    while current_col >= "XMAS".len() - 1 {
        current_row = 0;
        while current_row < matrix.len() - "XMAS".len() + 1 && current_row + "XMAS".len() <= matrix.len() {
            if (matrix[current_row][current_col] == "X" && matrix[current_row + 1][current_col - 1] == "M" && matrix[current_row + 2][current_col - 2] == "A" && matrix[current_row + 3][current_col - 3] == "S") ||
               (matrix[current_row][current_col] == "S" && matrix[current_row + 1][current_col - 1] == "A" && matrix[current_row + 2][current_col - 2] == "M" && matrix[current_row + 3][current_col - 3] == "X") {
                part1_count += 1;
            }
            current_row += 1;
        }
        current_col -= 1;
    }

    println!("Diagonal count (left and down): {}", part1_count);
    println!("Part 1 Count: {}", part1_count);
    
    // Part 2

    println!("Part 2 Count: {}", part2_count);
}
