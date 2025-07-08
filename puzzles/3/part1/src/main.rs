use std::{collections::HashMap, fs};

fn main() {
    let input = parse_input("./src/input.txt");
    let mut max: Option<(String, u32)> = None;
    let mut occurrences: HashMap<String, u32> = HashMap::new();

    for line in input {
        let mut entry = *occurrences.get(&line).unwrap_or(&0);

        entry += 1;

        if entry > max.clone().unwrap_or(("".to_string(), 0)).1 {
            max = Some((line.clone(), entry))
        }

        occurrences.insert(line, entry);
    }

    println!("max: {:?}", max);
}

fn parse_input(filename: &str) -> Vec<String> {
    let data = fs::read_to_string(filename).expect("Failed to read file");

    data.lines().map(|line| line.to_string()).collect()
}
