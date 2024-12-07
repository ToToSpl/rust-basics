use rayon::prelude::*;
use std::{fs, time};

const INPUT: &str = "input.txt";

#[derive(Debug)]
struct Calibration {
    value: u64,
    inputs: Vec<u64>,
}

impl Calibration {
    fn new(line: &str) -> Calibration {
        let splitted: Vec<&str> = line.split(": ").collect();

        Calibration {
            value: splitted[0].parse().unwrap(),
            inputs: splitted[1].split(' ').map(|p| p.parse().unwrap()).collect(),
        }
    }

    fn can_calibrate_brute(&self) -> Option<u64> {
        // 0 means addition, 1 means multiplication
        let mask_len = self.inputs.len() - 1;
        let comb_max = 2u64.pow(mask_len as u32);

        'outer: for comb_mask in 0..comb_max {
            let mut sum: u64 = self.inputs[0];
            for i in 0..mask_len {
                let bit = (comb_mask >> i) & 0b1;

                if bit == 0 {
                    sum += self.inputs[i + 1];
                } else {
                    sum *= self.inputs[i + 1];
                }

                if sum > self.value {
                    continue 'outer;
                }
            }

            if sum == self.value {
                return Some(self.value);
            }
        }

        None
    }

    fn can_calibrate_rec(&self) -> Option<u64> {
        if _rec_two(self.value, self.inputs.len() - 1, &self.inputs) {
            Some(self.value)
        } else {
            None
        }
    }

    fn can_calibrate_three_rec(&self) -> Option<u64> {
        if _rec_three(self.value, self.inputs.len() - 1, &self.inputs) {
            Some(self.value)
        } else {
            None
        }
    }

    fn can_calibrate_three_brute(&self) -> Option<u64> {
        // 0 means addition, 1 means multiplication, seperate mask for concat
        let mask_len = self.inputs.len() - 1;
        let comb_max = 2u64.pow(mask_len as u32);

        for concat_mask in 0..comb_max {
            'outer1: for comb_mask in 0..comb_max {
                let mut sum: u64 = self.inputs[0];
                for i in 0..mask_len {
                    let bit = (comb_mask >> i) & 0b1;
                    let bit_c = (concat_mask >> i) & 0b1;

                    if bit_c == 1 {
                        let input = self.inputs[i + 1];
                        let input_digits = input.checked_ilog10().unwrap_or(0) + 1;
                        sum = sum * 10u64.pow(input_digits) + input;
                    } else if bit == 0 {
                        sum += self.inputs[i + 1];
                    } else {
                        sum *= self.inputs[i + 1];
                    }

                    if sum > self.value {
                        continue 'outer1;
                    }
                }

                if sum == self.value {
                    return Some(self.value);
                }
            }
        }
        None
    }
}

fn _rec_two(value: u64, index: usize, inputs: &[u64]) -> bool {
    let v = inputs[index];
    if index == 0 {
        value == v
    } else if value > v && _rec_two(value - v, index - 1, inputs) {
        true
    } else if value % v == 0 && _rec_two(value / v, index - 1, inputs) {
        true
    } else {
        false
    }
}

fn _rec_three(value: u64, index: usize, inputs: &[u64]) -> bool {
    let v = inputs[index];
    if index == 0 {
        value == v
    } else if value > v && _rec_three(value - v, index - 1, inputs) {
        true
    } else if value % v == 0 && _rec_three(value / v, index - 1, inputs) {
        true
    } else {
        let v_digits = v.checked_ilog10().unwrap_or(0) + 1;
        let ten_mul = 10u64.pow(v_digits);

        if value % ten_mul == v && _rec_three(value / ten_mul, index - 1, inputs) {
            true
        } else {
            false
        }
    }
}

fn task2_fast() {
    let contents = fs::read_to_string(INPUT).unwrap();

    let count = contents
        .lines()
        .map(Calibration::new)
        .collect::<Vec<Calibration>>()
        .par_iter()
        .filter_map(|c| c.can_calibrate_three_rec())
        .sum::<u64>();

    println!("task2:\t{count}");
}

fn task1_fast() {
    let contents = fs::read_to_string(INPUT).unwrap();

    let count = contents
        .lines()
        .map(Calibration::new)
        .filter_map(|c| c.can_calibrate_rec())
        .sum::<u64>();

    println!("task1:\t{count}");
}

fn task2() {
    let contents = fs::read_to_string(INPUT).unwrap();

    let count = contents
        .lines()
        .map(Calibration::new)
        .collect::<Vec<Calibration>>()
        .par_iter()
        .filter_map(|c| c.can_calibrate_three_brute())
        .sum::<u64>();

    println!("task2:\t{count}");
}

fn task1() {
    let contents = fs::read_to_string(INPUT).unwrap();

    let count = contents
        .lines()
        .map(Calibration::new)
        .filter_map(|c| c.can_calibrate_brute())
        .sum::<u64>();

    println!("task1:\t{count}");
}

fn main() {
    println!("slow");
    let start = time::SystemTime::now();
    task1();
    task2();
    println!("{:?}", start.elapsed().unwrap());

    println!("fast");
    let start = time::SystemTime::now();
    task1_fast();
    task2_fast();
    println!("{:?}", start.elapsed().unwrap());
}
