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

fn main() {
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
    println!("{:?}", point_sum);
}
