use std::fs;

const ALLOWED_RED: u32 = 12;
const ALLOWED_GREEN: u32 = 13;
const ALLOWED_BLUE: u32 = 14;

#[derive(Debug, Clone, Copy)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

impl Set {
    fn new(raw_set: &str) -> Set {
        let mut new_set = Set {
            red: 0,
            green: 0,
            blue: 0,
        };

        raw_set.split(", ").for_each(|s| {
            let splitted: Vec<&str> = s.split(" ").collect();
            let amount = splitted[0].parse::<u32>().unwrap();
            match splitted[1] {
                "red" => new_set.red = amount,
                "green" => new_set.green = amount,
                "blue" => new_set.blue = amount,
                e => panic!("unknown word: {:?}", e),
            }
        });

        new_set
    }

    fn is_possible(&self) -> bool {
        if self.red > ALLOWED_RED || self.blue > ALLOWED_BLUE || self.green > ALLOWED_GREEN {
            return false;
        };

        true
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }

    fn least_common_from(sets: &Vec<Set>) -> Set {
        Set {
            red: sets.iter().max_by_key(|s| s.red).unwrap().red,
            green: sets.iter().max_by_key(|s| s.green).unwrap().green,
            blue: sets.iter().max_by_key(|s| s.blue).unwrap().blue,
        }
    }
}

fn task1() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut id_sum: u32 = 0;
    for (i, line) in contents.lines().enumerate() {
        let game: Vec<Set> = line
            .split(": ")
            .nth(1)
            .unwrap()
            .split("; ")
            .map(|s| Set::new(s))
            .collect();

        if game.iter().all(|s| s.is_possible()) {
            id_sum += (i + 1) as u32;
        }
    }

    println!("task1: {:?}", id_sum);
}

fn task2() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut power_sum: u32 = 0;
    for line in contents.lines() {
        let game: Vec<Set> = line
            .split(": ")
            .nth(1)
            .unwrap()
            .split("; ")
            .map(|s| Set::new(s))
            .collect();

        power_sum += Set::least_common_from(&game).power();
    }

    println!("task2: {:?}", power_sum);
}

fn main() {
    task1();
    task2();
}
