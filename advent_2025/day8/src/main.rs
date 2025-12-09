use std::{collections::HashMap, fs};

const INPUT: &str = "input.txt";
// const INPUT: &str = "input.test.txt";

#[derive(Clone, Copy, Debug)]
struct Pos {
    x: i64,
    y: i64,
    z: i64,
}

fn box_poses() -> Vec<Pos> {
    fs::read_to_string(INPUT)
        .unwrap()
        .lines()
        .map(|l| {
            let cords: Vec<i64> = l.split(",").map(|c| c.parse().unwrap()).collect();

            Pos {
                x: cords[0],
                y: cords[1],
                z: cords[2],
            }
        })
        .collect()
}

fn closests(box_poses: &Vec<Pos>) -> Vec<(usize, usize, i64)> {
    let mut c: Vec<_> = box_poses
        .iter()
        .enumerate()
        .flat_map(|(i, p)| {
            box_poses
                .iter()
                .enumerate()
                .filter_map(move |(j, pp)| {
                    if j >= i {
                        None
                    } else {
                        Some((
                            i,
                            j,
                            (p.x - pp.x).pow(2) + (p.y - pp.y).pow(2) + (p.z - pp.z).pow(2),
                        ))
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect();
    c.sort_unstable_by_key(|(_, _, d)| *d);
    c
}

fn task1() {
    let box_poses = box_poses();
    let closests = closests(&box_poses);

    let circuit_map = {
        let mut c: Vec<Option<usize>> = vec![None; box_poses.len()];
        let mut circuit_index = 0;

        for kk in 0..1000 {
            // for kk in 0..10 {
            let (i, j, _) = closests[kk];
            match (c[i], c[j]) {
                (Some(ci), None) => c[j] = Some(ci),
                (None, Some(cj)) => c[i] = Some(cj),
                (Some(ci), Some(cj)) => {
                    if ci != cj {
                        for k in 0..c.len() {
                            if c[k] == Some(cj) {
                                c[k] = Some(ci);
                            }
                        }
                    }
                }
                (None, None) => {
                    c[i] = Some(circuit_index);
                    c[j] = Some(circuit_index);
                    circuit_index += 1;
                }
            }
        }

        let mut map = HashMap::new();

        for cp in c {
            if let Some(cpp) = cp {
                *map.entry(cpp).or_insert(0) += 1;
            }
        }

        map
    };

    let mut biggest: Vec<_> = circuit_map.values().collect();
    biggest.sort_unstable();

    let mut product = 1;
    for i in 0..3 {
        product *= biggest[biggest.len() - 1 - i];
    }

    println!("task1:\t{product}");
}

fn task2() {
    let box_poses = box_poses();
    let closests = closests(&box_poses);

    let mut c: Vec<Option<usize>> = vec![None; box_poses.len()];
    let mut circuit_index = 0;

    let mut closests = closests.into_iter();

    let product = loop {
        let (i, j, _) = closests.next().unwrap();
        match (c[i], c[j]) {
            (Some(ci), None) => c[j] = Some(ci),
            (None, Some(cj)) => c[i] = Some(cj),
            (Some(ci), Some(cj)) => {
                if ci != cj {
                    for k in 0..c.len() {
                        if c[k] == Some(cj) {
                            c[k] = Some(ci);
                        }
                    }
                }
            }
            (None, None) => {
                c[i] = Some(circuit_index);
                c[j] = Some(circuit_index);
                circuit_index += 1;
            }
        }

        if let Some(all_circuit) = c[0] {
            if c.iter().all(|e| *e == Some(all_circuit)) {
                break box_poses[i].x * box_poses[j].x;
            }
        }
    };

    println!("task2:\t{:?}", product);
}

fn main() {
    task1();
    task2();
}
