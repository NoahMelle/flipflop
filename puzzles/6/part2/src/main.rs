use std::fs;

const SKY_SIZE: i64 = 1000;
const FRAME_SIZE: i64 = 500;
const FRAME_START: i64 = (SKY_SIZE - FRAME_SIZE) / 2;
const FRAME_END: i64 = FRAME_START + FRAME_SIZE;
const NUM_PICTURES: i64 = 1000;
const SECONDS_PER_PICTURE: i64 = 3600;

fn main() {
    let bird_velocities = parse_input("./src/input.txt");

    let mut total = 0;

    for i in 0..NUM_PICTURES {
        // time since start of sim
        let t = i * SECONDS_PER_PICTURE;
        let mut count = 0;

        for &(vx, vy) in &bird_velocities {
            // rem euclid is like modulo but automatically wraps around
            let x = ((vx * t).rem_euclid(SKY_SIZE)) as i64;
            let y = ((vy * t).rem_euclid(SKY_SIZE)) as i64;

            if x >= FRAME_START && x < FRAME_END && y >= FRAME_START && y < FRAME_END {
                count += 1;
            }
        }

        total += count;
    }

    println!("total: {total}");
}

fn parse_input(filename: &str) -> Vec<(i64, i64)> {
    let data = fs::read_to_string(filename).expect("Failed to read file");

    data.lines()
        .map(|line| {
            let mut parts = line.split(',');

            let vx = parts.next().unwrap().parse::<i64>().unwrap();
            let vy = parts.next().unwrap().parse::<i64>().unwrap();

            (vx, vy)
        })
        .collect()
}
