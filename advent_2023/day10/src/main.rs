use std::collections::{HashMap, HashSet};
use std::fs;
use std::iter::zip;

const INPUT: &str = "input.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord(usize, usize);

#[derive(Debug)]
struct Map {
    map: Vec<Vec<char>>,
    start: Coord,
    width: usize,
    height: usize,
}

impl Map {
    fn new(input: &str) -> Map {
        let contents = fs::read_to_string(input).unwrap();
        let map: Vec<Vec<_>> = contents.lines().map(|l| l.chars().collect()).collect();

        let height = map.len();
        let width = map[0].len();

        let mut temp = Map {
            map,
            start: Coord(0, 0),
            height,
            width,
        };
        temp.start = temp.find_start();
        temp
    }

    fn find_start(&self) -> Coord {
        for (y, v) in self.map.iter().enumerate() {
            for (x, c) in v.iter().enumerate() {
                if *c == 'S' {
                    return Coord(y, x);
                }
            }
        }
        panic!("Start was not found!");
    }

    fn step_from_start(&self) -> Coord {
        let options = [
            ((1, 0), ['|', 'L', 'J']),
            ((-1, 0), ['|', 'F', '7']),
            ((0, 1), ['-', 'J', '7']),
            ((0, -1), ['-', 'L', 'F']),
        ];
        for (dir, option) in options {
            let y = self.start.0 as i32 + dir.0;
            let x = self.start.1 as i32 + dir.1;
            if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
                continue;
            }
            let neigh = self.map[y as usize][x as usize];
            if option.contains(&neigh) {
                return Coord(y as usize, x as usize);
            }
        }
        panic!("Not matching direction coming from start!");
    }

    fn next_step(&self, prev: &Coord, curr: &Coord) -> Coord {
        let paths = HashMap::from([
            ('|', [(-1, 0), (1, 0)]),
            ('-', [(0, -1), (0, 1)]),
            ('L', [(-1, 0), (0, 1)]),
            ('J', [(-1, 0), (0, -1)]),
            ('7', [(1, 0), (0, -1)]),
            ('F', [(1, 0), (0, 1)]),
        ]);

        let curr_c = self.map[curr.0][curr.1];
        let directions: Vec<_> = paths[&curr_c]
            .iter()
            .map(|d| {
                Coord(
                    (curr.0 as i32 + d.0) as usize,
                    (curr.1 as i32 + d.1) as usize,
                )
            })
            .collect();

        if directions[0] == *prev {
            directions[1]
        } else {
            directions[0]
        }
    }

    fn patch_start(&mut self) {
        let path = self.plot_path();
        let start = path[1];
        let start = (
            start.0 as i32 - self.start.0 as i32,
            start.1 as i32 - self.start.1 as i32,
        );
        let end = path.last().unwrap();
        let end = (
            end.0 as i32 - self.start.0 as i32,
            end.1 as i32 - self.start.1 as i32,
        );

        let pipe = match (start, end) {
            ((1, 0), (-1, 0)) | ((-1, 0), (1, 0)) => '|',
            ((0, 1), (0, -1)) | ((0, -1), (0, 1)) => '-',
            ((-1, 0), (0, 1)) | ((0, 1), (-1, 0)) => 'L',
            ((-1, 0), (0, -1)) | ((0, -1), (-1, 0)) => 'J',
            ((1, 0), (0, -1)) | ((0, -1), (1, 0)) => '7',
            ((1, 0), (0, 1)) | ((0, 1), (1, 0)) => 'F',
            _ => panic!("Unknown start patch: {:?}\t{:?}", start, end),
        };

        self.map[self.start.0][self.start.1] = pipe;
    }

    fn plot_path(&self) -> Vec<Coord> {
        let mut prev = self.start;
        let mut curr = self.step_from_start();
        let mut path = vec![prev];
        while curr != self.start {
            path.push(curr);
            let next = self.next_step(&prev, &curr);
            prev = curr;
            curr = next;
        }
        path
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TileType {
    Unknown,
    Path,
    Inside,
    Outside,
}

struct OccupyMap<'a> {
    map: &'a Map,
    occupy: Vec<Vec<TileType>>,
    width: usize,
    height: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SideType {
    Left,
    Right,
}

struct Side {
    side: SideType,
    coord: Coord,
}

impl<'a> OccupyMap<'a> {
    fn new(map: &mut Map) -> OccupyMap {
        map.patch_start();
        let mut occupy = vec![vec![TileType::Unknown; map.width]; map.height];
        let path = map.plot_path();

        for point in &path {
            occupy[point.0][point.1] = TileType::Path;
        }

        let mut temp = OccupyMap {
            width: map.width,
            height: map.height,
            map,
            occupy,
        };

        let outside_side = temp.get_outside_side();

        let mut prev = path.last().unwrap().clone();
        for i in 0..path.len() - 1 {
            let curr = path[i];
            let sides = temp.get_sides(&prev, &curr);
            prev = curr;

            for side in sides {
                if side.side == outside_side {
                    temp.flood_with(TileType::Outside, side.coord);
                } else {
                    temp.flood_with(TileType::Inside, side.coord);
                }
            }
        }

        temp
    }

    fn get_outside_side(&mut self) -> SideType {
        let first = self.find_first_outside();
        self.flood_with(TileType::Outside, first);

        let path = self.map.plot_path();

        let mut prev = path.last().unwrap().clone();
        for i in 0..path.len() - 1 {
            let curr = path[i];
            let sides = self.get_sides(&prev, &curr);
            for side in sides {
                if self.occupy[side.coord.0][side.coord.1] == TileType::Outside {
                    return side.side;
                }
            }
            prev = curr;
        }
        panic!("Path does not touch outside!");
    }

    fn find_first_outside(&self) -> Coord {
        for y in 1..self.height - 1 {
            if self.occupy[y][0] == TileType::Unknown {
                return Coord(y, 0);
            }

            if *self.occupy[y].last().unwrap() == TileType::Unknown {
                return Coord(y, self.width - 1);
            }
        }

        for x in 1..self.width - 1 {
            if self.occupy[0][x] == TileType::Unknown {
                return Coord(0, x);
            }

            if self.occupy.last().unwrap()[x] == TileType::Unknown {
                return Coord(self.height - 1, x);
            }
        }
        panic!("First outside was not found!");
    }

    fn flood_with(&mut self, tile: TileType, start: Coord) {
        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        let mut checked: HashSet<Coord> = HashSet::new();
        let mut stack = vec![start];
        while let Some(curr) = stack.pop() {
            if checked.contains(&curr) {
                continue;
            }
            checked.insert(curr);
            if self.occupy[curr.0][curr.1] != TileType::Unknown {
                continue;
            }
            self.occupy[curr.0][curr.1] = tile;

            for dir in directions {
                let y = curr.0 as i32 + dir.0;
                let x = curr.1 as i32 + dir.1;
                if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
                    continue;
                }
                stack.push(Coord(y as usize, x as usize));
            }
        }
    }

    fn get_sides(&self, prev: &Coord, curr: &Coord) -> Vec<Side> {
        use SideType::{Left, Right};
        let pipe = self.map.map[curr.0][curr.1];
        let vec = (curr.0 as i32 - prev.0 as i32, curr.1 as i32 - prev.1 as i32);

        #[rustfmt::skip]
        let sides = match pipe {
            '-' => { if vec.1 > 0 { [Left, Right] } else { [Right, Left] } } // right
            '|' => { if vec.0 < 0 { [Left, Right] } else { [Right, Left] } } // up
            'L' => { if vec.1 == 0 { [Right, Right] } else { [Left, Left] } } // right
            'J' => { if vec.1 == 0 { [Left, Left] } else { [Right, Right] } } // left
            '7' => { if vec.1 == 0 { [Right, Right] } else { [Left, Left] } } // left
            'F' => { if vec.1 == 0 { [Left, Left] } else { [Right, Right] } } // right
            _ => panic!("Unknown pipe type!"),
        };

        let dirs = match pipe {
            '-' => [(-1, 0), (1, 0)],
            '|' => [(0, -1), (0, 1)],
            'L' => [(0, -1), (1, 0)],
            'J' => [(0, 1), (1, 0)],
            '7' => [(0, 1), (-1, 0)],
            'F' => [(0, -1), (-1, 0)],
            _ => panic!("Unknown pipe type!"),
        };

        zip(sides, dirs)
            .filter_map(|(side, dir)| {
                let y = curr.0 as i32 + dir.0;
                let x = curr.1 as i32 + dir.1;
                if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
                    None
                } else {
                    Some(Side {
                        side,
                        coord: Coord(y as usize, x as usize),
                    })
                }
            })
            .collect()
    }

    fn count_inside(&self) -> usize {
        self.occupy
            .iter()
            .map(|v| {
                v.iter()
                    .map(|t| if *t == TileType::Inside { 1 } else { 0 })
                    .sum::<usize>()
            })
            .sum()
    }
}

fn task2() {
    let mut map = Map::new(INPUT);
    let occupy_map = OccupyMap::new(&mut map);
    println!("task2 {:?}", occupy_map.count_inside());
}

fn task1() {
    let map = Map::new(INPUT);
    let path = map.plot_path();
    let middle = path.len() / 2 + path.len() % 2;
    println!("task1 {:?}", middle);
}

fn main() {
    task1();
    task2();
}
