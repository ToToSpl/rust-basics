use std::fs;

const INPUT: &str = "input.txt";

fn task1() {
    // this problem can be solved naivly due to input
    let contents = fs::read_to_string(INPUT).unwrap();

    let shape_sizes = vec![7u32, 7, 6, 7, 5, 7];

    let problems: Vec<_> = contents
        .lines()
        .skip(30)
        .map(|l| {
            let mut v = l.split(": ");
            let mut dims = v.next().unwrap().split("x");
            let dims: (u32, u32) = (
                dims.next().unwrap().parse().unwrap(),
                dims.next().unwrap().parse().unwrap(),
            );

            let sizes: Vec<u32> = v
                .next()
                .unwrap()
                .split(" ")
                .map(|d| d.parse().unwrap())
                .collect();

            (dims, sizes)
        })
        .collect();

    let out = problems
        .iter()
        .filter(|(dim, sizes)| {
            dim.0 * dim.1
                >= sizes
                    .iter()
                    .enumerate()
                    .map(|(i, c)| c * shape_sizes[i])
                    .sum()
        })
        .count();

    println!("task1:\t{out}");
}

fn main() {
    task1();
}
