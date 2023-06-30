const INPUT: &str = "./input.txt";

use std::fs;

fn range_raw_to_int(range: &str) -> (i32, i32) {
    let left_right: Vec<&str> = range.split("-").collect();
    let left = left_right[0].parse::<i32>().unwrap();
    let right = left_right[1].parse::<i32>().unwrap();
    (left, right)
}

fn main() {
    let contents = fs::read_to_string(INPUT).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.split("\n").filter(|l| l.len() != 0).collect();

    let mut sum = 0;
    for line in lines {
        let ranges: Vec<&str> = line.split(",").collect();
        let range_a = range_raw_to_int(ranges[0]);
        let range_b = range_raw_to_int(ranges[1]);

        let a_in_b = range_a.0 >= range_b.0 && range_a.1 <= range_b.1;
        let b_in_a = range_b.0 >= range_a.0 && range_b.1 <= range_a.1;
        if a_in_b || b_in_a {
            sum += 1;
        }
    }
    println!("{:?}", sum);
}
