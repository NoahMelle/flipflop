use std::fs;

fn main() {
    let grids = parse_input("./src/input.txt");
    let mut total = 0;

    for grid in grids {
        let z = grid.0;
        let count = count_3d_paths(grid.0, grid.1, z);
        total += count;
    }

    println!("Total: {total}")
}

fn parse_input(filename: &str) -> Vec<(u64, u64)> {
    let data = fs::read_to_string(filename).expect("Failed to read file");

    data.lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let x = parts[0].parse::<u64>().unwrap();
            let y = parts[1].parse::<u64>().unwrap();
            (x, y)
        })
        .collect()
}

// got these parts from internet lol
fn count_3d_paths(x: u64, y: u64, z: u64) -> u64 {
    let dx = (x - 1) as u64;
    let dy = (y - 1) as u64;
    let dz = (z - 1) as u64;

    let total = dx + dy + dz;

    multinomial(total, &[dx, dy, dz])
}

fn multinomial(n: u64, ks: &[u64]) -> u64 {
    let mut result = factorial(n);
    for &k in ks {
        result /= factorial(k);
    }
    result
}

fn factorial(n: u64) -> u64 {
    let mut acc = 1;
    for i in 2..=n {
        acc *= i;
    }
    acc
}
