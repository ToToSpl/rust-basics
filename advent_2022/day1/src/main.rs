const INPUT: &str = "./input.txt";

use std::fs;

fn main() {
    let contents = fs::read_to_string(INPUT).expect("Should have been able to read the file");
    let mut splitted: Vec<i32> = contents
        .split("\n\n")
        .map(|elf_char| {
            elf_char
                .split("\n")
                .filter(|x| x.len() != 0)
                .map(|x| x.parse::<i32>().unwrap())
                .sum()
        })
        .collect();
    splitted.sort_unstable();
    let mut sum = 0;

    for i in splitted.len() - 3..splitted.len() {
        sum += splitted[i];
    }

    println!("{:?}", sum);
}
