use std::{collections::HashMap, fs};

const INPUT: &str = "input.txt";
// const INPUT: &str = "input.test.txt";

fn load_tiles() -> Vec<(i64, i64)> {
    fs::read_to_string(INPUT)
        .unwrap()
        .lines()
        .map(|l| {
            let mut d = l.split(",");
            (
                d.next().unwrap().parse().unwrap(),
                d.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

fn task1() {
    let tiles = load_tiles();

    let max_area = tiles
        .iter()
        .enumerate()
        .flat_map(|(i, t1)| {
            tiles.iter().enumerate().filter_map(move |(j, t2)| {
                if i >= j {
                    None
                } else {
                    Some((1 + (t1.0 - t2.0).abs()) * (1 + (t1.1 - t2.1).abs()))
                }
            })
        })
        .max()
        .unwrap();

    println!("task1:\t{max_area}");
}

fn task2() {
    let tiles = load_tiles();

    let x_map: HashMap<u32, u32> = {
        let mut x: Vec<_> = tiles.iter().map(|(x, _)| x).collect();
        x.sort_unstable();
        x.dedup();

        HashMap::from_iter(
            x.into_iter()
                .enumerate()
                .map(|(i, v)| (*v as u32, i as u32)),
        )
    };

    let y_map: HashMap<u32, u32> = {
        let mut y: Vec<_> = tiles.iter().map(|(_, y)| y).collect();
        y.sort_unstable();
        y.dedup();

        HashMap::from_iter(
            y.into_iter()
                .enumerate()
                .map(|(i, v)| (*v as u32, i as u32)),
        )
    };

    let tiles_reduced: Vec<_> = tiles
        .iter()
        .map(|&(x, y)| (x_map[&(x as u32)], y_map[&(y as u32)]))
        .collect();

    let width = (tiles_reduced.iter().map(|(x, _)| x).max().unwrap() + 1) as usize;
    let height = (tiles_reduced.iter().map(|(_, y)| y).max().unwrap() + 1) as usize;

    let mut grid = vec![vec![false; width]; height];

    {
        let mut p = *tiles_reduced.last().unwrap();
        for t in &tiles_reduced {
            for y in p.1.min(t.1)..=p.1.max(t.1) {
                for x in p.0.min(t.0)..=p.0.max(t.0) {
                    grid[y as usize][x as usize] = true;
                }
            }
            p = *t;
        }
    }

    {
        let checks = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

        // after anylizing input, this is a save spot to start
        // also input does not have any tight turns which would block flood fill
        let mut visited = vec![((height / 2) as i64, (width / 2) as i64)];
        while let Some(t) = visited.pop() {
            grid[t.1 as usize][t.0 as usize] = true;

            for c in &checks {
                let p0 = t.0 + c.0;
                let p1 = t.1 + c.1;

                if p0 >= 0 && (p0 as usize) < width && p1 >= 0 && (p1 as usize) < height {
                    if !grid[p1 as usize][p0 as usize] {
                        visited.push((p0, p1));
                    }
                }
            }
        }
    }

    let max_area = tiles
        .iter()
        .enumerate()
        .flat_map(|(i, t1)| {
            let (grid, tiles_reduced) = (&grid, &tiles_reduced);
            tiles.iter().enumerate().filter_map(move |(j, t2)| {
                if i >= j {
                    return None;
                }

                let ir = tiles_reduced[i];
                let jr = tiles_reduced[j];

                for y in jr.1.min(ir.1)..=jr.1.max(ir.1) {
                    for x in jr.0.min(ir.0)..=jr.0.max(ir.0) {
                        if !grid[y as usize][x as usize] {
                            return None;
                        }
                    }
                }

                Some((1 + (t1.0 - t2.0).abs()) * (1 + (t1.1 - t2.1).abs()))
            })
        })
        .max()
        .unwrap();

    println!("task2:\t{max_area}");
}

fn main() {
    task1();
    task2();
}
