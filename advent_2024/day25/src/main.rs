use std::fs;

const INPUT: &str = "input.txt";
const KEY_LEN: usize = 5;
const KEY_WIDTH: u8 = 5;

fn task1() {
    let contents = fs::read_to_string(INPUT).unwrap();
    let mut lines = contents.lines();

    let mut keys: Vec<[u8; KEY_LEN]> = Vec::new();
    let mut locks: Vec<[u8; KEY_LEN]> = Vec::new();

    loop {
        let mut buffer = Vec::new();
        for _ in 0..7 {
            buffer.push(lines.next().unwrap());
        }

        let mut values = [0u8; KEY_LEN];

        if buffer[0] == "#####" {
            for i in 1..7 {
                for j in 0..5 {
                    if buffer[i].chars().nth(j).unwrap() == '#' {
                        values[j] += 1
                    }
                }
            }
            locks.push(values);
        } else {
            for i in 0..6 {
                for j in 0..5 {
                    if buffer[i].chars().nth(j).unwrap() == '#' {
                        values[j] += 1
                    }
                }
            }
            keys.push(values);
        }

        if lines.by_ref().peekable().peek().is_none() {
            break;
        }
    }

    let mut score: usize = 0;
    for key in keys {
        for lock in &locks {
            let alligned = key
                .iter()
                .zip(lock)
                .map(|(k, l)| k + l)
                .filter(|&s| s <= KEY_WIDTH)
                .count();

            if alligned == KEY_LEN {
                score += 1;
            }
        }
    }

    println!("task1:\t{score}");
}

fn main() {
    task1();
}
