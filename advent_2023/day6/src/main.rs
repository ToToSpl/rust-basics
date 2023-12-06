use std::fs;
use std::iter::zip;

const INPUT: &str = "input.txt";

#[derive(Debug)]
struct RaceRecord {
    time: usize,
    distance: usize,
}

impl RaceRecord {
    fn count_winnings(&self) -> usize {
        for i in 1..self.time {
            if i * (self.time - i) > self.distance {
                return (self.time - 1) - 2 * (i - 1);
            }
        }
        0
    }
}

fn task1() {
    let records: Vec<RaceRecord> = {
        let contents = fs::read_to_string(INPUT).unwrap();
        let lines: Vec<&str> = contents.lines().collect();
        let extract = |line: &str| -> Vec<_> {
            line.split_whitespace()
                .skip(1)
                .map(|t| t.parse::<usize>().unwrap())
                .collect()
        };
        zip(extract(lines[0]), extract(lines[1]))
            .map(|s| RaceRecord {
                time: s.0,
                distance: s.1,
            })
            .collect()
    };

    let winnings: usize = records
        .iter()
        .map(|r| r.count_winnings())
        .fold(1, |acc, w| acc * w);

    println!("task1: {:?}", winnings);
}

fn task2() {
    let record: RaceRecord = {
        let contents = fs::read_to_string(INPUT).unwrap();
        let lines: Vec<_> = contents.lines().collect();
        let extract = |line: &str| -> usize {
            line.split_whitespace()
                .skip(1)
                .collect::<Vec<&str>>()
                .join("")
                .parse::<usize>()
                .unwrap()
        };
        RaceRecord {
            time: extract(lines[0]),
            distance: extract(lines[1]),
        }
    };

    println!("task2: {:?}", record.count_winnings());
}

fn main() {
    task1();
    task2();
}
