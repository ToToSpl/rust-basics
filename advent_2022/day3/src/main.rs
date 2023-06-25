const INPUT: &str = "./input.txt";

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs;

fn char_to_points(val: char) -> u32 {
    if val >= 'a' && val <= 'z' {
        val as u32 - 'a' as u32 + 1
    } else {
        val as u32 - 'A' as u32 + 27
    }
}

fn task1() -> u32 {
    let contents = fs::read_to_string(INPUT).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.split("\n").filter(|l| l.len() != 0).collect();
    let mut point_sum = 0;
    for line in lines {
        let mut table: HashMap<char, u32> = HashMap::new();
        for i in 0..line.len() / 2 {
            table
                .entry(line.chars().nth(i).unwrap())
                .and_modify(|x| *x = *x + 1)
                .or_insert(1);
        }

        for i in line.len() / 2..line.len() {
            match table.entry(line.chars().nth(i).unwrap()) {
                Entry::Occupied(e) => {
                    point_sum += char_to_points(*e.key());
                    break;
                }
                Entry::Vacant(_) => (),
            }
        }
    }
    point_sum
}

fn task2() -> u32 {
    let contents = fs::read_to_string(INPUT).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.split("\n").filter(|l| l.len() != 0).collect();
    let mut point_sum = 0;
    for i in (0..lines.len()).step_by(3) {
        let mut table: HashMap<char, u32> = HashMap::new();
        let line1 = lines[i];
        let line2 = lines[i + 1];
        let line3 = lines[i + 2];
        for j in 0..line1.len() {
            table.entry(line1.chars().nth(j).unwrap()).or_insert(0b1);
        }
        for j in 0..line2.len() {
            table
                .entry(line2.chars().nth(j).unwrap())
                .and_modify(|x| *x |= 0b10);
        }
        for j in 0..line3.len() {
            table
                .entry(line3.chars().nth(j).unwrap())
                .and_modify(|x| *x |= 0b100);
        }

        for (item, amount) in &table {
            if *amount == 0b111 {
                point_sum += char_to_points(*item);
                break;
            }
        }
    }
    point_sum
}

fn main() {
    let point_sum1 = task1();
    println!("Task1: {:?}", point_sum1);

    let point_sum2 = task2();
    println!("Task2: {:?}", point_sum2);
}
