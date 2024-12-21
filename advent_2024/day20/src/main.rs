use std::fs;

const INPUT: &str = "input.txt";

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Start(usize),
    End(usize),
    Wall,
    Empty(usize),
}

fn generate_map(input: &str) -> Vec<Vec<Tile>> {
    let contents = fs::read_to_string(input).unwrap();
    let mut map: Vec<Vec<Tile>> = contents
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '#' => Tile::Wall,
                    '.' => Tile::Empty(0),
                    'S' => Tile::Start(0),
                    'E' => Tile::End(0),
                    _e => panic!("wrong char in map: {_e}"),
                })
                .collect()
        })
        .collect();

    let start = map
        .iter()
        .enumerate()
        .find_map(|(y, l)| {
            l.iter().enumerate().find_map(|(x, t)| {
                if *t == Tile::Start(0) {
                    Some((y, x))
                } else {
                    None
                }
            })
        })
        .unwrap();

    {
        let mut count = 0;
        let mut coord = start;
        loop {
            let tile = &mut map[coord.0][coord.1];

            if *tile == Tile::End(0) {
                *tile = Tile::End(count);
                break;
            }

            if *tile != Tile::Start(0) {
                *tile = Tile::Empty(count);
            }

            for c in [
                (coord.0 + 1, coord.1),
                (coord.0 - 1, coord.1),
                (coord.0, coord.1 + 1),
                (coord.0, coord.1 - 1),
            ] {
                if map[c.0][c.1] == Tile::Empty(0) || map[c.0][c.1] == Tile::End(0) {
                    coord = c;
                    break;
                }
            }

            count += 1;
        }
    };

    map
}

fn task1() {
    let map = generate_map(INPUT);

    let mut shorted_paths = Vec::new();

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            use Tile::*;
            let dist = match map[y][x] {
                Start(s) | End(s) | Empty(s) => s,
                Wall => continue,
            };

            for coord in [
                (y as i64 - 2, x as i64),
                (y as i64 + 2, x as i64),
                (y as i64, x as i64 - 2),
                (y as i64, x as i64 + 2),
            ] {
                if coord.0 >= 0
                    && coord.0 < map.len() as i64
                    && coord.1 >= 0
                    && coord.1 < map[0].len() as i64
                {
                    match map[coord.0 as usize][coord.1 as usize] {
                        Start(s) | End(s) | Empty(s) => {
                            if s > dist && s - dist != 2 {
                                shorted_paths.push(s - dist - 2);
                            }
                        }
                        Wall => (),
                    };
                }
            }
        }
    }

    let best_shortcuts_count = shorted_paths.iter().filter(|d| **d >= 100).count();

    println!("task1:\t{best_shortcuts_count}");
}

fn task2() {
    let map = generate_map(INPUT);

    let mut shorted_paths = Vec::new();

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            use Tile::*;
            let dist = match map[y][x] {
                Start(s) | End(s) | Empty(s) => s,
                Wall => continue,
            };

            for i in -20..21i64 {
                for j in -20..21i64 {
                    let path_len = (i.abs() + j.abs()) as usize;
                    if path_len > 20 {
                        continue;
                    }

                    let coord = (y as i64 + i, x as i64 + j);

                    if !(coord.0 >= 0
                        && coord.0 < map.len() as i64
                        && coord.1 >= 0
                        && coord.1 < map[0].len() as i64)
                    {
                        continue;
                    }

                    match map[coord.0 as usize][coord.1 as usize] {
                        Start(s) | End(s) | Empty(s) => {
                            if s > dist && s - dist != path_len {
                                shorted_paths.push(s - dist - path_len);
                            }
                        }
                        Wall => (),
                    };
                }
            }
        }
    }

    let best_shortcuts_count = shorted_paths.iter().filter(|d| **d >= 100).count();

    println!("task2:\t{best_shortcuts_count}");
}

fn main() {
    task1();
    task2();
}
