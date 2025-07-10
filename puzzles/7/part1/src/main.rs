use pathfinding::prelude::count_paths;
use std::fs;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(u32, u32);

fn main() {
    let grids = parse_input("./src/input.txt");
    let mut total = 0;

    for grid in grids {
        total += amount_of_shortest_paths(grid);
    }

    println!("Total: {total}")
}

fn amount_of_shortest_paths(grid_size: Pos) -> usize {
    count_paths(
        (0, 0),
        |&(x, y)| {
            [(x + 1, y), (x, y + 1)]
                .into_iter()
                .filter(|&(x, y)| x < grid_size.0 && y < grid_size.1)
        },
        |&c| c == (grid_size.0 - 1, grid_size.1 - 1),
    )
}

fn parse_input(filename: &str) -> Vec<Pos> {
    let data = fs::read_to_string(filename).expect("Failed to read file");

    data.lines()
        .map(|line| {
            let splitted: Vec<&str> = line.split(" ").collect();

            let x = splitted[0].parse::<u32>().unwrap();
            let y = splitted[1].parse::<u32>().unwrap();

            Pos(x, y)
        })
        .collect()
}
