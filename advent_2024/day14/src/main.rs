use std::fs;

use image::{GrayImage, Luma};

const INPUT: &str = "input.txt";
// const GRID_Y: i64 = 7;
// const GRID_X: i64 = 11;
const GRID_Y: i64 = 103;
const GRID_X: i64 = 101;

#[derive(Debug, Clone, Copy)]
struct Vector<T> {
    x: T,
    y: T,
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    p: Vector<i64>,
    v: Vector<i64>,
}

fn task1() {
    let contents = fs::read_to_string(INPUT).unwrap();

    let mut robots: Vec<Robot> = contents
        .lines()
        .map(|l| {
            let p_v: Vec<i64> = l
                .split(' ')
                .map(|p| p[2..].split(',').map(|n| n.parse().unwrap()))
                .flatten()
                .collect();

            Robot {
                p: Vector {
                    x: p_v[0],
                    y: p_v[1],
                },
                v: Vector {
                    x: p_v[2],
                    y: p_v[3],
                },
            }
        })
        .collect();

    for _ in 0..100 {
        for robot in &mut robots {
            robot.p.x += robot.v.x;
            if robot.p.x >= GRID_X {
                robot.p.x -= GRID_X;
            } else if robot.p.x < 0 {
                robot.p.x += GRID_X;
            }

            robot.p.y += robot.v.y;
            if robot.p.y >= GRID_Y {
                robot.p.y -= GRID_Y;
            } else if robot.p.y < 0 {
                robot.p.y += GRID_Y;
            }
        }
    }

    let mut quadrants = vec![0; 4];

    for robot in &robots {
        if robot.p.x < GRID_X / 2 && robot.p.y < GRID_Y / 2 {
            quadrants[0] += 1;
        } else if robot.p.x > GRID_X / 2 && robot.p.y < GRID_Y / 2 {
            quadrants[1] += 1;
        } else if robot.p.x < GRID_X / 2 && robot.p.y > GRID_Y / 2 {
            quadrants[2] += 1;
        } else if robot.p.x > GRID_X / 2 && robot.p.y > GRID_Y / 2 {
            quadrants[3] += 1;
        }
    }

    let score = quadrants.iter().product::<usize>();

    println!("task1:\t{score}");
}

fn task2() {
    let contents = fs::read_to_string(INPUT).unwrap();

    let mut robots: Vec<Robot> = contents
        .lines()
        .map(|l| {
            let p_v: Vec<i64> = l
                .split(' ')
                .map(|p| p[2..].split(',').map(|n| n.parse().unwrap()))
                .flatten()
                .collect();

            Robot {
                p: Vector {
                    x: p_v[0],
                    y: p_v[1],
                },
                v: Vector {
                    x: p_v[2],
                    y: p_v[3],
                },
            }
        })
        .collect();

    for i in 0..10_000 {
        for robot in &mut robots {
            robot.p.x += robot.v.x;
            if robot.p.x >= GRID_X {
                robot.p.x -= GRID_X;
            } else if robot.p.x < 0 {
                robot.p.x += GRID_X;
            }

            robot.p.y += robot.v.y;
            if robot.p.y >= GRID_Y {
                robot.p.y -= GRID_Y;
            } else if robot.p.y < 0 {
                robot.p.y += GRID_Y;
            }
        }

        let mut img = GrayImage::new(GRID_X as u32 * 8, GRID_Y as u32 * 8 as u32);
        for robot in &robots {
            for i in 0..8 {
                for j in 0..8 {
                    img.put_pixel(
                        8 * robot.p.x as u32 + i as u32,
                        8 * robot.p.y as u32 + j as u32,
                        Luma([255]),
                    );
                }
            }
        }
        img.save(format!("./renders/{i}.png")).unwrap();
    }
}

fn main() {
    task1();
    task2();
}
