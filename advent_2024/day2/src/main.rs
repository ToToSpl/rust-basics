use std::fs;

const INPUT: &str = "input.txt";

#[derive(PartialEq, Eq)]
enum RecordErr {
    Ok,
    Err(usize),
}

fn is_record_safe(record: &[i64]) -> RecordErr {
    let diffs: Vec<i64> = record.windows(2).map(|w| w[0] - w[1]).collect();

    let sign = diffs
        .iter()
        .map(|d| if d.is_positive() { 1 } else { -1 })
        .sum::<i64>()
        > 0;
    for (i, diff) in diffs.into_iter().enumerate() {
        if diff.is_positive() != sign || diff == 0 || diff.abs() > 3 {
            return RecordErr::Err(i);
        }
    }

    RecordErr::Ok
}

fn task1() {
    let contents = fs::read_to_string(INPUT).unwrap();

    let records: Vec<Vec<i64>> = contents
        .lines()
        .into_iter()
        .map(|line| {
            line.split_whitespace()
                .map(|l| l.parse().unwrap())
                .collect()
        })
        .collect();

    let safe_count = records
        .iter()
        .map(|r| {
            if is_record_safe(r) == RecordErr::Ok {
                1
            } else {
                0
            }
        })
        .sum::<usize>();

    println!("task1:\t{safe_count:}");
}

fn is_record_safe_dampener(record: &Vec<i64>) -> bool {
    let is_all_safe = is_record_safe(&record);

    match is_all_safe {
        RecordErr::Ok => true,
        RecordErr::Err(i) => {
            let mut record_l = record.clone();
            record_l.remove(i);
            if is_record_safe(&record_l) == RecordErr::Ok {
                return true;
            }

            let mut record_r = record.clone();
            record_r.remove(i + 1);
            if is_record_safe(&record_r) == RecordErr::Ok {
                return true;
            }
            false
        }
    }
}

fn task2() {
    let contents = fs::read_to_string(INPUT).unwrap();

    let records: Vec<Vec<i64>> = contents
        .lines()
        .into_iter()
        .map(|line| {
            line.split_whitespace()
                .map(|l| l.parse().unwrap())
                .collect()
        })
        .collect();

    let safe_count = records
        .iter()
        .map(|r| if is_record_safe_dampener(r) { 1 } else { 0 })
        .sum::<usize>();

    println!("task2:\t{safe_count:}");
}

fn main() {
    task1();
    task2();
}
