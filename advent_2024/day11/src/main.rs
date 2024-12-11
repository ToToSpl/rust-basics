use std::{collections::HashMap, fs};

const INPUT: &str = "input.txt";

#[allow(dead_code)]
fn solution_brute(start_vec: &Vec<u64>, repeats: usize) -> usize {
    let mut stones = start_vec.clone();

    for _ in 0..repeats {
        let mut i = 0;
        let end = stones.len();
        while i < end {
            let elem = stones[i];

            if elem == 0 {
                stones[i] = 1;
            } else {
                let digits_num = elem.checked_ilog10().unwrap_or(0) + 1;
                if digits_num % 2 == 0 {
                    let expon = 10u64.pow(digits_num / 2);
                    let left_digits = elem / expon;
                    let right_digits = elem % (left_digits * expon);

                    stones[i] = left_digits;
                    stones.push(right_digits);
                } else {
                    stones[i] *= 2024;
                }
            }

            i += 1;
        }
    }

    stones.len()
}

fn solution_dict(start_vec: &Vec<u64>, repeats: usize) -> usize {
    let mut dict: HashMap<u64, usize> = start_vec.iter().map(|k| (*k, 1)).collect();

    for _ in 0..repeats {
        let mut new_dict = HashMap::new();
        for (key, amount) in dict {
            if key == 0 {
                *new_dict.entry(1).or_insert(0) += amount;
            } else {
                let digits_num = key.checked_ilog10().unwrap_or(0) + 1;
                if digits_num % 2 == 0 {
                    let expon = 10u64.pow(digits_num / 2);
                    let left_digits = key / expon;
                    let right_digits = key % (left_digits * expon);

                    *new_dict.entry(left_digits).or_insert(0) += amount;
                    *new_dict.entry(right_digits).or_insert(0) += amount;
                } else {
                    *new_dict.entry(key * 2024).or_insert(0) += amount;
                }
            }
        }
        dict = new_dict;
    }

    dict.values().sum()
}

fn task1() {
    let contents = fs::read_to_string(INPUT).unwrap();

    let stones: Vec<u64> = contents[0..contents.len() - 1]
        .split(' ')
        .map(|n| n.parse().unwrap())
        .collect();

    println!("task1:\t{:}", solution_dict(&stones, 25));
}

fn task2() {
    let contents = fs::read_to_string(INPUT).unwrap();

    let stones: Vec<u64> = contents[0..contents.len() - 1]
        .split(' ')
        .map(|n| n.parse().unwrap())
        .collect();

    println!("task2:\t{:}", solution_dict(&stones, 75));
}

fn main() {
    task1();
    task2();
}
