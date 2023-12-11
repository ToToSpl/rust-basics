use std::collections::HashSet;
use std::fs;

const INPUT: &str = "input.txt";

const GALAXY_MULTIPLIER_TASK1: i64 = 2;
const GALAXY_MULTIPLIER_TASK2: i64 = 1_000_000;

#[derive(Debug, Clone, Copy)]
struct Galaxy {
    x: i64,
    y: i64,
}

impl Galaxy {
    fn distance_between(a: &Galaxy, b: &Galaxy) -> i64 {
        i64::abs(a.x - b.x) + i64::abs(a.y - b.y)
    }
}

#[derive(Debug, Clone)]
struct Image {
    galaxies: Vec<Galaxy>,
}

impl Image {
    fn new(input: &str, galaxy_multiplier: i64) -> Image {
        let contents = fs::read_to_string(input).unwrap();
        let lines: Vec<_> = contents.lines().collect();

        let init_height = lines.len();
        let init_width = lines.first().unwrap().len();

        let mut empty_rows = HashSet::new();
        for (y, line) in lines.iter().enumerate() {
            if line.chars().all(|c| c == '.') {
                empty_rows.insert(y);
            }
        }

        let mut empty_cols = HashSet::new();
        'outer: for x in 0..init_width {
            for line in &lines {
                if line.chars().nth(x).unwrap() != '.' {
                    continue 'outer;
                }
            }
            empty_cols.insert(x);
        }

        let mut galaxies = Vec::new();

        {
            let mut y_gala: i64 = 0;
            for y in 0..init_width {
                let mut x_gala: i64 = 0;
                for x in 0..init_height {
                    if lines[y].chars().nth(x).unwrap() == '#' {
                        galaxies.push(Galaxy {
                            y: y_gala,
                            x: x_gala,
                        })
                    }

                    x_gala += if empty_cols.contains(&x) {
                        galaxy_multiplier
                    } else {
                        1
                    };
                }
                y_gala += if empty_rows.contains(&y) {
                    galaxy_multiplier
                } else {
                    1
                };
            }
        }

        Image { galaxies }
    }
}

fn task1() {
    let image = Image::new(INPUT, GALAXY_MULTIPLIER_TASK1);

    let mut sum = 0;
    for a_i in 0..image.galaxies.len() {
        let a = &image.galaxies[a_i];
        for b_i in a_i + 1..image.galaxies.len() {
            let b = &image.galaxies[b_i];
            sum += Galaxy::distance_between(a, b);
        }
    }

    println!("task1 {:?}", sum);
}

fn task2() {
    let image = Image::new(INPUT, GALAXY_MULTIPLIER_TASK2);

    let mut sum = 0;
    for a_i in 0..image.galaxies.len() {
        let a = &image.galaxies[a_i];
        for b_i in a_i + 1..image.galaxies.len() {
            let b = &image.galaxies[b_i];
            sum += Galaxy::distance_between(a, b);
        }
    }

    println!("task2 {:?}", sum);
}

fn main() {
    task1();
    task2();
}
