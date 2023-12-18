use priority_queue::PriorityQueue;
use std::collections::HashMap;
use std::iter::zip;
use std::{fs, usize};

const INPUT: &str = "input.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Tile {
    curr_dir: Direction,
    x: i64,
    y: i64,
    forward_steps: u8,
    cooldown_sum: i64,
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
                let steps = if d == self.curr_dir {
                    self.forward_steps + 1
                } else {
                    0
                };
                if steps == 3 {
                    return None;
                }

                let cooldown_sum = self.cooldown_sum + map.map[c.1 as usize][c.0 as usize] as i64;

                Some(Tile {
                    curr_dir: d,
                    x: c.0,
                    y: c.1,
                    forward_steps: steps,
                    cooldown_sum,
                })
            })
            .collect::<Vec<_>>();

        new_tiles
    }
}

fn a_star(tile_start: &Tile, end_coord: (i64, i64), map: &Map) -> Tile {
    let mut best: HashMap<(i64, i64, u8, Direction), Tile> = HashMap::new();

    let mut pq: PriorityQueue<Tile, i64> = PriorityQueue::new();
    pq.push(*tile_start, -tile_start.cooldown_sum);
    while let Some((tile, _)) = pq.pop() {
        let new_tiles = tile.new_tiles(map);
        for tile in new_tiles {
            if tile.x == end_coord.0 && tile.y == end_coord.1 {
                return tile;
            }
            if let Some(better) = best.get(&(tile.x, tile.y, tile.forward_steps, tile.curr_dir)) {
                if better.cooldown_sum < tile.cooldown_sum {
                    continue;
                }
            }
            pq.push(tile, -tile.cooldown_sum);
            best.insert(
                (tile.x, tile.y, tile.forward_steps, tile.curr_dir),
                tile.clone(),
            );
        }
    }

    panic!("Solution not found!");
}

fn task1() {
    let map = Map::new(INPUT);
    let start = (0, 0);
    let end_point = (map.width as i64 - 1, map.height as i64 - 1);
    let tile_start = Tile {
        curr_dir: Direction::Right,
        x: start.0,
        y: start.1,
        forward_steps: 0,
        cooldown_sum: 0,
    };
    let best_tile = a_star(&tile_start, end_point, &map);

    println!("task1 {:?}", best_tile.cooldown_sum);
}

fn main() {
    task1();
}
