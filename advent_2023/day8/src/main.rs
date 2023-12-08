use core::panic;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

const INPUT: &str = "input.txt";

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Directions {
    directions: Vec<Direction>,
    index: usize,
}

impl Directions {
    fn new(line: &str) -> Directions {
        let directions = line
            .chars()
            .map(|c| match c {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _e => panic!("Unrecognized direction {_e}"),
            })
            .collect();

        Directions {
            directions,
            index: 0,
        }
    }

    fn next(&mut self) -> Direction {
        let direction = self.directions[self.index].clone();
        self.index = if self.index == self.directions.len() - 1 {
            0
        } else {
            self.index + 1
        };
        direction
    }
}

fn task1() {
    let contents = fs::read_to_string(INPUT).unwrap();
    let mut lines = contents.lines();
    let mut directions = Directions::new(lines.next().unwrap());
    lines.next().unwrap();

    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();

    let re = Regex::new(r"([A-Z]+) = \(([A-Z]+), ([A-Z]+)\)").unwrap();
    for line in lines {
        let (_, [point, left, right]) = re.captures(line).unwrap().extract();
        map.insert(point, (left, right));
    }

    let mut count: usize = 0;
    let mut curr = "AAA";
    while curr != "ZZZ" {
        count += 1;
        let next_dir = directions.next();
        let options = map[curr];
        curr = match next_dir {
            Direction::Left => options.0,
            Direction::Right => options.1,
        };
    }

    println!("task1 {:?}", count);
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm_arr(arr: Vec<usize>) -> usize {
    let mut ans = arr[0];
    for elem in arr {
        ans = (elem * ans) / gcd(elem, ans);
    }
    ans
}

// the silly little tasks is designed, such that magically all cycles ends with multiples of length
// of directions allowing to use LCM :)
fn task2() {
    let contents = fs::read_to_string(INPUT).unwrap();
    let mut lines = contents.lines();
    let directions = Directions::new(lines.next().unwrap());

    lines.next().unwrap();

    let (map, mut cycles) = {
        let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
        let mut cycles: Vec<(&str, Directions)> = Vec::new();
        let re = Regex::new(r"([0-9,A-Z]+) = \(([0-9,A-Z]+), ([0-9,A-Z]+)\)").unwrap();
        for line in lines {
            let (_, [point, left, right]) = re.captures(line).unwrap().extract();
            if point.chars().nth(2).unwrap() == 'A' {
                cycles.push((point, directions.clone()));
            }
            map.insert(point, (left, right));
        }
        (map, cycles)
    };

    let cycles: Vec<usize> = cycles
        .iter_mut()
        .map(|(start, directions)| {
            let mut count: usize = 0;
            let mut curr = *start;
            while !(curr.chars().nth(2).unwrap() == 'Z' && directions.index == 0) {
                let next_dir = directions.next();
                let options = map[curr];
                curr = match next_dir {
                    Direction::Left => options.0,
                    Direction::Right => options.1,
                };
                if directions.index == 0 {
                    count += 1;
                }
            }
            count
        })
        .collect();

    let answer = directions.directions.len() * lcm_arr(cycles);

    println!("task2 {:?}", answer);
}

fn main() {
    task1();
    task2();
}
