use std::collections::HashSet;
use std::fs;
use std::io::Write;
use tqdm::tqdm;

const INPUT: &str = "input.txt";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Field {
    Empty,
    Occupied,
}

#[derive(Clone, Debug)]
struct Map {
    map: Vec<Vec<Field>>,
    start_x: i64,
    start_y: i64,
    width: usize,
    height: usize,
}

impl Map {
    fn new(input: &str) -> Map {
        let contents = fs::read_to_string(input).unwrap();
        let mut start_x = 0;
        let mut start_y = 0;
        let map = contents
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '.' => Field::Empty,
                        '#' => Field::Occupied,
                        'S' => {
                            start_x = x as i64;
                            start_y = y as i64;
                            Field::Empty
                        }
                        _e => panic!("Uknown input: {:?}", _e),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let height = map.len();
        let width = map[0].len();

        Map {
            map,
            start_x,
            start_y,
            width,
            height,
        }
    }

    fn get(&self, x: i64, y: i64) -> Field {
        let mut x = x % self.width as i64;
        if x < 0 {
            x += self.width as i64;
        }
        let mut y = y % self.height as i64;
        if y < 0 {
            y += self.height as i64;
        }
        self.map[y as usize][x as usize]
    }
}

#[derive(Hash, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

fn task1() {
    let map = Map::new(INPUT);
    let mut finished: HashSet<Point> = HashSet::from([Point {
        x: map.start_x,
        y: map.start_y,
    }]);
    let max_steps = 64;

    for _ in 0..max_steps {
        let mut new_finished = HashSet::new();
        for point in finished {
            for c in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let x = point.x + c.0;
                let y = point.y + c.1;
                if x < 0 || x >= map.width as i64 || y < 0 || y >= map.height as i64 {
                    continue;
                }
                if map.map[y as usize][x as usize] == Field::Occupied {
                    continue;
                }
                new_finished.insert(Point { x, y });
            }
        }

        finished = new_finished;
    }

    println!("task1 {:?}", finished.iter().count());
}

fn task2() {
    // fuck this "nice" inputs to NP problems
    let map = Map::new(INPUT);
    let mut finished: HashSet<Point> = HashSet::from([Point {
        x: map.start_x,
        y: map.start_y,
    }]);
    let max_steps = 26_501_365; // haha
    let mut file = fs::File::create("data_out.txt").unwrap();

    for s in tqdm(0..max_steps) {
        let mut new_finished = HashSet::new();
        for point in &finished {
            for c in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let x = point.x + c.0;
                let y = point.y + c.1;

                if map.get(x, y) == Field::Occupied {
                    continue;
                }
                new_finished.insert(Point { x, y });
            }
        }

        finished = new_finished;
        file.write_all(format!("{:} {:}\n", s, finished.iter().count()).as_bytes())
            .unwrap();
    }

    println!("task2 {:?}", finished.iter().count());
}

fn main() {
    task1();
    task2();
}
