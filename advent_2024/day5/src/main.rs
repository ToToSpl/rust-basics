use std::collections::{HashMap, HashSet};
use std::fs;

const INPUT: &str = "input.txt";

fn task1() {
    let contents = fs::read_to_string(INPUT).unwrap();
    let mut lines = contents.lines();

    let mut rules: Vec<(u64, u64)> = Vec::new();
    let mut updates: Vec<Vec<u64>> = Vec::new();

    loop {
        let Some(line) = lines.next() else {
            break;
        };
        if line == "" {
            break;
        }
        let nums: Vec<u64> = line.split('|').map(|n| n.parse().unwrap()).collect();
        rules.push((nums[0], nums[1]));
    }

    loop {
        let Some(line) = lines.next() else {
            break;
        };
        updates.push(line.split(',').map(|n| n.parse().unwrap()).collect());
    }

    // reversed rules informes what numbers cannot be after the key
    let mut reversed_rules: HashMap<u64, Vec<u64>> = HashMap::new();
    rules.iter().for_each(|(a, b)| {
        reversed_rules
            .entry(*b)
            .and_modify(|r| r.push(*a))
            .or_insert(vec![*a]);
    });

    let mut count = 0;

    'outer: for update in &updates {
        let mut forbidden: HashSet<u64> = HashSet::new();
        for page in update {
            if forbidden.contains(page) {
                continue 'outer;
            }
            if let Some(rule) = reversed_rules.get(page) {
                rule.iter().for_each(|r| {
                    forbidden.insert(*r);
                });
            }
        }

        count += update[update.len() / 2];
    }

    println!("task1: {count}");
}

fn task2() {
    let contents = fs::read_to_string(INPUT).unwrap();
    let mut lines = contents.lines();

    let mut rules: Vec<(u64, u64)> = Vec::new();
    let mut updates: Vec<Vec<u64>> = Vec::new();

    loop {
        let Some(line) = lines.next() else {
            break;
        };
        if line == "" {
            break;
        }
        let nums: Vec<u64> = line.split('|').map(|n| n.parse().unwrap()).collect();
        rules.push((nums[0], nums[1]));
    }

    loop {
        let Some(line) = lines.next() else {
            break;
        };
        updates.push(line.split(',').map(|n| n.parse().unwrap()).collect());
    }

    // reversed rules informes what numbers cannot be after the key
    let mut reversed_rules: HashMap<u64, Vec<u64>> = HashMap::new();
    rules.iter().for_each(|(a, b)| {
        reversed_rules
            .entry(*b)
            .and_modify(|r| r.push(*a))
            .or_insert(vec![*a]);
    });

    let mut rules_map: HashMap<u64, Vec<u64>> = HashMap::new();
    rules.iter().for_each(|(a, b)| {
        rules_map
            .entry(*a)
            .and_modify(|r| r.push(*b))
            .or_insert(vec![*b]);
    });

    let mut bad_updates = Vec::new();

    'outer: for update in &updates {
        let mut forbidden: HashSet<u64> = HashSet::new();
        for page in update {
            if forbidden.contains(page) {
                bad_updates.push(update.clone());
                continue 'outer;
            }
            if let Some(rule) = reversed_rules.get(page) {
                rule.iter().for_each(|r| {
                    forbidden.insert(*r);
                });
            }
        }
    }

    let mut count = 0;

    for bad_update in &bad_updates {
        let mut left = bad_update.clone();
        let mut forbidden: HashSet<u64> = HashSet::new();
        left.iter().for_each(|l| {
            if let Some(rule) = rules_map.get(l) {
                rule.iter().for_each(|r| {
                    forbidden.insert(*r);
                });
            }
        });

        let mut fixed = Vec::new();

        for _ in 0..bad_update.len() {
            // find in left an element that does not break any rules
            let (next_index, next_element) = left
                .iter()
                .enumerate()
                .find(|(_i, l)| !forbidden.contains(l))
                .unwrap();
            fixed.push(*next_element);
            left.remove(next_index);
            // update forbidden rules
            forbidden = HashSet::new();
            left.iter().for_each(|l| {
                if let Some(rule) = rules_map.get(l) {
                    rule.iter().for_each(|r| {
                        forbidden.insert(*r);
                    });
                }
            });
        }

        count += fixed[fixed.len() / 2];
    }

    println!("task2: {count}");
}

fn main() {
    task1();
    task2();
}
