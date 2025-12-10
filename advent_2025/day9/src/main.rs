use std::fs;

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
    println!("task2:\t");
}

fn main() {
    task1();
    // task2();
}
