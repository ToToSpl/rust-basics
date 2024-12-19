use std::{
    collections::{HashMap, HashSet},
    fs,
};

const INPUT: &str = "input.txt";

fn check_towel(sub_string: &str, stripes: &HashSet<&str>) -> bool {
    let len = sub_string.len();
    if len == 0 {
        return true;
    }

    for i in 0..len + 1 {
        if stripes.contains(&sub_string[0..i]) {
            if check_towel(&sub_string[i..], stripes) {
                return true;
            }
        }
    }

    false
}

fn count_towel(
    sub_string: &str,
    stripes: &HashSet<&str>,
    memoization: &mut HashMap<usize, usize>,
) -> usize {
    let len = sub_string.len();
    if len == 0 {
        return 1;
    }

    if let Some(&combs) = memoization.get(&len) {
        return combs;
    }

    let mut count = 0;

    for i in 0..len + 1 {
        if stripes.contains(&sub_string[0..i]) {
            count += count_towel(&sub_string[i..], stripes, memoization);
        }
    }

    memoization.insert(len, count);
    count
}

fn task1() {
    let contents = fs::read_to_string(INPUT).unwrap();
    let mut lines = contents.lines();

    let stripes: HashSet<&str> = lines.next().unwrap().split(", ").collect();
    lines.next().unwrap();
    let towels: Vec<&str> = lines.collect();

    let count: usize = towels
        .iter()
        .map(|t| if check_towel(t, &stripes) { 1 } else { 0 })
        .sum();

    println!("task1:\t{count}");
}

fn task2() {
    let contents = fs::read_to_string(INPUT).unwrap();
    let mut lines = contents.lines();

    let stripes: HashSet<&str> = lines.next().unwrap().split(", ").collect();
    lines.next().unwrap();
    let towels: Vec<&str> = lines.collect();

    let count: usize = towels
        .iter()
        .map(|t| count_towel(t, &stripes, &mut HashMap::new()))
        .sum();

    println!("task2:\t{count}");
}

fn main() {
    task1();
    task2();
}
