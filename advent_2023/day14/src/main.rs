use std::collections::HashMap;
use std::fs;

const INPUT: &str = "input.txt";
const CYCLES: usize = 1_000_000_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Field {
    Round,
    Stationary,
    Empty,
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Platform {
    cols: Vec<Vec<Field>>,
    width: usize,
    height: usize,
}

impl Platform {
    fn new(input: &str) -> Platform {
        let contents = fs::read_to_string(input).unwrap();
        let rows: Vec<Vec<_>> = contents
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        'O' => Field::Round,
                        '#' => Field::Stationary,
                        '.' => Field::Empty,
                        _e => panic!("Unknown field {:?} found", _e),
                    })
                    .collect()
            })
            .collect();

        let height = rows.len();
        let width = rows[0].len();

        let cols: Vec<Vec<_>> = (0..width)
            .map(|i| (0..height).map(|j| rows[j][i]).collect())
            .collect();

        Platform {
            cols,
            width,
            height,
        }
    }

    fn tilt_north(&mut self) {
        for x in 0..self.width {
            let mut candidate = None;
            for y in 0..self.height {
                if self.cols[x][y] == Field::Empty {
                    if candidate.is_none() {
                        candidate = Some(y);
                    }
                    continue;
                } else if self.cols[x][y] == Field::Stationary {
                    candidate = None;
                    continue;
                }

                let new_y = if let Some(new_y) = candidate {
                    new_y
                } else {
                    let mut new_y = y;
                    while new_y > 0 && self.cols[x][new_y - 1] == Field::Empty {
                        new_y -= 1;
                    }
                    new_y
                };
                candidate = Some(new_y + 1);

                self.cols[x].swap(y, new_y);
            }
        }
    }

    fn tilt_south(&mut self) {
        for x in 0..self.width {
            let mut candidate = None;
            for y in (0..self.height).rev() {
                if self.cols[x][y] == Field::Empty {
                    if candidate.is_none() {
                        candidate = Some(y);
                    }
                    continue;
                } else if self.cols[x][y] == Field::Stationary {
                    candidate = None;
                    continue;
                }

                let new_y = if let Some(new_y) = candidate {
                    new_y
                } else {
                    let mut new_y = y;
                    while new_y < self.height - 1 && self.cols[x][new_y + 1] == Field::Empty {
                        new_y += 1;
                    }
                    new_y
                };
                candidate = Some(new_y - 1);

                self.cols[x].swap(y, new_y);
            }
        }
    }

    fn tilt_west(&mut self) {
        for y in 0..self.height {
            let mut candidate = None;
            for x in 0..self.width {
                if self.cols[x][y] == Field::Empty {
                    if candidate.is_none() {
                        candidate = Some(x);
                    }
                    continue;
                } else if self.cols[x][y] == Field::Stationary {
                    candidate = None;
                    continue;
                }

                let new_x = if let Some(new_x) = candidate {
                    new_x
                } else {
                    let mut new_x = x;

                    while new_x > 0 && self.cols[new_x - 1][y] == Field::Empty {
                        new_x -= 1;
                    }
                    new_x
                };
                candidate = Some(new_x + 1);

                let temp = self.cols[x][y];
                self.cols[x][y] = self.cols[new_x][y];
                self.cols[new_x][y] = temp;
            }
        }
    }

    fn tilt_east(&mut self) {
        for y in 0..self.height {
            let mut candidate = None;
            for x in (0..self.width).rev() {
                if self.cols[x][y] == Field::Empty {
                    if candidate.is_none() {
                        candidate = Some(x);
                    }
                    continue;
                } else if self.cols[x][y] == Field::Stationary {
                    candidate = None;
                    continue;
                }

                let new_x = if let Some(new_x) = candidate {
                    new_x
                } else {
                    let mut new_x = x;

                    while new_x < self.width - 1 && self.cols[new_x + 1][y] == Field::Empty {
                        new_x += 1;
                    }
                    new_x
                };
                candidate = Some(new_x - 1);

                let temp = self.cols[x][y];
                self.cols[x][y] = self.cols[new_x][y];
                self.cols[new_x][y] = temp;
            }
        }
    }

    fn cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }

    fn count_weight_north(&self) -> usize {
        let mut sum = 0;
        for x in 0..self.width {
            for y in 0..self.height {
                if self.cols[x][y] == Field::Round {
                    sum += self.height - y;
                }
            }
        }
        sum
    }
}

fn task1() {
    let mut platform = Platform::new(INPUT);
    platform.tilt_north();
    println!("task1 {:?}", platform.count_weight_north());
}

fn task2() {
    let mut platform = Platform::new(INPUT);
    let mut dict: HashMap<Platform, usize> = HashMap::new();

    let mut left = 0;
    for i in 0..CYCLES {
        platform.cycle();
        match dict.get(&platform) {
            None => {
                dict.insert(platform.clone(), i);
            }
            Some(p_i) => {
                let cycle_len = i - p_i;
                left = (CYCLES - 1 - i) % cycle_len;
                break;
            }
        }
    }
    for _ in 0..left {
        platform.cycle();
    }

    println!("task2 {:?}", platform.count_weight_north());
}

fn main() {
    task1();
    task2();
}
