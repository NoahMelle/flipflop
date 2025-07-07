use std::fs;

fn main() {
    let input = parse_input("./src/input.txt");
    let possibilities = ["ba", "na", "ne"];
    let mut total = 0;

    for line in input {
        for i in 0..line.len() - 1 {
            let substr = &line[i..i + 2];

            if possibilities.contains(&substr) {
                total += 1;
            }
        }
    }

    println!("Total: {total}");
}

fn parse_input(filename: &str) -> Vec<String> {
    let data = fs::read_to_string(filename).expect("Failed to read file");

    data.lines().map(|line| line.to_string()).collect()
}
