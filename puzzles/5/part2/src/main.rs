use std::{collections::HashSet, fs};

fn main() {
    let input = parse_input("./src/input.txt");

    let binding = input[0].clone();
    let tunnels: Vec<&str> = binding.trim().split("").filter(|s| !s.is_empty()).collect();
    let mut current_pos = 0;
    let mut visited: HashSet<String> = HashSet::new();

    while current_pos < tunnels.len() {
        let current_tunnel = &tunnels[current_pos];
        let other_occurence = tunnels
            .iter()
            .enumerate()
            .find(|(i, t)| *i != current_pos && ***t == **current_tunnel)
            .map(|(i, _)| i);

        let ending_pos = other_occurence.unwrap_or(current_pos);

        current_pos = ending_pos + 1;
        visited.insert(current_tunnel.to_string());
    }

    let mut not_visited: Vec<&str> = Vec::new();

    for tunnel in tunnels {
        if !visited.contains(tunnel) && !not_visited.contains(&tunnel) {
            not_visited.push(tunnel);
        }
    }

    println!("Not visited: {}", not_visited.join(""));
}

fn parse_input(filename: &str) -> Vec<String> {
    let data = fs::read_to_string(filename).expect("Failed to read file");

    data.lines().map(|line| line.to_string()).collect()
}
