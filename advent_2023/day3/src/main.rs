use std::fs;

const INPUT: &str = "input.txt";

#[derive(Debug)]
struct InstructionMat {
    w: usize,
    h: usize,
    instruction: Vec<Vec<char>>,
}

impl InstructionMat {
    fn new(file_name: &str) -> InstructionMat {
        let mut new_instr = InstructionMat {
            w: 0,
            h: 0,
            instruction: Vec::new(),
        };
        let contents = fs::read_to_string(file_name).unwrap();
        let line_w = contents.lines().next().unwrap().len();

        new_instr.instruction.push(vec!['.'; line_w + 2]);
        for line in contents.lines() {
            let mut paginated = ".".to_string();
            paginated.push_str(line);
            paginated.push('.');
            new_instr.instruction.push(paginated.chars().collect());
        }
        new_instr.instruction.push(vec!['.'; line_w + 2]);

        new_instr.w = new_instr.instruction[0].len();
        new_instr.h = new_instr.instruction.len();

        new_instr
    }

    fn check_neigh_for_part(&self, x: usize, y: usize) -> bool {
        fn is_part(c: char) -> bool {
            c != '.' && !c.is_numeric()
        }

        let combs = [
            (0, -1),
            (0, 1),
            (-1, 0),
            (1, 0),
            (-1, -1),
            (-1, 1),
            (1, -1),
            (1, 1),
        ];

        for comb in combs {
            if is_part(self.instruction[(y as i32 + comb.0) as usize][(x as i32 + comb.1) as usize])
            {
                return true;
            }
        }

        false
    }

    fn get_part_number(&self, x: usize, y: usize) -> Option<u32> {
        if !self.instruction[y][x].is_numeric() {
            return None;
        }
        let mut start_x = x;
        while self.instruction[y][start_x].is_numeric() {
            start_x -= 1;
        }
        start_x += 1;
        let mut number_str = String::from("");
        while self.instruction[y][start_x].is_numeric() {
            number_str.push(self.instruction[y][start_x]);
            start_x += 1;
        }
        let number = number_str.parse::<u32>().unwrap();

        Some(number)
    }

    fn check_neigh_for_numbers(&self, x: usize, y: usize) -> Option<Vec<u32>> {
        let mut numbers = Vec::new();

        for x_p in [1, -1] {
            if let Some(n) = self.get_part_number((x as i32 + x_p) as usize, y) {
                numbers.push(n);
            }
        }

        for y_p in [1, -1] {
            if self.instruction[(y as i32 + y_p) as usize][x] == '.' {
                if let Some(n) = self.get_part_number(x - 1, (y as i32 + y_p) as usize) {
                    numbers.push(n);
                }
                if let Some(n) = self.get_part_number(x + 1, (y as i32 + y_p) as usize) {
                    numbers.push(n);
                }
            } else {
                if let Some(n) = self.get_part_number(x, (y as i32 + y_p) as usize) {
                    numbers.push(n);
                }
            }
        }

        return if numbers.len() > 0 {
            Some(numbers)
        } else {
            None
        };
    }
}

fn task2() {
    let instr = InstructionMat::new(INPUT);
    let mut gear_sum: u32 = 0;
    for y in 1..instr.h - 1 {
        for x in 1..instr.w - 1 {
            if instr.instruction[y][x] != '*' {
                continue;
            }
            if let Some(nums) = instr.check_neigh_for_numbers(x, y) {
                if nums.len() == 2 {
                    gear_sum += nums[0] * nums[1];
                }
            }
        }
    }
    println!("task2: {:?}", gear_sum);
}

fn task1() {
    let instr = InstructionMat::new(INPUT);
    let mut part_sum: u32 = 0;
    let mut number_str = String::from("");
    let mut is_part = false;
    for y in 1..instr.h - 1 {
        for x in 1..instr.w - 1 {
            let c = instr.instruction[y][x];
            if !c.is_numeric() {
                if number_str.len() != 0 && is_part {
                    part_sum += number_str.parse::<u32>().unwrap();
                }
                number_str = String::from("");
                is_part = false;
            } else {
                number_str.push(c);
                is_part |= instr.check_neigh_for_part(x, y);
            }
        }
    }
    println!("task1: {:?}", part_sum);
}

fn main() {
    task1();
    task2();
}
