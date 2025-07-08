use std::{cmp, fs};

fn main() {
    let mut input = parse_input("./src/input.txt");

    let mut pos = (0, 0);
    let mut steps = 0;

    sort_by_manhattan_distance(&mut input);

    for new_pos in input {
        let steps_x = (new_pos.0 as i32 - pos.0 as i32).abs() as u32;
        let steps_y = (new_pos.1 as i32 - pos.1 as i32).abs() as u32;

        let diagonal_steps = cmp::min(steps_x, steps_y);
        let straight_steps = cmp::max(steps_x, steps_y) - diagonal_steps;

        steps += diagonal_steps + straight_steps;
        pos = new_pos;
    }

    println!("Total steps: {}", steps);
}

fn parse_input(filename: &str) -> Vec<(u32, u32)> {
    let data = fs::read_to_string(filename).expect("Failed to read file");

    data.lines()
        .map(|line| {
            let mut parts = line.split(',');
            let first = parts.next().unwrap().trim().parse::<u32>().unwrap();
            let second = parts.next().unwrap().trim().parse::<u32>().unwrap();
            (first, second)
        })
        .collect()
}

fn sort_by_manhattan_distance(points: &mut Vec<(u32, u32)>) {
    points.sort_by_key(|&(x, y)| x + y);
}
