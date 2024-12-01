use std::collections::HashMap;
use std::fs;

const INPUT: &str = "input.txt";

fn task1() {
    let contents = fs::read_to_string(INPUT).unwrap();

    let mut left_list: Vec<i64> = Vec::new();
    let mut right_list: Vec<i64> = Vec::new();

    contents.lines().into_iter().for_each(|line| {
        let mut numbers = line.split_whitespace();
        let left_num = numbers.next().unwrap().parse().unwrap();
        let right_num = numbers.next().unwrap().parse().unwrap();

        left_list.push(left_num);
        right_list.push(right_num);
    });

    left_list.sort();
    right_list.sort();

    let it = left_list.iter().zip(right_list.iter());

    let total_distance = it.map(|(l, r)| (l - r).abs()).sum::<i64>();

    println!("task1:\t{total_distance:}");
}

fn task2() {
    let contents = fs::read_to_string(INPUT).unwrap();

    let mut left_list: Vec<i64> = Vec::new();
    let mut right_list: Vec<i64> = Vec::new();

    contents.lines().into_iter().for_each(|line| {
        let mut numbers = line.split_whitespace();
        let left_num = numbers.next().unwrap().parse().unwrap();
        let right_num = numbers.next().unwrap().parse().unwrap();

        left_list.push(left_num);
        right_list.push(right_num);
    });

    let mut right_list_map: HashMap<i64, i64> = HashMap::new();
    right_list
        .iter()
        .for_each(|e| *right_list_map.entry(*e).or_insert(0) += 1);

    let total_similarity = left_list
        .iter()
        .map(|e| {
            let apperance = right_list_map.get(e).unwrap_or(&0);
            u64::try_from(*e).unwrap() * u64::try_from(*apperance).unwrap()
        })
        .sum::<u64>();

    println!("task2:\t{total_similarity:}");
}

fn main() {
    task1();
    task2();
}
