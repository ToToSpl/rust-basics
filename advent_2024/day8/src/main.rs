use std::{char, collections::HashMap, fs, usize};

const INPUT: &str = "input.txt";

fn task1() {
    let contents = fs::read_to_string(INPUT).unwrap();

    let map: Vec<Vec<Option<char>>> = contents
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| if c == '.' { None } else { Some(c) })
                .collect()
        })
        .collect();

    let station_locations = {
        let mut station_locations: HashMap<char, Vec<(i64, i64)>> = HashMap::new();
        map.iter()
            .enumerate()
            .map(|(y, l)| {
                l.iter()
                    .enumerate()
                    .filter_map(|(x, c)| {
                        if let Some(c) = c {
                            Some((*c, (y as i64, x as i64)))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<(char, (i64, i64))>>()
            })
            .flatten()
            .for_each(|(s, c)| {
                station_locations
                    .entry(s)
                    .and_modify(|v| v.push(c))
                    .or_insert(vec![c]);
            });

        station_locations
    };

    let map_dimensions = (map.len(), map[0].len());
    let mut antinodes_map = vec![vec![false; map_dimensions.1]; map_dimensions.0];

    station_locations.iter().for_each(|(_station, locations)| {
        for (i, location) in locations.iter().enumerate() {
            for (j, other_location) in locations.iter().enumerate() {
                if i == j {
                    continue;
                }

                let dif = (location.0 - other_location.0, location.1 - other_location.1);
                let antinode_pos = (location.0 + dif.0, location.1 + dif.1);

                if antinode_pos.0 < 0
                    || antinode_pos.0 >= map_dimensions.0 as i64
                    || antinode_pos.1 < 0
                    || antinode_pos.1 >= map_dimensions.1 as i64
                {
                    continue;
                }

                antinodes_map[antinode_pos.0 as usize][antinode_pos.1 as usize] = true;
            }
        }
    });

    let count = antinodes_map
        .iter()
        .map(|l| l.iter().map(|c| if *c { 1 } else { 0 }))
        .flatten()
        .sum::<u64>();

    println!("task1:\t{count}");
}

fn task2() {
    let contents = fs::read_to_string(INPUT).unwrap();

    let map: Vec<Vec<Option<char>>> = contents
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| if c == '.' { None } else { Some(c) })
                .collect()
        })
        .collect();

    let station_locations = {
        let mut station_locations: HashMap<char, Vec<(i64, i64)>> = HashMap::new();
        map.iter()
            .enumerate()
            .map(|(y, l)| {
                l.iter()
                    .enumerate()
                    .filter_map(|(x, c)| {
                        if let Some(c) = c {
                            Some((*c, (y as i64, x as i64)))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<(char, (i64, i64))>>()
            })
            .flatten()
            .for_each(|(s, c)| {
                station_locations
                    .entry(s)
                    .and_modify(|v| v.push(c))
                    .or_insert(vec![c]);
            });

        station_locations
    };

    let map_dimensions = (map.len(), map[0].len());
    let mut antinodes_map = vec![vec![false; map_dimensions.1]; map_dimensions.0];

    station_locations.iter().for_each(|(_station, locations)| {
        for (i, location) in locations.iter().enumerate() {
            for other_location in locations.iter().skip(i + 1) {
                let dif = (other_location.0 - location.0, other_location.1 - location.1);
                let mut freq_pos = (location.0, location.1);
                while freq_pos.0 - dif.0 >= 0
                    && freq_pos.0 - dif.0 < map_dimensions.0 as i64
                    && freq_pos.1 - dif.1 >= 0
                    && freq_pos.1 - dif.1 < map_dimensions.1 as i64
                {
                    freq_pos.0 -= dif.0;
                    freq_pos.1 -= dif.1;
                }

                while freq_pos.0 >= 0
                    && freq_pos.0 < map_dimensions.0 as i64
                    && freq_pos.1 >= 0
                    && freq_pos.1 < map_dimensions.1 as i64
                {
                    antinodes_map[freq_pos.0 as usize][freq_pos.1 as usize] = true;

                    freq_pos.0 += dif.0;
                    freq_pos.1 += dif.1;
                }
            }
        }
    });

    // for y in 0..map_dimensions.0 {
    //     for x in 0..map_dimensions.1 {
    //         if let Some(s) = map[y][x] {
    //             print!("{s}")
    //         } else if antinodes_map[y][x] {
    //             print!("#")
    //         } else {
    //             print!(".")
    //         }
    //     }
    //     println!("");
    // }

    let count = antinodes_map
        .iter()
        .map(|l| l.iter().map(|c| if *c { 1 } else { 0 }))
        .flatten()
        .sum::<u64>();

    println!("task2:\t{count}");
}

fn main() {
    task1();
    task2();
}
