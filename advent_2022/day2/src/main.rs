const INPUT: &str = "./input.txt";

use std::fs;

// TODO: use enum to represent rock paper scissors

fn points_shape(shape: char) -> Option<u32> {
    match shape {
        'X' => Some(1),
        'Y' => Some(2),
        'Z' => Some(3),
        _ => None,
    }
}

fn points_battle(enemy: char, player: char) -> Option<u32> {
    match player {
        'X' => match enemy {
            'A' => Some(3),
            'B' => Some(0),
            'C' => Some(6),
            _ => None,
        },
        'Y' => match enemy {
            'A' => Some(6),
            'B' => Some(3),
            'C' => Some(0),
            _ => None,
        },
        'Z' => match enemy {
            'A' => Some(0),
            'B' => Some(6),
            'C' => Some(3),
            _ => None,
        },
        _ => None,
    }
}

fn set_tactic(enemy: char, tactic: char) -> Option<char> {
    match tactic {
        'X' => match enemy {
            'A' => Some('Z'),
            'B' => Some('X'),
            'C' => Some('Y'),
            _ => None,
        },
        'Y' => match enemy {
            'A' => Some('X'),
            'B' => Some('Y'),
            'C' => Some('Z'),
            _ => None,
        },
        'Z' => match enemy {
            'A' => Some('Y'),
            'B' => Some('Z'),
            'C' => Some('X'),
            _ => None,
        },
        _ => None,
    }
}

fn main() {
    let contents = fs::read_to_string(INPUT).expect("Should have been able to read the file");
    let pairs: Vec<(char, char)> = contents
        .split("\n")
        .filter(|l| l.len() != 0)
        .map(|l| (l.chars().nth(0).unwrap(), l.chars().nth(2).unwrap()))
        .collect();

    let mut won_sum = 0;
    for battle in pairs {
        let tactic = set_tactic(battle.0, battle.1).unwrap();
        won_sum += points_shape(tactic).unwrap();
        won_sum += points_battle(battle.0, tactic).unwrap();
    }

    println!("{:?}", won_sum);
}
