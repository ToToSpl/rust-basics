use priority_queue::PriorityQueue;
use std::collections::HashSet;
use std::iter::zip;
use std::{fs, usize};

const INPUT: &str = "input.test.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Direction {
        use Direction::*;
        match *self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
}

struct Map {
    width: usize,
    height: usize,
    map: Vec<Vec<u8>>,
}

impl Map {
    fn new(input: &str) -> Map {
        let contents = fs::read_to_string(input).unwrap();
        let map = contents
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let height = map.len();
        let width = map[0].len();

        Map { map, height, width }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Tile {
    curr_dir: Direction,
    x: i64,
    y: i64,
    forward_steps: u8,
    cooldown_sum: i64,
    f_factor: i64,
    path: Vec<(i64, i64)>,
}

impl Tile {
    fn new_tiles(&self, map: &Map) -> Vec<Tile> {
        use Direction::*;
        let new_dirs = match self.curr_dir {
            Up => vec![Up, Left, Right],
            Down => vec![Down, Left, Right],
            Left => vec![Left, Up, Down],
            Right => vec![Right, Up, Down],
        };

        let new_coords = new_dirs
            .iter()
            .map(|d| match d {
                Up => (self.x, self.y - 1),
                Down => (self.x, self.y + 1),
                Left => (self.x - 1, self.y),
                Right => (self.x + 1, self.y),
            })
            .collect::<Vec<_>>();

        let new_tiles = zip(new_dirs, new_coords)
            .filter_map(|(d, c)| {
                if c.0 < 0 || c.0 >= map.width as i64 || c.1 < 0 || c.1 >= map.height as i64 {
                    return None;
                }
                if d == self.curr_dir && self.forward_steps > 2 {
                    return None;
                }
                let steps = if d == self.curr_dir {
                    self.forward_steps + 1
                } else {
                    1
                };

                let cooldown_sum = self.cooldown_sum + map.map[c.1 as usize][c.0 as usize] as i64;
                let distance = (map.width as i64 - 1) - c.0 + (map.height as i64 - 1) - c.1;
                let mut path = self.path.clone();
                path.push(c);

                Some(Tile {
                    curr_dir: d,
                    x: c.0,
                    y: c.1,
                    forward_steps: steps,
                    cooldown_sum,
                    f_factor: cooldown_sum, // + distance,
                    path,
                })
            })
            .collect::<Vec<_>>();

        new_tiles
    }
}

fn a_star(tile_start: &Tile, end_coord: (i64, i64), map: &Map) -> Tile {
    let mut best: Vec<Vec<Option<Tile>>> = vec![vec![None; map.width]; map.height];
    best[tile_start.y as usize][tile_start.x as usize] = Some(tile_start.clone());

    let mut pq: PriorityQueue<Tile, i64> = PriorityQueue::new();
    pq.push(tile_start.clone(), -tile_start.f_factor);
    while let Some((tile, _)) = pq.pop() {
        let new_tiles = tile.new_tiles(map);
        for tile in &new_tiles {
            if tile.x == end_coord.0 && tile.y == end_coord.1 {
                return tile.clone();
            }
            // if let Some((better, _)) = pq.get(tile) {
            //     let can_compare = if better.curr_dir == tile.curr_dir {
            //         tile.forward_steps <= better.forward_steps
            //     } else {
            //         better.curr_dir.opposite() != tile.curr_dir
            //     };
            //     if can_compare && better.f_factor < tile.f_factor {
            //         continue;
            //     }
            // }
            if let Some(better) = &best[tile.y as usize][tile.x as usize] {
                let can_compare = if better.curr_dir == tile.curr_dir {
                    tile.forward_steps <= better.forward_steps
                } else {
                    better.curr_dir.opposite() != tile.curr_dir
                };
                if can_compare && better.f_factor < tile.f_factor {
                    continue;
                }
            }
            best[tile.y as usize][tile.x as usize] = Some(tile.clone());
            pq.push(tile.clone(), -tile.f_factor);
        }
    }
    return best[end_coord.1 as usize][end_coord.0 as usize]
        .clone()
        .unwrap();

    // panic!("Solution not found!");
}

fn task1() {
    let map = Map::new(INPUT);
    let start = (0, 0);
    let end_point = (map.width as i64 - 1, map.height as i64 - 1);
    let tile_start = Tile {
        curr_dir: Direction::Right,
        x: start.0,
        y: start.1,
        forward_steps: 1,
        cooldown_sum: 0,
        f_factor: 0, //(map.width - 1) as i64 - start.0 + (map.height - 1) as i64 - start.1,
        path: vec![start],
    };
    let best_tile = a_star(&tile_start, end_point, &map);

    let mut set = HashSet::new();
    best_tile.path.iter().for_each(|c| {
        set.insert(c);
    });

    println!("");
    map.map.iter().enumerate().for_each(|(y, l)| {
        l.iter().enumerate().for_each(|(x, v)| {
            if set.contains(&(x as i64, y as i64)) {
                print!("\x1b[31m#\x1b[0m");
            } else {
                print!("{:}", v);
            }
        });
        println!("");
    });

    println!("task1 {:?}", best_tile.cooldown_sum);
}

fn main() {
    task1();
    // task2();
}
