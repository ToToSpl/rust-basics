use std::fs;

const INPUT: &str = "input.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    Filled { size: usize, id: usize },
    Empty { size: usize },
}

fn task2() {
    let contents = fs::read_to_string(INPUT).unwrap();

    let mut disk_map = contents
        .chars()
        .filter(|c| *c != '\n')
        .enumerate()
        .map(|(i, s)| {
            let repeats: usize = s.to_digit(10).unwrap() as usize;
            if i % 2 == 0 {
                Space::Filled {
                    id: i / 2,
                    size: repeats,
                }
            } else {
                Space::Empty { size: repeats }
            }
        })
        .collect::<Vec<Space>>();

    'outer: for i in (0..disk_map.len()).rev() {
        match disk_map[i] {
            Space::Filled { size, id } => {
                for j in 0..i {
                    match disk_map[j] {
                        Space::Filled { size: _, id: _ } => continue,
                        Space::Empty { size: empty_size } => {
                            if empty_size < size {
                                continue;
                            }
                            if empty_size == size {
                                disk_map.swap(i, j);
                            } else {
                                disk_map[j] = Space::Filled { size, id };
                                disk_map[i] = Space::Empty { size };

                                disk_map.insert(
                                    j + 1,
                                    Space::Empty {
                                        size: empty_size - size,
                                    },
                                );
                            }
                            continue 'outer;
                        }
                    }
                }
            }
            Space::Empty { size: _ } => continue,
        }
    }

    let checksum = disk_map
        .iter()
        .map(|d| match d {
            Space::Filled { size, id } => vec![Some(*id); *size],
            Space::Empty { size } => vec![None; *size],
        })
        .flatten()
        .enumerate()
        .filter_map(|(i, id)| match id {
            Some(id) => Some((i, id)),
            None => None,
        })
        .map(|(i, id)| i * id)
        .sum::<usize>();

    println!("task2:\t{checksum}");
}

fn task1() {
    let contents = fs::read_to_string(INPUT).unwrap();

    let mut disk_map = contents
        .chars()
        .filter(|c| *c != '\n')
        .enumerate()
        .map(|(i, s)| {
            let repeats: usize = s.to_digit(10).unwrap() as usize;
            let content = if i % 2 == 0 { Some(i / 2) } else { None };

            vec![content; repeats]
        })
        .flatten()
        .collect::<Vec<Option<usize>>>();

    let mut left_index = 0;
    let mut right_index = disk_map.len() - 1;
    while left_index < right_index {
        let left = disk_map[left_index];
        let right = disk_map[right_index];

        match (left, right) {
            (Some(_), None) => {
                left_index += 1;
                right_index -= 1;
            }
            (Some(_), Some(_)) => {
                left_index += 1;
            }
            (None, Some(_)) => {
                disk_map.swap(left_index, right_index);
                left_index += 1;
                right_index -= 1;
            }
            (None, None) => {
                right_index -= 1;
            }
        }
    }

    let checksum = disk_map
        .into_iter()
        .filter_map(|d| d)
        .enumerate()
        .map(|(i, d)| i * d)
        .sum::<usize>();

    println!("task1:\t{checksum}");
}

fn main() {
    task1();
    task2();
}
