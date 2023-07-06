const INPUT: &str = "./input.txt";

use regex::Regex;
use std::collections::VecDeque;
use std::fs;

type Stacks = Vec<VecDeque<char>>;

fn get_stacks(lines: &Vec<&str>) -> Stacks {
    const SPACE_JUMP: usize = 4;
    let mut stacks: Stacks = vec![VecDeque::new(); (lines[0].len() + 1) / SPACE_JUMP];
    {
        let mut i = 0;
        while lines[i].chars().nth(1).unwrap() >= 'A' && lines[i].chars().nth(1).unwrap() <= 'Z' {
            let mut j_l = 0;
            for j in (1..lines[i].len()).step_by(SPACE_JUMP) {
                let c = lines[i].chars().nth(j).unwrap();
                if c == ' ' {
                    j_l += 1;
                    continue;
                }
                stacks[j_l].push_front(c.clone());
                j_l += 1;
            }
            i += 1;
        }
    }
    stacks
}

fn task_1() -> String {
    let contents = fs::read_to_string(INPUT).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.split("\n").filter(|l| l.len() != 0).collect();

    let mut stacks = get_stacks(&lines);

    let mut start_index = 0;
    for i in 0..lines.len() {
        if lines[i].chars().nth(0).unwrap() == 'm' {
            start_index = i;
            break;
        }
    }

    let re = Regex::new(r"\d+").unwrap();

    for i in start_index..lines.len() {
        let caps: Vec<u32> = re
            .captures_iter(lines[i])
            .map(|x| x.get(0).unwrap().as_str().parse::<u32>().unwrap())
            .collect();
        let amount = caps[0] as usize;
        let from = (caps[1] - 1) as usize;
        let dest = (caps[2] - 1) as usize;

        for _ in 0..amount {
            let elem = stacks[from].pop_back().unwrap();
            stacks[dest].push_back(elem);
        }
    }

    let mut answer = "".to_string();
    for i in 0..stacks.len() {
        answer.push(stacks[i].back().unwrap().clone())
    }
    answer
}

fn task_2() -> String {
    let contents = fs::read_to_string(INPUT).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.split("\n").filter(|l| l.len() != 0).collect();

    let mut stacks = get_stacks(&lines);

    let mut start_index = 0;
    for i in 0..lines.len() {
        if lines[i].chars().nth(0).unwrap() == 'm' {
            start_index = i;
            break;
        }
    }

    let re = Regex::new(r"\d+").unwrap();

    for i in start_index..lines.len() {
        let caps: Vec<u32> = re
            .captures_iter(lines[i])
            .map(|x| x.get(0).unwrap().as_str().parse::<u32>().unwrap())
            .collect();
        let amount = caps[0] as usize;
        let from = (caps[1] - 1) as usize;
        let dest = (caps[2] - 1) as usize;

        let mut temp: Vec<char> = vec![0x0 as char; amount];
        for j in (0..amount).rev() {
            temp[j] = stacks[from].pop_back().unwrap();
        }
        for j in 0..amount {
            stacks[dest].push_back(temp[j].clone());
        }
    }

    let mut answer = "".to_string();
    for i in 0..stacks.len() {
        answer.push(stacks[i].back().unwrap().clone())
    }
    answer
}

fn main() {
    let task_1 = task_1();
    println!("{:?}", task_1);

    let task_2 = task_2();
    println!("{:?}", task_2);
}
