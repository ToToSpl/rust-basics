use std::fs;

const INPUT: &str = "input.txt";
// const INPUT: &str = "input.test.txt";

fn to_digits(n: u64) -> Vec<u8> {
    let mut n = n;
    let mut d = Vec::new();

    while n > 0 {
        d.push((n % 10) as u8);
        n = n / 10;
    }

    d
}

fn is_invalid_1(n: u64) -> bool {
    let d = to_digits(n);

    if d.len() % 2 == 1 {
        return false;
    }

    let half_start = d.len() / 2;

    for i in 0..half_start {
        if d[i] != d[half_start + i] {
            return false;
        }
    }

    true
}

fn is_invalid_2_sub(d: &Vec<u8>, s: usize) -> bool {
    if d.len() % s == 1 {
        return false;
    }

    let step = d.len() / s;
    let mut curr = 0;
    let mut next = 0 + step;

    while next < d.len() {
        for i in 0..step {
            if d[curr + i] != d[next + i] {
                return false;
            }
        }

        curr = next;
        next = curr + step;
    }

    true
}

fn is_invalid_2(n: u64) -> bool {
    let d = to_digits(n);

    for s in 2..=d.len() {
        if is_invalid_2_sub(&d, s) {
            return true;
        }
    }

    false
}

fn task1() {
    let contents = fs::read_to_string(INPUT).unwrap();

    let ranges: Vec<(u64, u64)> = contents
        .lines()
        .next()
        .unwrap()
        .split(',')
        .into_iter()
        .map(|s| {
            let mut r = s.split('-');
            (
                r.next().unwrap().parse().unwrap(),
                r.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let sum: u64 = ranges
        .into_iter()
        .map(|(l, h)| {
            let mut s = 0;
            for i in l..=h {
                if is_invalid_1(i) {
                    s += i;
                }
            }

            s
        })
        .sum();

    println!("task1\t{sum}");
}

fn task2() {
    let contents = fs::read_to_string(INPUT).unwrap();

    let ranges: Vec<(u64, u64)> = contents
        .lines()
        .next()
        .unwrap()
        .split(',')
        .into_iter()
        .map(|s| {
            let mut r = s.split('-');
            (
                r.next().unwrap().parse().unwrap(),
                r.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let sum: u64 = ranges
        .into_iter()
        .map(|(l, h)| {
            let mut s = 0;
            for i in l..=h {
                if is_invalid_2(i) {
                    s += i;
                }
            }

            s
        })
        .sum();

    println!("task2\t{sum}");
}

fn main() {
    task1();
    task2();
}
