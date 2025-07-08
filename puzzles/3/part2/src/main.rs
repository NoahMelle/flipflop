use std::{collections::HashMap, fs};

fn main() {
    let input = parse_input("./src/input.txt");
    let mut green_count = 0;

    for line in input {
        let is_green = is_color_green(&line);

        if is_green {
            green_count += 1;
        }
    }

    println!("green: {}", green_count);
}

fn is_color_green(line: &str) -> bool {
    let colors: Vec<u32> = line.split(",").map(|c| c.parse::<u32>().unwrap()).collect();

    !contains_duplicates(&colors) && colors[1] > colors[0] && colors[1] > colors[2]
}

fn parse_input(filename: &str) -> Vec<String> {
    let data = fs::read_to_string(filename).expect("Failed to read file");

    data.lines().map(|line| line.to_string()).collect()
}

fn contains_duplicates(v: &Vec<u32>) -> bool {
    let mut value_counts: HashMap<u32, u32> = HashMap::new();
    for color in v.clone() {
        *value_counts.entry(color).or_insert(0) += 1;
    }
    value_counts.values().any(|&count| count >= 2)
}
