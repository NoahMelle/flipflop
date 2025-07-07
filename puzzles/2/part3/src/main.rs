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
                let to_add = if last_char == Some('^') {
                    fib(streak)
                } else {
                    -fib(streak)
                };

                current_height += to_add;

                streak = 1;
            }

            if current_height > max_height {
                max_height = current_height;
            }
            last_char = Some(character);
        }

        let to_add = if last_char == Some('^') {
            fib(streak)
        } else {
            -fib(streak)
        };
        current_height += to_add;

        if current_height > max_height {
            max_height = current_height;
        }
    }

    println!("Max height: {max_height}");
}

fn parse_input(filename: &str) -> Vec<String> {
    let data = fs::read_to_string(filename).expect("Failed to read file");

    data.lines().map(|line| line.to_string()).collect()
}

fn fib(n: i32) -> i32 {
    if n <= 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}
