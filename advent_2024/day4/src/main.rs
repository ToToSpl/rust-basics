use std::fs;

const INPUT: &str = "input.txt";

const KERNEL_SIZE: usize = 4;
const MAS_KERNEL_SIZE: usize = 3;

type Matrix = Vec<Vec<char>>;

fn check_horizontal(m: &Matrix, y: usize, x: usize) -> bool {
    m[y][x] == 'X' && m[y][x + 1] == 'M' && m[y][x + 2] == 'A' && m[y][x + 3] == 'S'
}

fn check_horizontal_rev(m: &Matrix, y: usize, x: usize) -> bool {
    m[y][x] == 'S' && m[y][x + 1] == 'A' && m[y][x + 2] == 'M' && m[y][x + 3] == 'X'
}

fn check_vertical(m: &Matrix, y: usize, x: usize) -> bool {
    m[y][x] == 'X' && m[y + 1][x] == 'M' && m[y + 2][x] == 'A' && m[y + 3][x] == 'S'
}

fn check_vertical_rev(m: &Matrix, y: usize, x: usize) -> bool {
    m[y][x] == 'S' && m[y + 1][x] == 'A' && m[y + 2][x] == 'M' && m[y + 3][x] == 'X'
}

fn check_diagonal_l(m: &Matrix, y: usize, x: usize) -> bool {
    m[y][x] == 'X' && m[y + 1][x + 1] == 'M' && m[y + 2][x + 2] == 'A' && m[y + 3][x + 3] == 'S'
}

fn check_diagonal_l_rev(m: &Matrix, y: usize, x: usize) -> bool {
    m[y][x] == 'S' && m[y + 1][x + 1] == 'A' && m[y + 2][x + 2] == 'M' && m[y + 3][x + 3] == 'X'
}

fn check_diagonal_r(m: &Matrix, y: usize, x: usize) -> bool {
    m[y][x + 3] == 'X' && m[y + 1][x + 2] == 'M' && m[y + 2][x + 1] == 'A' && m[y + 3][x] == 'S'
}

fn check_diagonal_r_rev(m: &Matrix, y: usize, x: usize) -> bool {
    m[y][x + 3] == 'S' && m[y + 1][x + 2] == 'A' && m[y + 2][x + 1] == 'M' && m[y + 3][x] == 'X'
}

fn task1() {
    let contents = fs::read_to_string(INPUT).unwrap();

    let mut matrix: Matrix = Vec::new();

    for line in contents.lines() {
        matrix.push(line.chars().collect())
    }

    let mut count = 0;

    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() - KERNEL_SIZE + 1 {
            if check_horizontal(&matrix, y, x) {
                count += 1
            };

            if check_horizontal_rev(&matrix, y, x) {
                count += 1
            };
        }
    }

    for y in 0..matrix.len() - KERNEL_SIZE + 1 {
        for x in 0..matrix[y].len() {
            if check_vertical(&matrix, y, x) {
                count += 1
            };

            if check_vertical_rev(&matrix, y, x) {
                count += 1
            };
        }
    }

    for y in 0..matrix.len() - KERNEL_SIZE + 1 {
        for x in 0..matrix[y].len() - KERNEL_SIZE + 1 {
            if check_diagonal_l(&matrix, y, x) {
                count += 1
            };

            if check_diagonal_l_rev(&matrix, y, x) {
                count += 1
            };

            if check_diagonal_r(&matrix, y, x) {
                count += 1
            };

            if check_diagonal_r_rev(&matrix, y, x) {
                count += 1
            };
        }
    }

    println!("task1:\t{count}");
}

fn check_mas_diagonal_l(m: &Matrix, y: usize, x: usize) -> bool {
    (m[y][x] == 'M' && m[y + 1][x + 1] == 'A' && m[y + 2][x + 2] == 'S')
        || (m[y][x] == 'S' && m[y + 1][x + 1] == 'A' && m[y + 2][x + 2] == 'M')
}

fn check_mas_diagonal_r(m: &Matrix, y: usize, x: usize) -> bool {
    (m[y][x + 2] == 'M' && m[y + 1][x + 1] == 'A' && m[y + 2][x] == 'S')
        || (m[y][x + 2] == 'S' && m[y + 1][x + 1] == 'A' && m[y + 2][x] == 'M')
}

fn task2() {
    let contents = fs::read_to_string(INPUT).unwrap();

    let mut matrix: Matrix = Vec::new();

    for line in contents.lines() {
        matrix.push(line.chars().collect())
    }

    let mut count = 0;

    for y in 0..matrix.len() - MAS_KERNEL_SIZE + 1 {
        for x in 0..matrix[y].len() - MAS_KERNEL_SIZE + 1 {
            let l = check_mas_diagonal_l(&matrix, y, x);
            let r = check_mas_diagonal_r(&matrix, y, x);
            if l && r {
                count += 1
            }
        }
    }

    println!("task2:\t{count}");
}

fn main() {
    task1();
    task2();
}
