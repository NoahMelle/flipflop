use std::fs;

fn main() {
    let input = parse_input("./src/input.txt");
    let mut current_height: i32 = 0;
    let mut max_height: i32 = 0;

    for line in input {
        let mut last_char: Option<char> = None;
        let mut streak = 0;

        for character in line.chars() {
            if last_char == Some(character) {
                streak += 1;
            } else {
                streak = 1;
            }

            if character == '^' {
                current_height += streak;
                if current_height > max_height {
                    max_height = current_height;
                }
            } else if character == 'v' {
                current_height -= streak;
            }

            last_char = Some(character);
        }
    }

    println!("Max height: {max_height}");
}

fn parse_input(filename: &str) -> Vec<String> {
    let data = fs::read_to_string(filename).expect("Failed to read file");

    data.lines().map(|line| line.to_string()).collect()
}
