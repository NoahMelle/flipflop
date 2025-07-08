use std::{collections::HashMap, fs};

#[derive(Hash, Eq, PartialEq, Debug)]
enum Color {
    Blue,
    Red,
    Green,
    Special,
}

fn main() {
    let input = parse_input("./src/input.txt");
    let mut total = 0;

    for line in input {
        let color = get_color(&line);

        match color {
            Color::Red => total += 5,
            Color::Green => total += 2,
            Color::Blue => total += 4,
            Color::Special => total += 10,
        }
    }

    println!("total: {:?}", total);
}

fn get_color(line: &str) -> Color {
    let colors: Vec<&str> = line.split(",").collect();
    let mut max_value: Option<(Color, u32)> = None;

    if contains_duplicates(&colors) {
        return Color::Special;
    }

    for (index, color) in colors.iter().enumerate() {
        let value = color.parse::<u32>().unwrap();

        if let Some((ref _max_color, max_val)) = max_value {
            if value > max_val {
                let color_variant = match index {
                    0 => Color::Red,
                    1 => Color::Green,
                    2 => Color::Blue,
                    _ => continue,
                };

                max_value = Some((color_variant, value));
            }
        } else {
            let color_variant = match index {
                0 => Color::Red,
                1 => Color::Green,
                2 => Color::Blue,
                _ => continue,
            };

            max_value = Some((color_variant, value));
        }
    }

    max_value.unwrap_or((Color::Special, 0)).0
}

fn parse_input(filename: &str) -> Vec<String> {
    let data = fs::read_to_string(filename).expect("Failed to read file");

    data.lines().map(|line| line.to_string()).collect()
}

fn contains_duplicates(v: &Vec<&str>) -> bool {
    let mut value_counts: HashMap<u32, u32> = HashMap::new();
    for color in v.clone() {
        let value = color.parse::<u32>().unwrap();
        *value_counts.entry(value).or_insert(0) += 1;
    }
    value_counts.values().any(|&count| count >= 2)
}
