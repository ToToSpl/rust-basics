use std::{fs, usize};

const INPUT: &str = "input.txt";

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn update_next_pos(
    guard_dir: &Direction,
    guard_pos: &(usize, usize),
    grid_size: &(usize, usize),
) -> Option<(i64, i64)> {
    let next_pos = match guard_dir {
        Direction::Up => (guard_pos.0 as i64 - 1, guard_pos.1 as i64),
        Direction::Down => (guard_pos.0 as i64 + 1, guard_pos.1 as i64),
        Direction::Left => (guard_pos.0 as i64, guard_pos.1 as i64 - 1),
        Direction::Right => (guard_pos.0 as i64, guard_pos.1 as i64 + 1),
    };

    if next_pos.0 < 0
        || next_pos.0 >= grid_size.0 as i64
        || next_pos.1 < 0
        || next_pos.1 >= grid_size.1 as i64
    {
        return None;
    }

    Some(next_pos)
}

fn get_walked_grid(
    occupancy_grid: &Vec<Vec<bool>>,
    guard_start_dir: &Direction,
    guard_start_pos: &(usize, usize),
) -> Option<Vec<Vec<Option<Direction>>>> {
    let grid_size = (occupancy_grid.len(), occupancy_grid[0].len());
    let mut walked_grid = vec![vec![None; grid_size.1]; grid_size.0];

    let mut guard_pos = *guard_start_pos;
    let mut guard_dir = *guard_start_dir;

    loop {
        if walked_grid[guard_pos.0][guard_pos.1] == Some(guard_dir) {
            return None;
        }

        walked_grid[guard_pos.0][guard_pos.1] = Some(guard_dir);

        let Some(mut next_pos) = update_next_pos(&guard_dir, &guard_pos, &grid_size) else {
            break;
        };

        let mut is_next_occupied = occupancy_grid[next_pos.0 as usize][next_pos.1 as usize];

        while is_next_occupied {
            guard_dir = match guard_dir {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            };

            next_pos = if let Some(next_pos) = update_next_pos(&guard_dir, &guard_pos, &grid_size) {
                next_pos
            } else {
                break;
            };

            is_next_occupied = occupancy_grid[next_pos.0 as usize][next_pos.1 as usize];
        }

        guard_pos = (next_pos.0 as usize, next_pos.1 as usize);
    }

    return Some(walked_grid);
}

fn task1() {
    let contents = fs::read_to_string(INPUT).unwrap();

    let mut guard_pos: (usize, usize) = (0, 0);
    let guard_dir = Direction::Up;
    let mut occupancy_grid: Vec<Vec<bool>> = Vec::new();

    for (y_pos, line) in contents.lines().enumerate() {
        let occupancy_line = line.chars().map(|c| c == '#').collect();
        occupancy_grid.push(occupancy_line);
        if let Some(x_pos) = line.find('^') {
            guard_pos = (y_pos, x_pos);
        }
    }

    let walked_grid = get_walked_grid(&occupancy_grid, &guard_dir, &guard_pos).unwrap();

    let count = walked_grid
        .into_iter()
        .map(|l| l.into_iter().filter(|g| g.is_some()).count())
        .sum::<usize>();
    println!("task1:\t{count}");
}

fn task2() {
    let contents = fs::read_to_string(INPUT).unwrap();

    let mut guard_pos: (usize, usize) = (0, 0);
    let guard_dir = Direction::Up;
    let mut occupancy_grid: Vec<Vec<bool>> = Vec::new();

    for (y_pos, line) in contents.lines().enumerate() {
        let occupancy_line = line.chars().map(|c| c == '#').collect();
        occupancy_grid.push(occupancy_line);
        if let Some(x_pos) = line.find('^') {
            guard_pos = (y_pos, x_pos);
        }
    }

    let walked_grid = get_walked_grid(&occupancy_grid, &guard_dir, &guard_pos).unwrap();
    let mut count = 0;

    for (y_pos, walked_line) in walked_grid.iter().enumerate() {
        for (x_pos, walked) in walked_line.iter().enumerate() {
            if walked.is_none() || (y_pos, x_pos) == guard_pos {
                continue;
            }

            let mut new_occupancy = occupancy_grid.clone();
            new_occupancy[y_pos][x_pos] = true;

            if get_walked_grid(&new_occupancy, &guard_dir, &guard_pos).is_none() {
                count += 1;
            }
        }
    }

    println!("task2:\t{count}");
}

fn main() {
    task1();
    task2();
}
