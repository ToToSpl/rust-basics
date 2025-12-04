use std::fs;

const INPUT: &str = "input.txt";
// const INPUT: &str = "input.test.txt";

enum GridState {
    Nothing,
    Paper,
}

static CHECK_MAP: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn load_map() -> Vec<Vec<GridState>> {
    fs::read_to_string(INPUT)
        .unwrap()
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => GridState::Nothing,
                    '@' => GridState::Paper,
                    _ => panic!("unknown grid type: {c}"),
                })
                .collect()
        })
        .collect()
}

fn task1() {
    let map = load_map();

    let width = map[0].len();
    let height = map.len();

    let mut accessible_paper = 0;

    for (y, line) in map.iter().enumerate() {
        for (x, state) in line.iter().enumerate() {
            if matches!(state, GridState::Nothing) {
                continue;
            }

            let mut neigh_paper = 0;

            for (py, px) in CHECK_MAP {
                let pot_y = y as i32 + py;
                let pot_x = x as i32 + px;

                if pot_y >= 0 && pot_y < height as i32 && pot_x >= 0 && pot_x < width as i32 {
                    if matches!(map[pot_y as usize][pot_x as usize], GridState::Paper) {
                        neigh_paper += 1;
                    }
                }
            }

            if neigh_paper < 4 {
                accessible_paper += 1;
            }
        }
    }

    println!("task1:\t{accessible_paper}");
}

fn task2() {
    let mut map = load_map();

    let width = map[0].len();
    let height = map.len();

    let mut accessible_paper = 0;

    loop {
        let mut paper_to_remove = Vec::new();

        for (y, line) in map.iter().enumerate() {
            for (x, state) in line.iter().enumerate() {
                if matches!(state, GridState::Nothing) {
                    continue;
                }

                let mut neigh_paper = 0;

                for (py, px) in CHECK_MAP {
                    let pot_y = y as i32 + py;
                    let pot_x = x as i32 + px;

                    if pot_y >= 0 && pot_y < height as i32 && pot_x >= 0 && pot_x < width as i32 {
                        if matches!(map[pot_y as usize][pot_x as usize], GridState::Paper) {
                            neigh_paper += 1;
                        }
                    }
                }

                if neigh_paper < 4 {
                    paper_to_remove.push((y, x));
                }
            }
        }

        accessible_paper += paper_to_remove.len();

        if paper_to_remove.len() == 0 {
            break;
        }

        for (y, x) in paper_to_remove {
            map[y][x] = GridState::Nothing;
        }
    }

    println!("task2:\t{accessible_paper}");
}

fn main() {
    task1();
    task2();
}
