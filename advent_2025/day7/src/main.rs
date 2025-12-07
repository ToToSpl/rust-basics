use std::{collections::HashMap, fs};

const INPUT: &str = "input.txt";
// const INPUT: &str = "input.test.txt";

enum MapState {
    Start,
    Empty,
    Beam,
    Splitter,
}

fn load_map() -> Vec<Vec<MapState>> {
    fs::read_to_string(INPUT)
        .unwrap()
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    'S' => MapState::Start,
                    '.' => MapState::Empty,
                    '|' => MapState::Beam,
                    '^' => MapState::Splitter,
                    _c => panic!("unknown character {_c}"),
                })
                .collect()
        })
        .collect()
}

fn task1() {
    let mut map = load_map();

    let mut split_count = 0;

    for y in 0..map.len() - 1 {
        for x in 0..map[0].len() {
            match map[y][x] {
                MapState::Start => map[y + 1][x] = MapState::Beam,
                MapState::Beam => match map[y + 1][x] {
                    MapState::Splitter => {
                        map[y + 1][x + 1] = MapState::Beam;
                        map[y + 1][x - 1] = MapState::Beam;
                        split_count += 1;
                    }
                    _ => map[y + 1][x] = MapState::Beam,
                },
                MapState::Empty | MapState::Splitter => {}
            }
        }
    }

    println!("task1:\t{split_count}");
}

fn beam_follow(
    start: (usize, usize),
    map: &Vec<Vec<MapState>>,
    lookup: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    let height = map.len();
    let mut pos = start; // y x

    while pos.0 < height {
        match map[pos.0][pos.1] {
            MapState::Splitter => {
                if let Some(&down_count) = lookup.get(&pos) {
                    return down_count;
                }

                let left = beam_follow((pos.0, pos.1 + 1), map, lookup);
                let right = beam_follow((pos.0, pos.1 - 1), map, lookup);

                lookup.insert(pos, left + right);

                return left + right;
            }
            _ => pos.0 += 1,
        }
    }

    1
}

fn task2() {
    let map = load_map();

    let start_x = map[0]
        .iter()
        .position(|s| matches!(s, MapState::Start))
        .unwrap();

    let mut lookup = HashMap::new();

    let timelines = beam_follow((0, start_x), &map, &mut lookup);

    println!("task2:\t{timelines}");
}

fn main() {
    task1();
    task2();
}
