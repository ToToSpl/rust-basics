use std::fs;

const INPUT: &str = "input.txt";
// const INPUT: &str = "input.test.txt";

#[derive(Debug, Clone, Copy)]
enum MathOperation {
    Multiply,
    Add,
}

fn task1() {
    let table: Vec<(MathOperation, Vec<u64>)> = {
        let contents = fs::read_to_string(INPUT).unwrap();

        let mut numbers = Vec::new();
        let mut operations = Vec::new();

        for l in contents.lines() {
            for (i, symbol) in l.split_whitespace().enumerate() {
                match symbol {
                    "*" => operations.push(MathOperation::Multiply),
                    "+" => operations.push(MathOperation::Add),
                    n => {
                        let num: u64 = n.parse().unwrap();

                        if i >= numbers.len() {
                            numbers.push(vec![num])
                        } else {
                            numbers[i].push(num);
                        }
                    }
                }
            }
        }

        operations.into_iter().zip(numbers).collect()
    };

    let sum: u64 = table
        .iter()
        .map(|(op, n)| match op {
            MathOperation::Add => n.iter().sum(),
            MathOperation::Multiply => n.iter().fold(1, |acc, x| acc * x),
        })
        .sum();

    println!("task1:\t{sum}");
}

fn task2() {
    let table: Vec<(MathOperation, Vec<u64>)> = {
        let contents = fs::read_to_string(INPUT).unwrap();
        let lines: Vec<_> = contents.lines().collect();

        let digits: Vec<Vec<Option<u64>>> = lines
            .iter()
            .take(lines.len() - 1)
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        ' ' => None,
                        d => Some(d.to_digit(10).unwrap() as u64),
                    })
                    .collect()
            })
            .collect();

        let mut sets: Vec<Vec<u64>> = Vec::new();
        let mut set: Vec<u64> = Vec::new();

        for x in 0..digits[0].len() {
            let mut num = 0;
            let mut pow = 1;
            let mut was_some = false;

            for y in (0..digits.len()).rev() {
                if let Some(d) = digits[y][x] {
                    num += d * pow;
                    pow *= 10;
                    was_some = true;
                }
            }

            if was_some {
                set.push(num);
            } else {
                sets.push(set.clone());
                set.clear();
            }
        }
        sets.push(set);

        let operations: Vec<MathOperation> = contents
            .lines()
            .last()
            .unwrap()
            .split_whitespace()
            .map(|op| match op {
                "+" => MathOperation::Add,
                "*" => MathOperation::Multiply,
                _c => panic!("unknown operation: {_c}"),
            })
            .collect();

        operations.into_iter().zip(sets).collect()
    };

    let sum: u64 = table
        .iter()
        .map(|(op, n)| match op {
            MathOperation::Add => n.iter().sum(),
            MathOperation::Multiply => n.iter().fold(1, |acc, x| acc * x),
        })
        .sum();

    println!("task2:\t{sum}");
}

fn main() {
    task1();
    task2();
}
