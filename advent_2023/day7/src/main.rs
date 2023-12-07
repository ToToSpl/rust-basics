use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::iter::zip;

const INPUT: &str = "input.txt";

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum HandType {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeOfKind = 3,
    FullHouse = 4,
    FourOfKind = 5,
    FiveOfKind = 6,
}

#[derive(Debug)]
struct Hand {
    hand_type: HandType,
    cards: [u8; 5],
    bid: usize,
}

impl Hand {
    fn new(line: &str) -> Hand {
        let splitted: Vec<_> = line.split(' ').collect();
        let bid = splitted[1].parse::<usize>().unwrap();
        let cards: Vec<char> = splitted[0].chars().collect();
        let mut collapsed: Vec<u8> = {
            let mut dict: HashMap<char, u8> = HashMap::new();
            for card in cards.to_vec() {
                *dict.entry(card).or_insert(0) += 1;
            }
            dict.iter().map(|(_, v)| *v).collect()
        };
        collapsed.sort_unstable();
        collapsed.reverse();

        let hand_type: HandType = match collapsed[0] {
            5 => HandType::FiveOfKind,
            4 => HandType::FourOfKind,
            3 => {
                if collapsed[1] == 2 {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfKind
                }
            }
            2 => {
                if collapsed[1] == 2 {
                    HandType::TwoPair
                } else {
                    HandType::OnePair
                }
            }
            1 => HandType::HighCard,
            _ => panic!("Not found hand type for ${:?}", cards),
        };

        let cards: [u8; 5] = cards
            .iter()
            .map(|c| match c.to_digit(10) {
                Some(n) => n as u8,
                None => match c {
                    'T' => 10,
                    'J' => 11,
                    'Q' => 12,
                    'K' => 13,
                    'A' => 14,
                    _ => panic!("Wrong innput card: {:?}", c),
                },
            })
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();

        Hand {
            cards,
            bid,
            hand_type,
        }
    }

    fn new_with_joker(line: &str) -> Hand {
        let splitted: Vec<_> = line.split(' ').collect();
        let bid = splitted[1].parse::<usize>().unwrap();
        let cards: Vec<char> = splitted[0].chars().collect();

        let mut jokers: u8 = 0;
        let mut collapsed: Vec<u8> = {
            let mut dict: HashMap<char, u8> = HashMap::new();
            for card in cards.to_vec() {
                if card == 'J' {
                    jokers += 1;
                } else {
                    *dict.entry(card).or_insert(0) += 1;
                }
            }
            dict.iter().map(|(_, v)| *v).collect()
        };
        if collapsed.len() == 0 {
            collapsed.push(jokers);
        } else {
            collapsed.sort_unstable();
            collapsed.reverse();
            collapsed[0] += jokers;
        }

        let hand_type: HandType = match collapsed[0] {
            5 => HandType::FiveOfKind,
            4 => HandType::FourOfKind,
            3 => {
                if collapsed[1] == 2 {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfKind
                }
            }
            2 => {
                if collapsed[1] == 2 {
                    HandType::TwoPair
                } else {
                    HandType::OnePair
                }
            }
            1 => HandType::HighCard,
            _ => panic!("Not found hand type for ${:?}", cards),
        };

        let cards: [u8; 5] = cards
            .iter()
            .map(|c| match c.to_digit(10) {
                Some(n) => n as u8,
                None => match c {
                    'J' => 1,
                    'T' => 10,
                    'Q' => 12,
                    'K' => 13,
                    'A' => 14,
                    _ => panic!("Wrong innput card: {:?}", c),
                },
            })
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();

        Hand {
            cards,
            bid,
            hand_type,
        }
    }

    fn cmp(a: &Hand, b: &Hand) -> Ordering {
        if a.hand_type != b.hand_type {
            return (a.hand_type.clone() as u8).cmp(&(b.hand_type.clone() as u8));
        } else {
            for (a_c, b_c) in zip(a.cards, b.cards) {
                if a_c != b_c {
                    return a_c.cmp(&b_c);
                }
            }
        }
        Ordering::Equal
    }
}

fn task1() {
    let contents = fs::read_to_string(INPUT).unwrap();

    let mut hands: Vec<Hand> = contents.lines().map(|l| Hand::new(l)).collect();
    hands.sort_unstable_by(|a, b| Hand::cmp(a, b));

    let sum_hands: usize = hands.iter().enumerate().map(|(i, h)| (i + 1) * h.bid).sum();
    println!("task1: {:?}", sum_hands);
}

fn task2() {
    let contents = fs::read_to_string(INPUT).unwrap();

    let mut hands: Vec<Hand> = contents.lines().map(|l| Hand::new_with_joker(l)).collect();
    hands.sort_unstable_by(|a, b| Hand::cmp(a, b));

    let sum_hands: usize = hands.iter().enumerate().map(|(i, h)| (i + 1) * h.bid).sum();
    println!("task2: {:?}", sum_hands);
}

fn main() {
    task1();
    task2();
}
