use std::fs;

fn main() {
    let input = parse_input("./src/input.txt");

    let mut result = 0;

    for line in &input {
        result += line;
    }

    println!("Result: {:.2}", result as f32 / input.len() as f32);
}

fn parse_input(filename: &str) -> Vec<i32> {
    let data = fs::read_to_string(filename).expect("Failed to read file");

    data.lines()
        .map(|line| line.parse::<i32>().expect("Invalid number"))
        .collect()
}
