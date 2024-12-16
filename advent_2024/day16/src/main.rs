use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    fs,
};

const INPUT: &str = "input.txt";

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Tile {
    Start,
    End,
    Wall,
    Empty,
}

impl Tile {
    fn from_char(c: char) -> Tile {
        use Tile::*;
        match c {
            '#' => Wall,
            '.' => Empty,
            'S' => Start,
            'E' => End,
            _ => panic!("cannot convert {c} to tile"),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn rotate_cw(&self) -> Dir {
        use Dir::*;
        match *self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    fn rotate_ccw(&self) -> Dir {
        use Dir::*;
        match *self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up,
        }
    }

    // could potentialy overflow, but map is surrounded by wall so should not happen
    fn move_with_dir(&self, pos: &(usize, usize)) -> (usize, usize) {
        use Dir::*;
        match *self {
            Up => (pos.0 - 1, pos.1),
            Down => (pos.0 + 1, pos.1),
            Left => (pos.0, pos.1 - 1),
            Right => (pos.0, pos.1 + 1),
        }
    }

    fn to_flag(&self) -> u8 {
        use Dir::*;
        match *self {
            Up => 0x1,
            Left => 0x2,
            Down => 0x4,
            Right => 0x8,
        }
    }

    fn to_index(&self) -> usize {
        use Dir::*;
        match *self {
            Up => 0,
            Left => 1,
            Down => 2,
            Right => 3,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct State {
    path: Vec<(usize, usize)>,
    score: usize,
    dir: Dir,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn task1() {
    let contents = fs::read_to_string(INPUT).unwrap();

    let map: Vec<Vec<Tile>> = contents
        .lines()
        .map(|l| l.chars().map(Tile::from_char).collect())
        .collect();

    let start_state = map
        .iter()
        .enumerate()
        .find_map(|(y, l)| {
            l.iter().enumerate().find_map(|(x, t)| {
                if *t == Tile::Start {
                    Some(State {
                        path: vec![(y, x)],
                        score: 0,
                        dir: Dir::Right,
                    })
                } else {
                    None
                }
            })
        })
        .unwrap();
    let mut end_state: Option<State> = None;

    let mut visited = vec![vec![0u8; map[0].len()]; map.len()];

    let mut heap = BinaryHeap::new();
    heap.push(start_state);

    while let Some(state) = heap.pop() {
        let pos = state.path.last().unwrap();
        if visited[pos.0][pos.1] & state.dir.to_flag() != 0 {
            continue;
        }
        visited[pos.0][pos.1] |= state.dir.to_flag();

        let tile = map[pos.0][pos.1];
        if tile == Tile::Wall {
            continue;
        }

        if tile == Tile::End {
            end_state = Some(state);
            break;
        }

        let mut new_path = state.path.clone();
        new_path.push(state.dir.move_with_dir(pos));

        heap.push(State {
            path: new_path,
            dir: state.dir,
            score: state.score + 1,
        });

        heap.push(State {
            path: state.path.clone(),
            dir: state.dir.rotate_cw(),
            score: state.score + 1000,
        });

        heap.push(State {
            path: state.path,
            dir: state.dir.rotate_ccw(),
            score: state.score + 1000,
        });
    }

    let Some(end_state) = end_state else {
        panic!("no path has been found");
    };

    println!("task1:\t{:}", end_state.score);
}

fn task2() {
    let contents = fs::read_to_string(INPUT).unwrap();

    let map: Vec<Vec<Tile>> = contents
        .lines()
        .map(|l| l.chars().map(Tile::from_char).collect())
        .collect();

    let start_state = map
        .iter()
        .enumerate()
        .find_map(|(y, l)| {
            l.iter().enumerate().find_map(|(x, t)| {
                if *t == Tile::Start {
                    Some(State {
                        path: vec![(y, x)],
                        score: 0,
                        dir: Dir::Right,
                    })
                } else {
                    None
                }
            })
        })
        .unwrap();
    let mut end_states: Vec<State> = Vec::new();

    let mut visited: Vec<Vec<[Option<usize>; 4]>> = vec![vec![[None; 4]; map[0].len()]; map.len()];

    let mut heap = BinaryHeap::new();
    heap.push(start_state);

    while let Some(state) = heap.pop() {
        let pos = state.path.last().unwrap();
        let tile = map[pos.0][pos.1];
        if tile == Tile::Wall {
            continue;
        }

        if let Some(other_score) = visited[pos.0][pos.1][state.dir.to_index()] {
            if other_score < state.score {
                continue;
            }
        }
        visited[pos.0][pos.1][state.dir.to_index()] = Some(state.score);

        if tile == Tile::End {
            if end_states.len() == 0 || state.score == end_states[0].score {
                end_states.push(state.clone());
            } else {
                break;
            }
        }

        let mut new_path = state.path.clone();
        new_path.push(state.dir.move_with_dir(pos));

        heap.push(State {
            path: new_path,
            dir: state.dir,
            score: state.score + 1,
        });

        heap.push(State {
            path: state.path.clone(),
            dir: state.dir.rotate_cw(),
            score: state.score + 1000,
        });

        heap.push(State {
            path: state.path,
            dir: state.dir.rotate_ccw(),
            score: state.score + 1000,
        });
    }

    if end_states.len() == 0 {
        panic!("no path has been found");
    };

    let unique_tiles: HashSet<(usize, usize)> =
        end_states.into_iter().map(|s| s.path).flatten().collect();

    println!("task2:\t{:}", unique_tiles.len());
}

fn main() {
    task1();
    task2();
}
