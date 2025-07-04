use std::fs;

fn main() {
    let input = parse_input("./src/input.txt");

    let mut result = 0;

    for line in input {
        result += line;
    }

    println!("Result: {}", result);
}

fn parse_input(filename: &str) -> Vec<i32> {
    let data = fs::read_to_string(filename).expect("Failed to read file");

    data.lines()
        .map(|line| line.parse::<i32>().expect("Invalid number"))
        .collect()
}
