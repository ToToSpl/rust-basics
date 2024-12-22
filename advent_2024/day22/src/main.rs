use std::{collections::HashMap, fs};

const INPUT: &str = "input.txt";

fn hash_step(input: &mut usize) {
    let mut output = *input;

    let h1 = output << 6;
    output ^= h1;
    output &= 0xFF_FF_FF;

    let h2 = output >> 5;
    output ^= h2;
    output &= 0xFF_FF_FF;

    let h3 = output << 11;
    output ^= h3;
    output &= 0xFF_FF_FF;

    *input = output;
}

fn task1() {
    let contents = fs::read_to_string(INPUT).unwrap();

    let secret_numbers: Vec<usize> = contents.lines().map(|l| l.parse().unwrap()).collect();

    let score: usize = secret_numbers
        .iter()
        .map(|n| {
            let mut hash = *n;
            for _ in 0..2000 {
                hash_step(&mut hash);
            }

            hash
        })
        .sum();

    println!("task1:\t{score}");
}

fn task2() {
    let contents = fs::read_to_string(INPUT).unwrap();

    let secret_numbers: Vec<usize> = contents.lines().map(|l| l.parse().unwrap()).collect();
    let hash_sequences_with_secret: Vec<Vec<i8>> = secret_numbers
        .iter()
        .map(|n| {
            let mut hash = *n;
            let mut out = vec![0; 2001];
            out[0] = (n % 10) as i8;
            for i in 1..2001 {
                hash_step(&mut hash);
                out[i] = (hash % 10) as i8
            }

            out
        })
        .collect();

    let hash_sequences: Vec<Vec<i8>> = hash_sequences_with_secret
        .iter()
        .map(|v| v.clone().into_iter().skip(1).collect())
        .collect();

    let hash_diffs: Vec<Vec<i8>> = hash_sequences_with_secret
        .iter()
        .map(|s| s.windows(2).map(|w| w[1] - w[0]).collect())
        .collect();

    let mut hash_map: HashMap<(i8, i8, i8, i8), Vec<Option<i8>>> = HashMap::new();

    for (i, (s, d)) in hash_sequences.iter().zip(hash_diffs).enumerate() {
        for j in 0..d.len() - 3 {
            let window = (d[j], d[j + 1], d[j + 2], d[j + 3]);
            let entry = hash_map.entry(window).or_insert(vec![None; s.len()]);
            if entry[i].is_none() {
                entry[i] = Some(s[j + 3]);
            }
        }
    }

    let banana_sums: Vec<usize> = hash_map
        .values()
        .map(|v| {
            v.iter()
                .map(|b| match b {
                    Some(b) => *b as usize,
                    None => 0,
                })
                .sum()
        })
        .collect();

    let score = banana_sums.iter().max().unwrap();

    println!("task2:\t{score}");
}

fn main() {
    task1();
    task2();
}
