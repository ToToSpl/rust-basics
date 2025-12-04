use std::fs;

const INPUT: &str = "input.txt";
// const INPUT: &str = "input.test.txt";

fn first_max<T>(v: &Vec<T>, skip_first: usize, look_at: usize) -> Option<(usize, T)>
where
    T: Copy + PartialOrd,
{
    let mut max = None;

    for (i, e) in v.iter().enumerate().skip(skip_first).take(look_at) {
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

fn best_bank_sum(bank_len: u32) -> u64 {
    let contents = fs::read_to_string(INPUT).unwrap();

    let best_banks: Vec<u64> = contents
        .lines()
        .map(|l| {
            let digits: Vec<u32> = l.chars().map(|c| c.to_digit(10).unwrap()).collect();

            let mut best_bank: u64 = 0;
            let mut mantissa = 10u64.pow(bank_len - 1);

            let mut skips = digits.len() - bank_len as usize;
            let mut i = 0;

            for _ in 0..bank_len {
                let d = if skips != 0 {
                    let (d_i, d_v) = first_max(&digits, i, skips + 1).unwrap();
                    skips -= d_i - i;
                    i = d_i + 1;

                    d_v
                } else {
                    let d = digits[i];
                    i += 1;
                    d
                };

                best_bank += d as u64 * mantissa;
                mantissa /= 10;
            }

            best_bank
        })
        .collect();

    best_banks.into_iter().sum()
}

fn main() {
    println!("task1:\t{:}", best_bank_sum(2));
    println!("task2:\t{:}", best_bank_sum(12));
}
