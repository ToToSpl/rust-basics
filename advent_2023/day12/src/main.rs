use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use std::collections::HashMap;
use std::{fs, iter::zip};

const INPUT: &str = "input.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum SpringType {
    Working,
    Broken,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ChunkCheck {
    broken: Vec<u32>,
    index: usize,
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
        fn recur(
            springs: &Vec<SpringType>,
            broken: &Vec<u32>,
            chunk: &mut ChunkCheck,
            dict: &mut HashMap<ChunkCheck, usize>,
            count: u32,
        ) -> usize {
            if count == 0 {
                if let Some(combs) = dict.get(chunk) {
                    return *combs;
                }
            }

            if chunk.index >= springs.len() {
                if count != 0 {
                    chunk.broken.push(count);
                }
                return if broken.len() == chunk.broken.len()
                    && zip(broken, &chunk.broken)
                        .filter(|(a, b)| **a != **b)
                        .count()
                        == 0
                {
                    1
                } else {
                    0
                };
            }

            let mut count = count;
            let curr_spring = springs[chunk.index];

            if curr_spring == SpringType::Working {
                if count != 0 {
                    chunk.broken.push(count);
                    if zip(broken, &chunk.broken)
                        .filter(|(a, b)| **a != **b)
                        .count()
                        != 0
                    {
                        return 0;
                    }
                    count = 0;
                }
                chunk.index += 1;
                let combs = recur(springs, broken, chunk, dict, count);
                if count == 0 {
                    dict.insert(chunk.clone(), combs);
                }
                return combs;
            }

            if curr_spring == SpringType::Broken {
                count += 1;
                chunk.index += 1;
                return recur(springs, broken, chunk, dict, count);
            }

            return match Record::propose_next_spring(chunk, broken, count) {
                Some(opts) => {
                    let mut combs: usize = 0;
                    for opt in opts {
                        let mut new_chunk = chunk.clone();
                        let mut new_count = count;
                        if opt == SpringType::Working {
                            if new_count != 0 {
                                new_chunk.broken.push(new_count);
                                if zip(broken, &new_chunk.broken)
                                    .filter(|(a, b)| **a != **b)
                                    .count()
                                    != 0
                                {
                                    continue;
                                }

                                new_count = 0;
                            }
                        } else {
                            new_count += 1;
                        }
                        new_chunk.index += 1;
                        let comb = recur(springs, broken, &mut new_chunk, dict, new_count);
                        if new_count == 0 {
                            dict.insert(new_chunk, comb);
                        }
                        combs += comb;
                    }
                    if count == 0 {
                        dict.insert(chunk.clone(), combs);
                    }
                    combs
                }
                None => 0,
            };
        }
        let mut dict: HashMap<ChunkCheck, usize> = HashMap::new();
        let mut chunk = ChunkCheck {
            index: 0,
            broken: vec![],
        };
        recur(&self.springs, &self.broken, &mut chunk, &mut dict, 0)
    }

    fn propose_next_spring(
        chunk: &ChunkCheck,
        broken: &Vec<u32>,
        count: u32,
    ) -> Option<Vec<SpringType>> {
        if chunk.broken.len() > broken.len() {
            return None;
        } else if chunk.broken.len() == broken.len() {
            if chunk.broken.last().unwrap() != broken.last().unwrap() {
                return None;
            }
            return Some(vec![SpringType::Working]);
        }

        if count == 0 {
            return Some(vec![SpringType::Working, SpringType::Broken]);
        }

        let target = broken[chunk.broken.len()];

        return if count > target {
            None
        } else if count < target {
            Some(vec![SpringType::Broken])
        } else {
            Some(vec![SpringType::Working])
        };
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
