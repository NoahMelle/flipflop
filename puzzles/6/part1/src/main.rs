use std::fs;

const SKY_SIZE: i32 = 1000;
const SECONDS: i32 = 100;
const FRAME_SIZE: i32 = 500;
const FRAME_START: i32 = (SKY_SIZE - FRAME_SIZE) / 2;
const FRAME_END: i32 = SKY_SIZE - FRAME_START;

fn main() {
    let bird_velocities = parse_input("./src/input.txt");
    let mut total = 0;

    for bird_velocity in bird_velocities {
        let new_x = (bird_velocity.0 * SECONDS).rem_euclid(SKY_SIZE);
        let new_y = (bird_velocity.1 * SECONDS).rem_euclid(SKY_SIZE);

        if new_x >= FRAME_START && new_x < FRAME_END && new_y >= FRAME_START && new_y < FRAME_END {
            total += 1;
        }
    }

    println!("Total: {total}");
}

fn parse_input(filename: &str) -> Vec<(i32, i32)> {
    let data = fs::read_to_string(filename).expect("Failed to read file");

    data.lines()
        .map(|line| {
            let splitted: Vec<&str> = line.split(",").collect();
            let vx = splitted[0].parse::<i32>().unwrap();
            let vy = splitted[1].parse::<i32>().unwrap();

            (vx, vy)
        })
        .collect()
}
