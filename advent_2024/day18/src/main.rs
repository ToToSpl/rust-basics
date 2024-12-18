use std::{cmp::Ordering, collections::BinaryHeap, fs};

const INPUT: &str = "input.txt";
// const MAP_SIZE: usize = 7;
// const BYTES_FALLEN: usize = 12;
const MAP_SIZE: usize = 71;
const BYTES_FALLEN: usize = 1024;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Step {
    pos: (usize, usize),
    distance: usize,
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_path(bytes_pos: &[(usize, usize)], bytes_taken: usize) -> Option<Step> {
    let mut obstacle_map = vec![vec![false; MAP_SIZE]; MAP_SIZE];
    let mut visited_map = vec![vec![false; MAP_SIZE]; MAP_SIZE];

    bytes_pos.iter().take(bytes_taken + 1).for_each(|&(y, x)| {
        obstacle_map[y][x] = true;
    });

    let mut heap = BinaryHeap::new();

    let start_step = Step {
        pos: (0, 0),
        distance: 0,
    };
    heap.push(start_step);

    let end = (MAP_SIZE - 1, MAP_SIZE - 1);
    let mut end_pos = None;

    while let Some(step) = heap.pop() {
        if step.pos.0 == end.0 && step.pos.1 == end.1 {
            end_pos = Some(step);
            break;
        }

        if obstacle_map[step.pos.0][step.pos.1] || visited_map[step.pos.0][step.pos.1] {
            continue;
        }
        visited_map[step.pos.0][step.pos.1] = true;

        if step.pos.0 != 0 {
            heap.push(Step {
                pos: (step.pos.0 - 1, step.pos.1),
                distance: step.distance + 1,
            });
        }

        if step.pos.0 != MAP_SIZE - 1 {
            heap.push(Step {
                pos: (step.pos.0 + 1, step.pos.1),
                distance: step.distance + 1,
            });
        }

        if step.pos.1 != 0 {
            heap.push(Step {
                pos: (step.pos.0, step.pos.1 - 1),
                distance: step.distance + 1,
            });
        }

        if step.pos.1 != MAP_SIZE - 1 {
            heap.push(Step {
                pos: (step.pos.0, step.pos.1 + 1),
                distance: step.distance + 1,
            });
        }
    }

    end_pos
}

fn task1() {
    let contents = fs::read_to_string(INPUT).unwrap();

    let bytes_pos: Vec<(usize, usize)> = contents
        .lines()
        .map(|l| {
            let pos: Vec<usize> = l.split(',').map(|p| p.parse().unwrap()).collect();
            (pos[1], pos[0])
        })
        .collect();

    let end_pos = find_path(&bytes_pos, BYTES_FALLEN);

    let Some(end_pos) = end_pos else {
        panic!("no path has been found");
    };

    println!("task1:\t{:}", end_pos.distance);
}

fn task2() {
    let contents = fs::read_to_string(INPUT).unwrap();

    let bytes_pos: Vec<(usize, usize)> = contents
        .lines()
        .map(|l| {
            let pos: Vec<usize> = l.split(',').map(|p| p.parse().unwrap()).collect();
            (pos[1], pos[0])
        })
        .collect();

    let blocking_byte_index = (0..bytes_pos.len())
        .collect::<Vec<usize>>()
        .binary_search_by(|&i| {
            if find_path(&bytes_pos, i).is_none() {
                if find_path(&bytes_pos, i - 1).is_some() {
                    Ordering::Equal
                } else {
                    Ordering::Greater
                }
            } else {
                Ordering::Less
            }
        })
        .unwrap();

    let blocking_byte = bytes_pos[blocking_byte_index];

    println!("task2:\t{:},{:}", blocking_byte.1, blocking_byte.0);
}

fn main() {
    task1();
    task2();
}
