use std::fs;

const INPUT: &str = "input.txt";

#[derive(Debug, Clone, Copy)]
enum Operation {
    Insert(u8),
    Remove,
}

#[derive(Debug, Clone)]
struct Cmd {
    lens: String,
    op: Operation,
}

#[derive(Debug, Clone)]
struct Lens {
    tag: String,
    focus: u8,
}

impl Cmd {
    fn new(cmd: &str) -> Cmd {
        let equal_split: Vec<_> = cmd.split("=").collect();
        let (lens, op) = if equal_split.len() == 2 {
            (
                equal_split[0].to_string(),
                Operation::Insert(equal_split[1].parse::<u8>().unwrap()),
            )
        } else {
            let mut lens = equal_split[0].to_string();
            lens.pop();
            (lens, Operation::Remove)
        };

        Cmd { lens, op }
    }
}

fn hasher(s: &str) -> u32 {
    let mut sum: u32 = 0;
    for c in s.chars() {
        sum += c as u32;
        sum *= 17;
        sum %= 256;
    }
    sum
}

fn task1() {
    let contents = fs::read_to_string(INPUT).unwrap();
    let instructions: Vec<_> = contents.lines().nth(0).unwrap().split(',').collect();
    let hashes: Vec<_> = instructions.iter().map(|s| hasher(s)).collect();

    println!("task1 {:?}", hashes.iter().sum::<u32>());
}

fn task2() {
    let contents = fs::read_to_string(INPUT).unwrap();
    let commands: Vec<_> = contents
        .lines()
        .nth(0)
        .unwrap()
        .split(',')
        .map(|c| Cmd::new(c))
        .collect();

    let mut boxes = vec![Vec::<Lens>::new(); 256];

    for command in &commands {
        let h = hasher(command.lens.as_str());
        let b = &mut boxes[h as usize];
        match command.op {
            Operation::Remove => {
                if let Some(index) = b.iter().position(|l| l.tag == command.lens) {
                    b.remove(index);
                }
            }
            Operation::Insert(f) => {
                let lens = Lens {
                    tag: command.lens.clone(),
                    focus: f,
                };

                if let Some(index) = b.iter().position(|l| l.tag == command.lens) {
                    b[index] = lens
                } else {
                    b.push(lens)
                }
            }
        }
    }

    let sum = boxes
        .iter()
        .enumerate()
        .map(|(i, b)| {
            b.iter()
                .enumerate()
                .map(|(j, l)| l.focus as usize * (i + 1) * (j + 1))
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("task2 {:?}", sum);
}

fn main() {
    task1();
    task2();
}
