use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use std::{fs, iter::zip};

const INPUT: &str = "input.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum SpringType {
    Working,
    Broken,
    Unknown,
}

#[derive(Debug, Clone)]
struct Record {
    springs: Vec<SpringType>,
    broken: Vec<u32>,
}

impl Record {
    fn new(line: &str) -> Record {
        let splitted: Vec<_> = line.split(' ').collect();
        let springs: Vec<_> = splitted[0]
            .chars()
            .map(|c| match c {
                '.' => SpringType::Working,
                '#' => SpringType::Broken,
                '?' => SpringType::Unknown,
                _ => panic!("Can't parse ${:?} to SpringType.", c),
            })
            .collect();

        let broken: Vec<_> = splitted[1]
            .split(',')
            .map(|c| c.parse::<u32>().unwrap())
            .collect();

        Record { springs, broken }
    }

    fn new_unfold(line: &str, unfolds: usize) -> Record {
        let splitted: Vec<_> = line.split(' ').collect();
        let mut springs: Vec<_> = splitted[0]
            .chars()
            .map(|c| match c {
                '.' => SpringType::Working,
                '#' => SpringType::Broken,
                '?' => SpringType::Unknown,
                _ => panic!("Can't parse ${:?} to SpringType.", c),
            })
            .collect();
        springs.push(SpringType::Unknown);

        let mut springs: Vec<SpringType> = springs
            .iter()
            .cycle()
            .take(springs.len() * unfolds)
            .map(|s| *s)
            .collect();
        springs.pop();

        let broken: Vec<_> = splitted[1]
            .split(',')
            .map(|c| c.parse::<u32>().unwrap())
            .collect();

        let broken: Vec<u32> = broken
            .iter()
            .cycle()
            .take(broken.len() * unfolds)
            .map(|b| *b)
            .collect();

        Record { springs, broken }
    }

    fn count_correct_combs(&self) -> usize {
        fn recur(index: usize, springs: &Vec<SpringType>, broken: &Vec<u32>) -> usize {
            if index >= springs.len() {
                return if Record::check_correctness(springs, broken) {
                    1
                } else {
                    0
                };
            }

            if springs[index] != SpringType::Unknown {
                return recur(index + 1, springs, broken);
            }

            let opts = if index == 0 {
                Some(vec![SpringType::Working, SpringType::Broken])
            } else {
                Record::propose_next_spring(springs, broken, index)
            };

            return match opts {
                Some(opts) => {
                    let mut combs: usize = 0;
                    for opt in opts {
                        let mut arr = springs.clone();
                        arr[index] = opt;
                        combs += recur(index + 1, &arr, broken)
                    }
                    combs
                }
                None => 0,
            };
        }
        recur(0, &self.springs, &self.broken)
    }

    fn propose_next_spring(
        springs: &Vec<SpringType>,
        broken: &Vec<u32>,
        index: usize,
    ) -> Option<Vec<SpringType>> {
        let mut prev = springs[0];
        let mut cycles = 0;
        let mut count = 0;
        for i in 0..index {
            let spring = springs[i];
            if spring == SpringType::Broken {
                count += 1;
            } else if prev == SpringType::Broken {
                cycles += 1;
                count = 0;
            }
            prev = spring;
        }

        if broken.len() <= cycles {
            return Some(vec![SpringType::Working]);
        }

        if count == 0 {
            return Some(vec![SpringType::Working, SpringType::Broken]);
        }

        return if count > broken[cycles] {
            None
        } else if count < broken[cycles] {
            Some(vec![SpringType::Broken])
        } else {
            Some(vec![SpringType::Working])
        };
    }

    fn print_springs(springs: &Vec<SpringType>) {
        for spring in springs {
            match spring {
                SpringType::Broken => print!("#"),
                SpringType::Working => print!("."),
                SpringType::Unknown => print!("?"),
            }
        }
        println!("");
    }

    fn check_correctness(springs: &Vec<SpringType>, broken: &Vec<u32>) -> bool {
        if springs
            .iter()
            .find(|&&s| s == SpringType::Unknown)
            .is_some()
        {
            return false;
        }

        let mut broken_in_springs = Vec::new();
        let mut prev = springs[0];
        let mut count = 0;
        for spring in springs {
            if *spring == SpringType::Broken {
                count += 1;
            } else if prev == SpringType::Broken {
                broken_in_springs.push(count);
                count = 0;
            }
            prev = *spring;
        }
        if count != 0 {
            broken_in_springs.push(count);
        }

        if broken_in_springs.len() != broken.len() {
            return false;
        }

        zip(broken_in_springs, broken)
            .filter(|(a, b)| *a != **b)
            .count()
            == 0
    }
}

fn task1() {
    let contents = fs::read_to_string(INPUT).unwrap();
    let records: Vec<_> = contents.lines().map(|line| Record::new(line)).collect();
    let combs: Vec<_> = records.iter().map(|r| r.count_correct_combs()).collect();
    println!("task1 {:?}", combs.iter().sum::<usize>());
}

fn task2() {
    let contents = fs::read_to_string(INPUT).unwrap();
    let records: Vec<_> = contents
        .lines()
        .map(|line| Record::new_unfold(line, 5))
        .collect();

    let combs: Vec<_> = records
        .par_iter()
        .progress_count(records.len() as u64)
        .map(|r| r.count_correct_combs())
        .collect();

    println!("task2 {:?}", combs.iter().sum::<usize>());
}

fn main() {
    task1();
    task2();
}
