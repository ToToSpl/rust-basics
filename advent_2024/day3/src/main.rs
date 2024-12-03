use regex::Regex;
use std::fs;

const INPUT: &str = "input.txt";

#[derive(PartialEq, Eq, Debug)]
enum Instruction {
    Mul((u64, u64)),
    Do(),
    Dont(),
}

fn task2() {
    let re =
        Regex::new(r"mul\((?:[0-9]|[1-9][0-9]|[1-9][0-9]{2}),(?:[0-9]|[1-9][0-9]|[1-9][0-9]{2})\)|do\(\)|don't\(\)")
            .unwrap();

    let contents = fs::read_to_string(INPUT).unwrap().to_string();

    let instructions = re
        .captures_iter(&contents)
        .map(|c| {
            let raw = c.extract::<0>().0;
            if raw.as_bytes()[0] == 'm' as u8 {
                let nums: Vec<u64> = raw[4..raw.len() - 1]
                    .split(',')
                    .map(|n| n.parse().unwrap())
                    .collect();

                return Instruction::Mul((nums[0], nums[1]));
            }

            if raw == "do()" {
                return Instruction::Do();
            }

            Instruction::Dont()
        })
        .collect::<Vec<Instruction>>();

    let mut is_active = true;
    let mut sum: u64 = 0;

    for instruction in &instructions {
        match instruction {
            Instruction::Mul((a, b)) => {
                if is_active {
                    sum += a * b;
                }
            }
            Instruction::Do() => {
                is_active = true;
            }
            Instruction::Dont() => {
                is_active = false;
            }
        }
    }

    println!("task2:\t{sum:}");
}

fn task1() {
    let re =
        Regex::new(r"mul\((?:[0-9]|[1-9][0-9]|[1-9][0-9]{2}),(?:[0-9]|[1-9][0-9]|[1-9][0-9]{2})\)")
            .unwrap();

    let contents = fs::read_to_string(INPUT).unwrap().to_string();

    let muls = re
        .captures_iter(&contents)
        .map(|c| {
            let raw = c.extract::<0>().0;
            raw[4..raw.len() - 1]
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<u64>>>();

    let sum = muls.iter().map(|nums| nums[0] * nums[1]).sum::<u64>();

    println!("task1:\t{sum:}");
}

fn main() {
    task1();
    task2();
}
