use std::fs;

use bitflags::bitflags;

const INPUT: &str = "input.txt";

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct Direction: u8 {
        const Empty = 0b0000;
        const Up = 0b0001;
        const Down = 0b0010;
        const Left = 0b0100;
        const Right = 0b1000;
    }
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Empty,
    SplitterHor, // splitter -
    SplitterVer, // splitter |
    Mirror1,     // mirror /
    Mirror2,     // mirror \
}

#[derive(Debug, Clone)]
struct Map {
    tiles: Vec<Vec<Tile>>,
    occupancy: Vec<Vec<Direction>>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(input: &str) -> Map {
        let contents = fs::read_to_string(input).unwrap();

        use Tile::*;
        let tiles = contents
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '.' => Empty,
                        '|' => SplitterVer,
                        '-' => SplitterHor,
                        '/' => Mirror1,
                        '\\' => Mirror2,
                        _e => {
                            panic!("unknown char: {:?}", _e)
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let height = tiles.len();
        let width = tiles[0].len();
        let occupancy = vec![vec![Direction::Empty; width]; height];

        Map {
            tiles,
            occupancy,
            width,
            height,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Beam {
    dir: Direction,
    x: i64,
    y: i64,
}

impl Beam {
    fn new(dir: Direction, x: i64, y: i64) -> Beam {
        Beam { dir, x, y }
    }

    fn update_step(&mut self) {
        match self.dir {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
            _e => {
                panic! {"Wrong directionmask type for beam: {:?}", _e}
            }
        };
    }

    fn update(&self, map: &mut Map) -> Vec<Beam> {
        let tile = map.tiles[self.y as usize][self.x as usize];

        use Tile::*;
        let new_dirs = match (self.dir, tile) {
            (d, Empty) => vec![d],
            (Direction::Up | Direction::Down, SplitterVer) => vec![self.dir],
            (Direction::Up | Direction::Down, SplitterHor) => {
                vec![Direction::Right, Direction::Left]
            }
            (Direction::Left | Direction::Right, SplitterHor) => vec![self.dir],
            (Direction::Left | Direction::Right, SplitterVer) => {
                vec![Direction::Up, Direction::Down]
            }
            (Direction::Up, Mirror1) => vec![Direction::Right],
            (Direction::Up, Mirror2) => vec![Direction::Left],

            (Direction::Down, Mirror1) => vec![Direction::Left],
            (Direction::Down, Mirror2) => vec![Direction::Right],

            (Direction::Left, Mirror1) => vec![Direction::Down],
            (Direction::Left, Mirror2) => vec![Direction::Up],

            (Direction::Right, Mirror1) => vec![Direction::Up],
            (Direction::Right, Mirror2) => vec![Direction::Down],
            _e => {
                panic! {"Wrong direction mask type for beam: {:?}", _e}
            }
        };

        let new_beams: Vec<_> = new_dirs
            .iter()
            .map(|d| {
                let mut b = Beam::new(*d, self.x, self.y);
                b.update_step();
                b
            })
            .filter(|b| b.is_active(map))
            .filter(|b| (map.occupancy[b.y as usize][b.x as usize] & b.dir) == Direction::Empty)
            .collect();

        new_beams
            .iter()
            .for_each(|b| map.occupancy[b.y as usize][b.x as usize] |= b.dir);

        new_beams
    }

    fn is_active(&self, map: &Map) -> bool {
        !(self.x < 0 || self.x >= map.width as i64 || self.y < 0 || self.y >= map.height as i64)
    }
}

fn task1() {
    let mut map = Map::new(INPUT);
    map.occupancy[0][0] = Direction::Right;
    let mut beams = vec![Beam::new(Direction::Right, 0, 0)];
    while beams.len() != 0 {
        beams = beams
            .into_iter()
            .map(|b| b.update(&mut map))
            .flatten()
            .collect();
    }

    let sum = map
        .occupancy
        .iter()
        .flatten()
        .filter(|c| **c != Direction::Empty)
        .count();

    println!("task1 {:?}", sum);
}

fn task2() {
    let map = Map::new(INPUT);
    let mut beams_start = Vec::new();
    for x in 0..map.width {
        beams_start.push(Beam::new(Direction::Down, x as i64, 0));
        beams_start.push(Beam::new(Direction::Up, x as i64, map.height as i64 - 1));
    }

    for y in 0..map.height {
        beams_start.push(Beam::new(Direction::Right, 0, y as i64));
        beams_start.push(Beam::new(Direction::Left, map.width as i64 - 1, y as i64));
    }

    let sums = beams_start
        .iter()
        .map(|b| {
            let mut map = map.clone();
            map.occupancy[b.y as usize][b.x as usize] = b.dir;
            let mut beams = vec![*b];
            while beams.len() != 0 {
                beams = beams
                    .into_iter()
                    .map(|b| b.update(&mut map))
                    .flatten()
                    .collect();
            }

            let sum = map
                .occupancy
                .iter()
                .flatten()
                .filter(|c| **c != Direction::Empty)
                .count();

            sum
        })
        .collect::<Vec<_>>();

    println!("task2 {:?}", sums.iter().max().unwrap());
}

fn main() {
    task1();
    task2();
}
