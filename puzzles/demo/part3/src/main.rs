use std::{collections::HashMap, fs};

fn main() {
    let mut numbers_map: HashMap<i32, i32> = HashMap::new();
    let mut digits_map: HashMap<i32, i32> = HashMap::new();

    let input = parse_input("./src/input.txt");

    for line in &input {
        let entry = numbers_map.get(line);
        let occurences = entry.unwrap_or(&0) + 1;

        for i in line.to_string().chars() {
            let digit = i.to_digit(10).unwrap() as i32;

            let digit_entry = digits_map.get(&digit).unwrap_or(&0) + 1;

            digits_map.insert(digit, digit_entry);
        }

        numbers_map.insert(*line, occurences);
    }

    let highest_number = highest_number_in_hashmap(&numbers_map);
    let lowest_digit = lowest_digit_in_hashmap(&digits_map);

    println!("{}{}", highest_number, lowest_digit);
}

fn parse_input(filename: &str) -> Vec<i32> {
    let data = fs::read_to_string(filename).expect("Failed to read file");

    data.lines()
        .map(|line| line.parse::<i32>().expect("Invalid number"))
        .collect()
}

fn highest_number_in_hashmap(map: &HashMap<i32, i32>) -> i32 {
    let highest_number = map.values().max().unwrap_or(&0);

    let mut max_key: Option<i32> = None;

    for (key, value) in map.iter() {
        if value == highest_number {
            max_key = Some(*key)
        }
    }

    max_key.unwrap_or(0)
}

fn lowest_digit_in_hashmap(map: &HashMap<i32, i32>) -> i32 {
    let lowest_digit = map.values().min().unwrap_or(&0);

    let mut min_key: Option<i32> = None;

    for (key, value) in map.iter() {
        if value == lowest_digit {
            min_key = Some(*key)
        }
    }

    min_key.unwrap_or(0)
}
