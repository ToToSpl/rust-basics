use std::cmp::min;
use std::str::Lines;
use std::{fs, usize};

use rayon::prelude::*;
use tqdm::tqdm;

const INPUT: &str = "input.txt";

#[derive(Debug)]
struct Map {
    input_start: usize,
    output_start: usize,
    range: usize,
}

impl Map {
    fn new(line: &str) -> Map {
        let nums: Vec<_> = line
            .split(' ')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        Map {
            output_start: nums[0],
            input_start: nums[1],
            range: nums[2],
        }
    }

    fn map(&self, input: usize) -> Option<usize> {
        if self.input_start <= input && input < self.input_start + self.range {
            return Some(self.output_start + (input - self.input_start));
        }
        None
    }
}

#[derive(Debug)]
struct Mapper {
    _name: String,
    maps: Vec<Map>,
}

impl Mapper {
    fn new(lines: &mut Lines) -> Option<Mapper> {
        let name = if let Some(desc) = lines.next() {
            desc.split(' ').nth(0).unwrap().to_string()
        } else {
            return None;
        };

        let mut maps = Vec::new();
        while let Some(line) = lines.next() {
            if line.len() > 0 {
                maps.push(Map::new(line));
            } else {
                break;
            }
        }

        maps.sort_unstable_by_key(|map| map.input_start);

        Some(Mapper { _name: name, maps })
    }

    fn map(&self, input: usize) -> usize {
        for map in &self.maps {
            if let Some(output) = map.map(input) {
                return output;
            }
        }
        return input;
    }
}

fn task1() {
    let contents = fs::read_to_string(INPUT).unwrap();
    let mut lines_iter = contents.lines();

    let seeds: Vec<_> = lines_iter
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|n| n.parse::<usize>().unwrap())
        .collect();

    lines_iter.next();

    let mut mappers = Vec::new();
    while let Some(mapper) = Mapper::new(&mut lines_iter) {
        mappers.push(mapper);
    }

    let mut smallest_location = usize::MAX;

    for seed in seeds {
        let mut stage = seed;
        for mapper in &mappers {
            stage = mapper.map(stage);
        }
        smallest_location = min(smallest_location, stage);
    }

    println!("task1: {:?}", smallest_location);
}

fn task2() {
    let contents = fs::read_to_string(INPUT).unwrap();
    let mut lines_iter = contents.lines();

    let seeds: Vec<usize> = lines_iter
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|n| n.parse::<usize>().unwrap())
        .collect();

    lines_iter.next();

    let mut mappers = Vec::new();
    while let Some(mapper) = Mapper::new(&mut lines_iter) {
        mappers.push(mapper);
    }

    let seed_ranges: Vec<(usize, usize)> = seeds.chunks(2).map(|c| (c[0], c[1])).collect();
    let smallest_location = seed_ranges
        .par_iter()
        .map(|c| {
            let mut local_smallest = usize::MAX;
            for seed in tqdm(c.0..c.0 + c.1) {
                let mut stage = seed;
                for mapper in &mappers {
                    stage = mapper.map(stage);
                }
                local_smallest = min(local_smallest, stage);
            }
            local_smallest
        })
        .min()
        .unwrap();

    println!("task2: {:?}", smallest_location);
}

fn main() {
    task1();
    task2();
}
