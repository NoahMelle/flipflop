use std::fs;

fn main() {
    let input = parse_input("./src/input.txt");

    let binding = input[0].clone();
    let tunnels: Vec<&str> = binding.trim().split("").filter(|s| !s.is_empty()).collect();
    let mut current_pos = 0;
    let mut steps = 0;

    while current_pos < tunnels.len() {
        let current_tunnel = &tunnels[current_pos];
        let other_occurence = tunnels
            .iter()
            .enumerate()
            .find(|(i, t)| *i != current_pos && ***t == **current_tunnel)
            .map(|(i, _)| i);

        let is_powered = current_tunnel.chars().all(char::is_uppercase);

        let difference_in_steps =
            (current_pos as i32 - other_occurence.unwrap_or(current_pos) as i32).abs();

        if is_powered {
            steps -= difference_in_steps;
        } else {
            steps += difference_in_steps;
        }

        println!("steps: {}", steps);

        let ending_pos = other_occurence.unwrap_or(current_pos);

        current_pos = ending_pos + 1;
    }

    println!("Total steps: {}", steps);
}

fn parse_input(filename: &str) -> Vec<String> {
    let data = fs::read_to_string(filename).expect("Failed to read file");

    data.lines().map(|line| line.to_string()).collect()
}
