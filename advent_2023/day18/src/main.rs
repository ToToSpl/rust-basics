use std::fs;

const INPUT: &str = "input.txt";

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    dir: Direction,
    len: i64,
}

impl Instruction {
    fn new(line: &str) -> Instruction {
        let mut parts = line.split(' ');
        let dir = match parts.next().unwrap() {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _e => panic!("Unknown symbol {:?}", _e),
        };
        let len = parts.next().unwrap().parse::<i64>().unwrap();

        Instruction { dir, len }
    }

    fn new_corrected(line: &str) -> Instruction {
        let raw_string = line.split(' ').collect::<Vec<_>>()[2];
        let mut raw_string = raw_string
            .chars()
            .skip(2)
            .filter(|c| *c != ')')
            .collect::<String>();

        let dir = match raw_string.pop().unwrap() {
            '0' => Direction::Right,
            '1' => Direction::Down,
            '2' => Direction::Left,
            '3' => Direction::Up,
            _e => panic!("Unknown symbol {:?}", _e),
        };

        let len = u64::from_str_radix(raw_string.as_str(), 16).unwrap() as i64;

        Instruction { dir, len }
    }
}

fn volume_from_instructions(instructions: &[Instruction]) -> i64 {
    let mut curr: (i64, i64) = (0, 0);
    let mut boundary: i64 = 0;
    let a_doubled = instructions
        .iter()
        .map(|instr| {
            boundary += instr.len;
            curr = match instr.dir {
                Direction::Up => (curr.0, curr.1 - instr.len),
                Direction::Down => (curr.0, curr.1 + instr.len),
                Direction::Left => (curr.0 - instr.len, curr.1),
                Direction::Right => (curr.0 + instr.len, curr.1),
            };
            curr
        })
        .collect::<Vec<_>>()
        .windows(2)
        .map(|c| c[0].0 * c[1].1 - c[0].1 * c[1].0)
        .sum::<i64>();

    assert!(a_doubled % 2 == 0);
    assert!(boundary % 2 == 0);
    a_doubled / 2 + boundary / 2 + 1
}

fn task1() {
    let contents = fs::read_to_string(INPUT).unwrap();
    let instructions = contents.lines().map(Instruction::new).collect::<Vec<_>>();

    let filled = volume_from_instructions(&instructions);
    println!("task1 {:?}", filled);
}

fn task2() {
    let contents = fs::read_to_string(INPUT).unwrap();
    let instructions = contents
        .lines()
        .map(Instruction::new_corrected)
        .collect::<Vec<_>>();

    let filled = volume_from_instructions(&instructions);
    println!("task2 {:?}", filled);
}

fn main() {
    task1();
    task2();
}
