use std::collections::{HashSet, VecDeque};
use std::fs;

const INPUT: &str = "input.txt";
const DOUBLER: u32 = 2;

fn task1() {
    let contents = fs::read_to_string(INPUT).unwrap();
    let mut points_sum: usize = 0;

    for line in contents.lines() {
        let card: Vec<Vec<u32>> = line
            .split(':')
            .nth(1)
            .unwrap()
            .split('|')
            .map(|s| {
                s.split(' ')
                    .filter(|n| !n.is_empty())
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect()
            })
            .collect();

        let winning: HashSet<u32> = HashSet::from_iter(card[0].iter().cloned());
        let mut matching: u32 = 0;
        for num in &card[1] {
            if winning.contains(num) {
                matching += 1;
            }
        }

        if matching > 0 {
            points_sum += DOUBLER.pow(matching - 1) as usize;
        }
    }
    println!("task1: {:?}", points_sum);
}

#[derive(Debug)]
struct ScratchCard {
    _id: u32,
    matches: usize,
}

fn task2() {
    let cards: Vec<ScratchCard> = fs::read_to_string(INPUT)
        .unwrap()
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let card: Vec<Vec<u32>> = line
                .split(':')
                .nth(1)
                .unwrap()
                .split('|')
                .map(|s| {
                    s.split(' ')
                        .filter(|n| !n.is_empty())
                        .map(|n| n.parse::<u32>().unwrap())
                        .collect()
                })
                .collect();

            let winning: HashSet<u32> = HashSet::from_iter(card[0].iter().cloned());
            let matches = card[1].iter().filter(|num| winning.contains(&num)).count();

            ScratchCard {
                _id: i as u32,
                matches,
            }
        })
        .collect();

    let mut scratch_cards: usize = 0;
    let mut stack = VecDeque::from_iter(0..cards.len());

    while let Some(id) = stack.pop_front() {
        if id >= cards.len() {
            continue;
        }
        scratch_cards += 1;

        for i in id + 1..id + 1 + cards[id].matches {
            stack.push_back(i);
        }
    }

    println!("task2: {:?}", scratch_cards);
}

fn main() {
    task1();
    task2();
}
