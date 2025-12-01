use std::fs;

const INPUT: &str = "input.txt";
// const INPUT: &str = "input.test.txt";

fn task1() {
    let contents = fs::read_to_string(INPUT).unwrap();

    let mut dial_pos = 50;
    let mut zero_point = 0;

    for l in contents.lines() {
        let (dir, count_str) = l.split_at(1);
        let count: i32 = count_str.parse().unwrap();

        let dir = match dir {
            "L" => -1,
            "R" => 1,
            _ => panic!("unkown dir"),
        };

        dial_pos = (dial_pos + dir * count) % 100;

        if dial_pos < 0 {
            dial_pos = 100 + dial_pos;
        }

        if dial_pos == 0 {
            zero_point += 1;
        }
    }

    println!("task1:\t{zero_point}");
}

fn task2() {
    let contents = fs::read_to_string(INPUT).unwrap();

    let mut dial_pos = 50;
    let mut zero_passed = 0;

    for l in contents.lines() {
        let (dir, count_str) = l.split_at(1);
        let count: i32 = count_str.parse().unwrap();

        match dir {
            "L" => {
                if count >= dial_pos && dial_pos != 0 {
                    zero_passed += 1;
                }

                zero_passed += (count - dial_pos) / 100;
                dial_pos = (100 + (dial_pos - count) % 100) % 100;
            }
            "R" => {
                zero_passed += (count + dial_pos) / 100;
                dial_pos = (count + dial_pos) % 100;
            }
            _ => panic!("unkown dir"),
        };
    }

    println!("task2:\t{zero_passed}");
}

fn main() {
    task1();
    task2();
}
