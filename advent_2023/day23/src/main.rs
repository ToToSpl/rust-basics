use std::collections::HashSet;
use std::fs;
use std::iter::zip;

const INPUT: &str = "input.txt";

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Field {
    Empty,
    Occupied,
    SlopeToLeft,
    SlopeToRight,
    SlopeToUp,
    SlopeToDown,
}

struct Map {
    map: Vec<Vec<Field>>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(input: &str) -> Map {
        let contents = fs::read_to_string(input).unwrap();
        let map = contents
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '#' => Field::Occupied,
                        '.' => Field::Empty,
                        '>' => Field::SlopeToRight,
                        '<' => Field::SlopeToLeft,
                        'v' => Field::SlopeToDown,
                        '^' => Field::SlopeToUp,
                        _e => panic!("Unrecognized symbol {:?}", _e),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let height = map.len();
        let width = map[0].len();
        Map { map, height, width }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

fn task1() {
    let map = Map::new(INPUT);
    let mut paths = Vec::new();
    let mut stack = Vec::new();
    let start = Point { x: 1, y: 1 };
    let end = Point {
        x: map.width - 2,
        y: map.height - 1,
    };

    let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let forbidden = [
        Field::SlopeToLeft,
        Field::SlopeToRight,
        Field::SlopeToUp,
        Field::SlopeToDown,
    ];

    stack.push((start, 1, HashSet::from([Point { x: 1, y: 0 }])));
    while let Some((point, steps, mut discovered)) = stack.pop() {
        if point == end {
            paths.push(steps);
            continue;
        }
        let new_points = zip(dirs, forbidden)
            .filter_map(|(d, f)| {
                let p = Point {
                    x: (point.x as i64 + d.0) as usize,
                    y: (point.y as i64 + d.1) as usize,
                };
                let field = map.map[p.y][p.x];
                if field == f || field == Field::Occupied {
                    return None;
                }
                if discovered.get(&p).is_some() {
                    return None;
                }
                Some(p)
            })
            .collect::<Vec<_>>();

        discovered.insert(point);
        if new_points.len() == 1 {
            stack.push((new_points[0], steps + 1, discovered));
        } else {
            for p in new_points.into_iter() {
                stack.push((p, steps + 1, discovered.clone()));
            }
        }
    }

    println!("task1 {:?}", paths.iter().max().unwrap());
}

fn task2() {
    // slow but works :P
    let map = Map::new(INPUT);
    let mut paths = Vec::new();
    let mut stack = Vec::new();
    let start = Point { x: 1, y: 1 };
    let end = Point {
        x: map.width - 2,
        y: map.height - 1,
    };

    let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    stack.push((start, 1, HashSet::from([Point { x: 1, y: 0 }])));
    while let Some((point, steps, mut discovered)) = stack.pop() {
        if point == end {
            paths.push(steps);
            continue;
        }
        let new_points = dirs
            .iter()
            .filter_map(|d| {
                let p = Point {
                    x: (point.x as i64 + d.0) as usize,
                    y: (point.y as i64 + d.1) as usize,
                };
                let field = map.map[p.y][p.x];
                if field == Field::Occupied {
                    return None;
                }
                if discovered.get(&p).is_some() {
                    return None;
                }
                Some(p)
            })
            .collect::<Vec<_>>();

        discovered.insert(point);
        if new_points.len() == 1 {
            stack.push((new_points[0], steps + 1, discovered));
        } else {
            for p in new_points.into_iter() {
                stack.push((p, steps + 1, discovered.clone()));
            }
        }
    }

    println!("task2 {:?}", paths.iter().max().unwrap());
}

fn main() {
    task1();
    task2();
}
