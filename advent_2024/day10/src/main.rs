use std::fs;

const INPUT: &str = "input.txt";

fn task1() {
    let contents = fs::read_to_string(INPUT).unwrap();

    let map: Vec<Vec<i8>> = contents
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
                .collect()
        })
        .collect();

    let zeros_pos = map
        .iter()
        .enumerate()
        .map(|(y, l)| {
            l.iter()
                .enumerate()
                .map(move |(x, h)| if *h == 0 { Some((y, x)) } else { None })
        })
        .flatten()
        .filter_map(|p| p)
        .collect::<Vec<(usize, usize)>>();

    let zeros_scores = zeros_pos
        .iter()
        .map(|&(y, x)| {
            let mut marked_map = vec![vec![false; map[0].len()]; map.len()];
            marked_map[y][x] = true;

            fn recur(map: &Vec<Vec<i8>>, marked_map: &mut Vec<Vec<bool>>, cy: i64, cx: i64) -> u64 {
                if map[cy as usize][cx as usize] == 9 {
                    return 1;
                }

                let cands = [(0, 1), (0, -1), (1, 0), (-1, 0)]
                    .iter()
                    .filter(|(dy, dx)| {
                        cy + dy >= 0
                            && cy + dy >= 0
                            && cx + dx >= 0
                            && cy + dy < map.len() as i64
                            && cx + dx < map[0].len() as i64
                            && marked_map[(cy + dy) as usize][(cx + dx) as usize] == false
                            && map[(cy + dy) as usize][(cx + dx) as usize]
                                - map[cy as usize][cx as usize]
                                == 1
                    })
                    .map(|(dy, dx)| ((cy + dy) as usize, (cx + dx) as usize))
                    .collect::<Vec<(usize, usize)>>();

                let mut sum = 0;

                for (y, x) in cands {
                    marked_map[y][x] = true;
                    let new_trail = recur(map, marked_map, y as i64, x as i64);
                    if new_trail > 0 {
                        sum += new_trail;
                    } else {
                        marked_map[y][x] = false;
                    }
                }

                sum
            }

            recur(&map, &mut marked_map, y as i64, x as i64)
        })
        .collect::<Vec<u64>>();

    println!("task1:\t{:}", zeros_scores.iter().sum::<u64>());
}

fn task2() {
    let contents = fs::read_to_string(INPUT).unwrap();

    let map: Vec<Vec<i8>> = contents
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
                .collect()
        })
        .collect();

    let zeros_pos = map
        .iter()
        .enumerate()
        .map(|(y, l)| {
            l.iter()
                .enumerate()
                .map(move |(x, h)| if *h == 0 { Some((y, x)) } else { None })
        })
        .flatten()
        .filter_map(|p| p)
        .collect::<Vec<(usize, usize)>>();

    let tracks: Vec<((usize, usize), Vec<(usize, usize)>)> = zeros_pos
        .iter()
        .filter_map(|&(y, x)| {
            let mut marked_map = vec![vec![false; map[0].len()]; map.len()];
            marked_map[y][x] = true;

            fn recur(
                map: &Vec<Vec<i8>>,
                marked_map: &mut Vec<Vec<bool>>,
                cy: i64,
                cx: i64,
            ) -> Vec<(usize, usize)> {
                if map[cy as usize][cx as usize] == 9 {
                    return vec![(cy as usize, cx as usize)];
                }

                let cands = [(0, 1), (0, -1), (1, 0), (-1, 0)]
                    .iter()
                    .filter(|(dy, dx)| {
                        cy + dy >= 0
                            && cy + dy >= 0
                            && cx + dx >= 0
                            && cy + dy < map.len() as i64
                            && cx + dx < map[0].len() as i64
                            && marked_map[(cy + dy) as usize][(cx + dx) as usize] == false
                            && map[(cy + dy) as usize][(cx + dx) as usize]
                                - map[cy as usize][cx as usize]
                                == 1
                    })
                    .map(|(dy, dx)| ((cy + dy) as usize, (cx + dx) as usize))
                    .collect::<Vec<(usize, usize)>>();

                let mut tracks = vec![];

                for (y, x) in cands {
                    marked_map[y][x] = true;
                    let mut new_trail = recur(map, marked_map, y as i64, x as i64);
                    if new_trail.len() > 0 {
                        tracks.append(&mut new_trail);
                    } else {
                        marked_map[y][x] = false;
                    }
                }

                tracks
            }

            let tracks = recur(&map, &mut marked_map, y as i64, x as i64);

            if tracks.len() > 0 {
                Some(((y, x), tracks))
            } else {
                None
            }
        })
        .collect();

    let scores = tracks
        .iter()
        .map(|((y, x), nines)| {
            let path_scores = nines
                .iter()
                .map(|&(ny, nx)| {
                    fn recur(map: &Vec<Vec<i8>>, cy: i64, cx: i64, ny: i64, nx: i64) -> u64 {
                        if map[cy as usize][cx as usize] == 9 && cy == ny && cx == nx {
                            return 1;
                        }

                        let cands = [(0, 1), (0, -1), (1, 0), (-1, 0)]
                            .iter()
                            .filter(|(dy, dx)| {
                                cy + dy >= 0
                                    && cy + dy >= 0
                                    && cx + dx >= 0
                                    && cy + dy < map.len() as i64
                                    && cx + dx < map[0].len() as i64
                                    && map[(cy + dy) as usize][(cx + dx) as usize]
                                        - map[cy as usize][cx as usize]
                                        == 1
                            })
                            .map(|(dy, dx)| ((cy + dy) as usize, (cx + dx) as usize))
                            .collect::<Vec<(usize, usize)>>();

                        let mut sum = 0;

                        for (y, x) in cands {
                            let new_trail = recur(map, y as i64, x as i64, ny, nx);
                            if new_trail > 0 {
                                sum += new_trail;
                            }
                        }

                        sum
                    }

                    recur(&map, *y as i64, *x as i64, ny as i64, nx as i64)
                })
                .collect::<Vec<u64>>();

            path_scores.iter().sum()
        })
        .collect::<Vec<u64>>();

    println!("task2:\t{:}", scores.iter().sum::<u64>());
}

fn main() {
    task1();
    task2();
}
