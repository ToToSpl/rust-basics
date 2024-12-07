use rayon::prelude::*;
use std::fs;

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

    // so fast that didnt even bother to optimize
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
    task1();
    task2();
}
