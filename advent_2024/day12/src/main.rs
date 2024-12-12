use std::{collections::VecDeque, fs};

const INPUT: &str = "input.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FenceDir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct FencePiece {
    dir: FenceDir,
    y: usize,
    x: usize,
    c: char,
}

#[allow(dead_code)]
struct FenceSide {
    pieces: Vec<FencePiece>,
}

impl FenceSide {
    fn new(start_piece: FencePiece, drain: &mut Vec<FencePiece>) -> FenceSide {
        let dir = start_piece.dir;
        let mut pieces = vec![start_piece];
        let mut cords = match dir {
            FenceDir::Left | FenceDir::Right => vec![start_piece.y],
            FenceDir::Up | FenceDir::Down => vec![start_piece.x],
        };

        loop {
            let next = match dir {
                FenceDir::Up | FenceDir::Down => drain.iter().enumerate().find(|(_, f)| {
                    f.dir == dir
                        && f.c == start_piece.c
                        && f.y == start_piece.y
                        && cords.iter().find(|&&c| c.abs_diff(f.x) == 1).is_some()
                }),
                FenceDir::Left | FenceDir::Right => drain.iter().enumerate().find(|(_, f)| {
                    f.dir == dir
                        && f.c == start_piece.c
                        && f.x == start_piece.x
                        && cords.iter().find(|&&c| c.abs_diff(f.y) == 1).is_some()
                }),
            };

            if let Some((next_i, _)) = next {
                let next_p = drain.remove(next_i);
                match dir {
                    FenceDir::Left | FenceDir::Right => cords.push(next_p.y),
                    FenceDir::Up | FenceDir::Down => cords.push(next_p.x),
                }
                pieces.push(next_p);
            } else {
                break;
            }
        }

        FenceSide { pieces }
    }
}

fn fill_field(
    sy: usize,
    sx: usize,
    map: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
) -> (usize, Vec<FencePiece>) {
    let mut areas = 0;
    let mut fence_pieces = Vec::new();

    let c = map[sy][sx];

    let mut field_search = VecDeque::new();
    field_search.push_back((sy, sx));

    while let Some((y, x)) = field_search.pop_front() {
        if visited[y][x] {
            continue;
        }
        areas += 1;
        visited[y][x] = true;

        if y == 0 || map[y - 1][x] != c {
            fence_pieces.push(FencePiece {
                dir: FenceDir::Up,
                y,
                x,
                c,
            });
        } else {
            field_search.push_back((y - 1, x));
        }

        if y == map.len() - 1 || map[y + 1][x] != c {
            fence_pieces.push(FencePiece {
                dir: FenceDir::Down,
                y,
                x,
                c,
            });
        } else {
            field_search.push_back((y + 1, x));
        }

        if x == 0 || map[y][x - 1] != c {
            fence_pieces.push(FencePiece {
                dir: FenceDir::Left,
                y,
                x,
                c,
            });
        } else {
            field_search.push_back((y, x - 1));
        }

        if x == map[0].len() - 1 || map[y][x + 1] != c {
            fence_pieces.push(FencePiece {
                dir: FenceDir::Right,
                y,
                x,
                c,
            });
        } else {
            field_search.push_back((y, x + 1));
        }
    }

    (areas, fence_pieces)
}

fn task1() {
    let contents = fs::read_to_string(INPUT).unwrap();

    let map: Vec<Vec<char>> = contents.lines().map(|l| l.chars().collect()).collect();
    let mut visited_fields = vec![vec![false; map[0].len()]; map.len()];

    let mut score = 0;

    for (y, l) in map.iter().enumerate() {
        for (x, _c) in l.iter().enumerate() {
            if !visited_fields[y][x] {
                let (areas, fence_pieces) = fill_field(y, x, &map, &mut visited_fields);
                score += areas * fence_pieces.len();
            }
        }
    }

    println!("task1:\t{score:}");
}

fn task2() {
    let contents = fs::read_to_string(INPUT).unwrap();

    let map: Vec<Vec<char>> = contents.lines().map(|l| l.chars().collect()).collect();
    let mut visited_fields = vec![vec![false; map[0].len()]; map.len()];

    let mut score = 0;

    for (y, l) in map.iter().enumerate() {
        for (x, _c) in l.iter().enumerate() {
            if !visited_fields[y][x] {
                let (areas, mut fence_pieces) = fill_field(y, x, &map, &mut visited_fields);

                let mut fence_sides = Vec::new();
                while let Some(piece) = fence_pieces.pop() {
                    fence_sides.push(FenceSide::new(piece, &mut fence_pieces));
                }

                score += areas * fence_sides.len();
            }
        }
    }

    println!("task2:\t{score:}");
}

fn main() {
    task1();
    task2();
}
