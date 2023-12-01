use regex::Regex;
use std::fs;

fn task1() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut sum: u32 = 0;
    for line in contents.lines() {
        let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
        if digits.len() == 1 {
            sum += 11 * digits[0];
        } else {
            sum += 10 * digits[0] + digits[digits.len() - 1];
        }
    }
    println!("{:?}", sum);
}

fn task2() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut sum: u32 = 0;
    // seperate regexes, because words can overlap
    let translators = [
        (1, Regex::new(r"(?:one)").unwrap()),
        (2, Regex::new(r"(?:two)").unwrap()),
        (3, Regex::new(r"(?:three)").unwrap()),
        (4, Regex::new(r"(?:four)").unwrap()),
        (5, Regex::new(r"(?:five)").unwrap()),
        (6, Regex::new(r"(?:six)").unwrap()),
        (7, Regex::new(r"(?:seven)").unwrap()),
        (8, Regex::new(r"(?:eight)").unwrap()),
        (9, Regex::new(r"(?:nine)").unwrap()),
    ];
    let re_digit = Regex::new(r"(?:[1-9])").unwrap();
    for line in contents.lines() {
        let mut digits: Vec<(usize, u32)> = Vec::new();
        for (num, re) in &translators {
            re.find_iter(line)
                .for_each(|m| digits.push((m.start(), num.clone())));
        }
        re_digit.find_iter(line).for_each(|m| {
            digits.push((
                m.start(),
                m.as_str().chars().nth(0).unwrap().to_digit(10).unwrap(),
            ))
        });
        digits.sort_by_key(|d| d.0);

        if digits.len() == 1 {
            sum += 11 * digits[0].1;
        } else {
            sum += 10 * digits[0].1 + digits[digits.len() - 1].1;
        }
    }
    println!("{:?}", sum);
}

fn main() {
    task1();
    task2();
}
