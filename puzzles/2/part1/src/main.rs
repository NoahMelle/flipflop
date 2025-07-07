use std::fs;

fn main() {
    let input = parse_input("./src/input.txt");
    let mut current_height: i32 = 0;
    let mut max_height: i32 = 0;

    for line in input {
        for character in line.chars() {
            if character == '^' {
                current_height += 1;
                if current_height > max_height {
                    max_height = current_height;
                }
            } else if character == 'v' {
                current_height -= 1;
            }
        }
    }

    println!("Max height: {max_height}");
}

fn parse_input(filename: &str) -> Vec<String> {
    let data = fs::read_to_string(filename).expect("Failed to read file");

    data.lines().map(|line| line.to_string()).collect()
}
