use std::fs;

// const INPUT: &str = "input.txt";
const INPUT: &str = "input.test.txt";

fn first_max<T>(v: &Vec<T>, skip_first: usize, skip_last: usize) -> Option<(usize, T)>
where
    T: Copy + PartialOrd,
{
    let mut max = None;

    for (i, e) in v
        .iter()
        .enumerate()
        .take(v.len() - skip_last)
        .skip(skip_first)
    {
        if let Some((_bi, be)) = &max {
            if e > be {
                max = Some((i, *e));
            }
        } else {
            max = Some((i, *e))
        }
    }

    max
}

fn task1() {
    let contents = fs::read_to_string(INPUT).unwrap();

    let sum: u32 = contents
        .lines()
        .map(|l| {
            let digits: Vec<u32> = l.chars().map(|c| c.to_digit(10).unwrap()).collect();

            let d1 = first_max(&digits, 0, 1).unwrap();
            let d2 = first_max(&digits, d1.0 + 1, 0).unwrap();

            d1.1 * 10 + d2.1
        })
        .sum();

    println!("task1:\t{sum:?}");
}

fn task2() {
    let contents = fs::read_to_string(INPUT).unwrap();

    let sum: u128 = contents
        .lines()
        .map(|l| {
            let digits: Vec<u32> = l.chars().map(|c| c.to_digit(10).unwrap()).collect();

            0
        })
        .sum();

    println!("task2:\t{sum}");
}

fn main() {
    task1();
    task2();
}
