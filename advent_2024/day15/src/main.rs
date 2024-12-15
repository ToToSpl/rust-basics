use std::fs;

const INPUT: &str = "input.txt";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Wall,
    Box,
    BoxWideL,
    BoxWideR,
    Robot,
    Empty,
}

impl Tile {
    fn from_char(t: char) -> Tile {
        use Tile::*;
        match t {
            '.' => Empty,
            '@' => Robot,
            'O' => Box,
            '#' => Wall,
            _ => panic!("wrong char passed to tile: {t}"),
        }
    }

    fn widen(&self) -> Vec<Tile> {
        use Tile::*;
        match self {
            Empty => vec![Empty, Empty],
            Robot => vec![Robot, Empty],
            Box => vec![BoxWideL, BoxWideR],
            Wall => vec![Wall, Wall],
            BoxWideL | BoxWideR => panic!("cannot widen wide box"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn from_char(d: char) -> Dir {
        use Dir::*;
        match d {
            '^' => Up,
            'v' => Down,
            '>' => Right,
            '<' => Left,
            _ => panic!("wrong char passed to dir: {d}"),
        }
    }

    fn to_n_pos(&self, pos: &(usize, usize), map: &Vec<Vec<Tile>>) -> Option<(usize, usize)> {
        use Dir::*;
        match self {
            Up => {
                if pos.0 == 0 {
                    return None;
                }
                Some((pos.0 - 1, pos.1))
            }
            Down => {
                if pos.0 == map.len() - 1 {
                    return None;
                }
                Some((pos.0 + 1, pos.1))
            }
            Left => {
                if pos.1 == 0 {
                    return None;
                }
                Some((pos.0, pos.1 - 1))
            }
            Right => {
                if pos.1 == map[0].len() - 1 {
                    return None;
                }
                Some((pos.0, pos.1 + 1))
            }
        }
    }
}

fn can_move_tile(map: &Vec<Vec<Tile>>, pos: &(usize, usize), dir: &Dir) -> bool {
    let tile = map[pos.0][pos.1];
    match tile {
        Tile::Wall => false,
        Tile::Empty => true,
        Tile::Box | Tile::Robot => {
            if let Some(n_pos) = dir.to_n_pos(&pos, map) {
                can_move_tile(map, &n_pos, dir)
            } else {
                false
            }
        }
        Tile::BoxWideL | Tile::BoxWideR => {
            let (cur_l_pos, cur_r_pos) = if tile == Tile::BoxWideL {
                (*pos, (pos.0, pos.1 + 1))
            } else {
                ((pos.0, pos.1 - 1), *pos)
            };

            let Some(l_pos) = dir.to_n_pos(&cur_l_pos, map) else {
                return false;
            };
            let Some(r_pos) = dir.to_n_pos(&cur_r_pos, map) else {
                return false;
            };

            match dir {
                Dir::Up | Dir::Down => {
                    can_move_tile(map, &l_pos, dir) && can_move_tile(map, &r_pos, dir)
                }
                Dir::Left => can_move_tile(map, &l_pos, dir),
                Dir::Right => can_move_tile(map, &r_pos, dir),
            }
        }
    }
}

fn move_tile(map: &mut Vec<Vec<Tile>>, pos: &(usize, usize), dir: &Dir) {
    let tile = map[pos.0][pos.1];
    match tile {
        Tile::Wall => (),
        Tile::Empty => (),
        Tile::Box | Tile::Robot => {
            let n_pos = dir.to_n_pos(pos, map).unwrap();

            move_tile(map, &n_pos, dir);

            map[n_pos.0][n_pos.1] = map[pos.0][pos.1];
            map[pos.0][pos.1] = Tile::Empty;
        }
        Tile::BoxWideL | Tile::BoxWideR => {
            let (cur_l_pos, cur_r_pos) = if tile == Tile::BoxWideL {
                (*pos, (pos.0, pos.1 + 1))
            } else {
                ((pos.0, pos.1 - 1), *pos)
            };

            let l_pos = dir.to_n_pos(&cur_l_pos, map).unwrap();
            let r_pos = dir.to_n_pos(&cur_r_pos, map).unwrap();

            match dir {
                Dir::Left => {
                    move_tile(map, &l_pos, dir);

                    map[l_pos.0][l_pos.1] = Tile::BoxWideL;
                    map[r_pos.0][r_pos.1] = Tile::BoxWideR;
                    map[cur_r_pos.0][cur_r_pos.1] = Tile::Empty;
                }
                Dir::Right => {
                    move_tile(map, &r_pos, dir);

                    map[l_pos.0][l_pos.1] = Tile::BoxWideL;
                    map[r_pos.0][r_pos.1] = Tile::BoxWideR;
                    map[cur_l_pos.0][cur_l_pos.1] = Tile::Empty;
                }
                Dir::Up | Dir::Down => {
                    move_tile(map, &l_pos, dir);
                    move_tile(map, &r_pos, dir);

                    map[l_pos.0][l_pos.1] = Tile::BoxWideL;
                    map[r_pos.0][r_pos.1] = Tile::BoxWideR;
                    map[cur_l_pos.0][cur_l_pos.1] = Tile::Empty;
                    map[cur_r_pos.0][cur_r_pos.1] = Tile::Empty;
                }
            }
        }
    }
}

fn task1() {
    let contents = fs::read_to_string(INPUT).unwrap();

    let mut map: Vec<Vec<Tile>> = contents
        .lines()
        .take_while(|l| l.len() > 0)
        .map(|l| l.chars().map(Tile::from_char).collect())
        .collect();

    let mut robot_pos = map
        .iter()
        .enumerate()
        .find_map(|(y, l)| {
            l.iter().enumerate().find_map(|(x, t)| {
                if *t == Tile::Robot {
                    Some((y, x))
                } else {
                    None
                }
            })
        })
        .unwrap();

    let dirs: Vec<Dir> = contents
        .lines()
        .skip_while(|l| l.len() == 0 || l.chars().nth(0).unwrap() == '#')
        .map(|l| l.chars().map(Dir::from_char).collect::<Vec<Dir>>())
        .flatten()
        .collect();

    for dir in &dirs {
        if can_move_tile(&map, &robot_pos, dir) {
            move_tile(&mut map, &robot_pos, dir);
            robot_pos = dir.to_n_pos(&robot_pos, &map).unwrap();
        }
    }

    let score = map
        .iter()
        .enumerate()
        .map(|(y, l)| {
            l.iter().enumerate().filter_map(move |(x, t)| {
                if *t != Tile::Box {
                    None
                } else {
                    Some(100 * y + x)
                }
            })
        })
        .flatten()
        .sum::<usize>();

    println!("task1:\t{score}");
}

fn task2() {
    let contents = fs::read_to_string(INPUT).unwrap();

    let mut map: Vec<Vec<Tile>> = contents
        .lines()
        .take_while(|l| l.len() > 0)
        .map(|l| {
            l.chars()
                .map(|c| Tile::from_char(c).widen())
                .flatten()
                .collect()
        })
        .collect();

    let mut robot_pos = map
        .iter()
        .enumerate()
        .find_map(|(y, l)| {
            l.iter().enumerate().find_map(|(x, t)| {
                if *t == Tile::Robot {
                    Some((y, x))
                } else {
                    None
                }
            })
        })
        .unwrap();

    let dirs: Vec<Dir> = contents
        .lines()
        .skip_while(|l| l.len() == 0 || l.chars().nth(0).unwrap() == '#')
        .map(|l| l.chars().map(Dir::from_char).collect::<Vec<Dir>>())
        .flatten()
        .collect();

    for dir in &dirs {
        if can_move_tile(&map, &robot_pos, dir) {
            move_tile(&mut map, &robot_pos, dir);
            robot_pos = dir.to_n_pos(&robot_pos, &map).unwrap();
        }
    }

    let score = map
        .iter()
        .enumerate()
        .map(|(y, l)| {
            l.iter().enumerate().filter_map(move |(x, t)| {
                if *t != Tile::BoxWideL {
                    None
                } else {
                    Some(100 * y + x)
                }
            })
        })
        .flatten()
        .sum::<usize>();

    println!("task2:\t{score}");
}

fn main() {
    task1();
    task2();
}
