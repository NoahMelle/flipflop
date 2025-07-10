use std::fs;

fn main() {
    let grids = parse_input("./src/input.txt");
    let mut total = 0;

    for (dimension, length) in grids {
        let steps = (length - 1) * dimension;
        let count = multinomial(steps, dimension, length - 1);
        total += count;
    }

    println!("Total: {total}")
}

fn parse_input(filename: &str) -> Vec<(u128, u128)> {
    let data = fs::read_to_string(filename).expect("Failed to read file");

    data.lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let x = parts[0].parse::<u128>().unwrap();
            let y = parts[1].parse::<u128>().unwrap();
            (x, y)
        })
        .collect()
}

// got these parts from internet lol
fn multinomial(n: u128, dimension: u128, step_per_dimension: u128) -> u128 {
    let mut result = factorial(n);
    for _ in 0..dimension {
        result /= factorial(step_per_dimension);
    }
    result
}

fn factorial(n: u128) -> u128 {
    let mut acc = 1;
    for i in 2..=n {
        acc *= i;
    }
    acc
}
