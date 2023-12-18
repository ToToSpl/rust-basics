use std::collections::HashSet;
use std::fs;

const INPUT: &str = "input.test.txt";

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    dir: Direction,
    len: i64,
}

impl Instruction {
    fn new(line: &str) -> Instruction {
        let mut parts = line.split(' ');
        let dir = match parts.next().unwrap() {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _e => panic!("Unknown symbol {:?}", _e),
        };
        let len = parts.next().unwrap().parse::<i64>().unwrap();

        Instruction { dir, len }
    }

    fn new_corrected(line: &str) -> Instruction {
        let raw_string = line.split(' ').collect::<Vec<_>>()[2];
        let mut raw_string = raw_string
            .chars()
            .skip(2)
            .filter(|c| *c != ')')
            .collect::<String>();

        let dir = match raw_string.pop().unwrap() {
            '0' => Direction::Right,
            '1' => Direction::Down,
            '2' => Direction::Left,
            '3' => Direction::Up,
            _e => panic!("Unknown symbol {:?}", _e),
        };

        let len = u64::from_str_radix(raw_string.as_str(), 16).unwrap() as i64;

        Instruction { dir, len }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Occupancy {
    Empty,
    Digged,
    Edge,
}

#[derive(Debug, Clone)]
struct Map {
    map: Vec<Vec<Occupancy>>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(instructions: &Vec<Instruction>) -> Map {
        let mut set: HashSet<(i64, i64)> = HashSet::new();
        let mut curr: (i64, i64) = (0, 0);
        set.insert(curr);
        for instr in instructions {
            for _ in 0..instr.len {
                curr = match instr.dir {
                    Direction::Up => (curr.0, curr.1 - 1),
                    Direction::Down => (curr.0, curr.1 + 1),
                    Direction::Left => (curr.0 - 1, curr.1),
                    Direction::Right => (curr.0 + 1, curr.1),
                };
                set.insert(curr);
            }
        }

        let offset_x = set.iter().min_by_key(|c| c.0).unwrap().0;
        let offset_y = set.iter().min_by_key(|c| c.1).unwrap().1;
        let width = (set.iter().max_by_key(|c| c.0).unwrap().0 - offset_x) as usize + 1;
        let height = (set.iter().max_by_key(|c| c.1).unwrap().1 - offset_y) as usize + 1;

        let mut map = vec![vec![Occupancy::Empty; width]; height];
        for c in &set {
            map[(c.1 - offset_y) as usize][(c.0 - offset_x) as usize] = Occupancy::Edge;
        }

        let mut to_fill: Vec<(i64, i64)> = Vec::new();
        // assumption that field right down to start is filled
        // (works for me, dont want to check inside/outside)
        to_fill.push((1 - offset_x, 1 - offset_y));
        while let Some(coord) = to_fill.pop() {
            [(0, 1), (0, -1), (1, 0), (-1, 0)]
                .iter()
                .map(|c| (coord.0 + c.0, coord.1 + c.1))
                .filter(|c| c.0 >= 0 && c.0 < width as i64 && c.1 >= 0 && c.1 < height as i64)
                .for_each(|c| {
                    let tile = map[c.1 as usize][c.0 as usize];
                    if tile == Occupancy::Empty {
                        map[c.1 as usize][c.0 as usize] = Occupancy::Digged;
                        to_fill.push(c);
                    }
                });
        }

        Map { map, width, height }
    }
}

fn task1() {
    let contents = fs::read_to_string(INPUT).unwrap();
    let instructions = contents
        .lines()
        .map(|l| Instruction::new(l))
        .collect::<Vec<_>>();

    let map = Map::new(&instructions);

    let filled = map
        .map
        .iter()
        .flatten()
        .filter(|o| **o != Occupancy::Empty)
        .count();

    println!("task1 {:?}", filled);
}

fn task2() {
    let contents = fs::read_to_string(INPUT).unwrap();
    let instructions = contents
        .lines()
        .map(|l| Instruction::new_corrected(l))
        .collect::<Vec<_>>();

    for instr in instructions {
        println!("{:?}", instr);
    }

    // let map = Map::new(&instructions);
    //
    // let filled = map
    //     .map
    //     .iter()
    //     .flatten()
    //     .filter(|o| **o != Occupancy::Empty)
    //     .count();
    //
    // println!("task2 {:?}", filled);
}

fn main() {
    task1();
    task2();
}
