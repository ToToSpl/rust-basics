use rayon::prelude::*;
use std::fs;

const INPUT: &str = "input.txt";
// const INPUT: &str = "input.test.txt";

fn get_mask(line: &str) -> u32 {
    line.chars()
        .skip(1)
        .take_while(|&c| c != ']')
        .enumerate()
        .map(|(i, c)| match c {
            '.' => 0,
            '#' => 1 << i,
            _c => panic!("unknown char {_c}"),
        })
        .sum()
}

fn get_buttons(line: &str) -> Vec<Vec<u32>> {
    let mut out = Vec::new();

    let mut start = 0;

    for (i, c) in line.chars().enumerate() {
        match c {
            '(' => {
                start = i;
            }
            ')' => {
                let button = line[start + 1..i]
                    .split(',')
                    .map(|d| d.parse::<u32>().unwrap())
                    .collect();
                out.push(button);
            }
            _ => {}
        }
    }

    out
}

fn get_buttons_mask(line: &str) -> Vec<u32> {
    let mut out = Vec::new();

    let mut start = 0;

    for (i, c) in line.chars().enumerate() {
        match c {
            '(' => {
                start = i;
            }
            ')' => {
                let button = line[start + 1..i]
                    .split(',')
                    .map(|d| 1 << d.parse::<u32>().unwrap())
                    .sum();
                out.push(button);
            }
            _ => {}
        }
    }

    out
}

fn get_joltage(line: &str) -> Vec<u32> {
    let start = line.chars().position(|c| c == '{').unwrap() + 1;
    let end = line.chars().position(|c| c == '}').unwrap();

    line[start..end]
        .split(',')
        .map(|d| d.parse().unwrap())
        .collect()
}

fn load_data_1() -> Vec<(u32, Vec<u32>)> {
    fs::read_to_string(INPUT)
        .unwrap()
        .lines()
        .map(|l| (get_mask(l), get_buttons_mask(l)))
        .collect()
}

fn load_data_2() -> Vec<(Vec<u32>, Vec<Vec<u32>>, Vec<u32>)> {
    fs::read_to_string(INPUT)
        .unwrap()
        .lines()
        .map(|l| (get_joltage(l), get_buttons(l), get_buttons_mask(l)))
        .collect()
}

fn flips_combinations(mask: u32, buttons_masks: &Vec<u32>) -> Vec<(usize, Vec<usize>)> {
    let mut out = Vec::new();

    for i in 0..(2usize).pow(buttons_masks.len() as u32) {
        let mut cur_mask = 0;
        let mut cur_presses = 0;
        for (j, &b) in buttons_masks.iter().enumerate() {
            if (i & (1 << j)) != 0 {
                cur_mask ^= b;
                cur_presses += 1;
            }
        }

        if cur_mask == mask {
            let clicks: Vec<_> = buttons_masks
                .iter()
                .enumerate()
                .filter_map(|(ii, _)| if (i >> ii) & 1 == 1 { Some(ii) } else { None })
                .collect();
            out.push((cur_presses, clicks));
        }
    }

    out
}

fn task1() {
    let mut press_sum = 0;

    for (mask, buttons) in load_data_1() {
        let (min_presses, _) = flips_combinations(mask, &buttons)
            .into_iter()
            .min_by_key(|(p, _)| *p)
            .unwrap();

        press_sum += min_presses;
    }

    println!("task1:\t{press_sum}");
}

// could use cache but idc
fn minimize_clicks(
    current_joltage: &Vec<u32>,
    buttons: &Vec<Vec<u32>>,
    buttons_masks: &Vec<u32>,
) -> Option<u32> {
    if current_joltage.iter().all(|&d| d == 0) {
        return Some(0);
    }

    let odd_mask_joltage: u32 = current_joltage
        .iter()
        .enumerate()
        .map(|(i, v)| (v & 1) << i)
        .sum();

    let click_options = flips_combinations(odd_mask_joltage, buttons_masks);

    let mut best_click_amount = None;

    'outer: for (clicks, buttons_used) in click_options {
        let new_joltage = {
            let mut j = current_joltage.clone();

            for c in buttons_used {
                for &d in &buttons[c] {
                    if j[d as usize] == 0 {
                        continue 'outer;
                    }

                    j[d as usize] -= 1;
                }
            }

            for d in &mut j {
                *d /= 2;
            }

            j
        };

        let more_clicks = minimize_clicks(&new_joltage, buttons, buttons_masks);
        if let Some(m) = more_clicks {
            let used_clicks = clicks as u32 + 2 * m;
            if let Some(best) = best_click_amount {
                if used_clicks < best {
                    best_click_amount = Some(used_clicks);
                }
            } else {
                best_click_amount = Some(used_clicks);
            }
        }
    }

    best_click_amount
}

// most solutions use linalg to solve this as system of linear equations
// this would be easy in python but in rust we would have to implement half of scikit
// so this solution seems to be a cool one:
// https://www.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/
fn task2() {
    let data = load_data_2();

    let best: u32 = data
        .par_iter()
        .map(|(joltage, buttons, buttons_mask)| {
            let best = minimize_clicks(&joltage, &buttons, &buttons_mask);
            println!("{best:?}");
            best.unwrap()
        })
        .sum();

    println!("task2:\t{best}");
}

fn main() {
    task1();
    task2();
}
