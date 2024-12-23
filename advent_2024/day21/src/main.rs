use std::{collections::HashMap, fs};

const INPUT: &str = "input.txt";

type LookupDict = HashMap<char, HashMap<char, Vec<String>>>;

fn generate_lookup_dict(
    keypad_forbidden: &(i32, i32),
    keypad_lookup: &HashMap<char, (i32, i32)>,
) -> LookupDict {
    let keys: Vec<char> = keypad_lookup.keys().map(|c| *c).collect();
    let mut lookup = HashMap::new();

    for &key1 in &keys {
        for &key2 in &keys {
            if key1 == key2 {
                lookup
                    .entry(key1)
                    .or_insert(HashMap::new())
                    .insert(key2, vec!["A".to_string()]);
                continue;
            }
            let c1 = keypad_lookup[&key1];
            let c2 = keypad_lookup[&key2];

            let y = c2.0 - c1.0;
            let x = c2.1 - c1.1;

            let y_char = if y > 0 { 'v' } else { '^' };
            let x_char = if x > 0 { '>' } else { '<' };

            let p1 = y_char.to_string().repeat(y.abs() as usize)
                + x_char.to_string().repeat(x.abs() as usize).as_str()
                + 'A'.to_string().as_str();

            let p2 = x_char.to_string().repeat(x.abs() as usize)
                + y_char.to_string().repeat(y.abs() as usize).as_str()
                + 'A'.to_string().as_str();

            let mut out = Vec::new();

            if !(c1.0 + y == keypad_forbidden.0 && c1.1 == keypad_forbidden.1) && p1 != p2 {
                out.push(p1);
            }

            if !(c1.0 == keypad_forbidden.0 && c1.1 + x == keypad_forbidden.1) {
                out.push(p2);
            }

            lookup
                .entry(key1)
                .or_insert(HashMap::new())
                .insert(key2, out);
        }
    }
    lookup
}

fn generate_keypad_lookup() -> LookupDict {
    let keypad_forbidden = (3, 0);
    let keypad_lookup: HashMap<char, (i32, i32)> = HashMap::from([
        ('A', (3, 2)),
        ('0', (3, 1)),
        ('1', (2, 0)),
        ('2', (2, 1)),
        ('3', (2, 2)),
        ('4', (1, 0)),
        ('5', (1, 1)),
        ('6', (1, 2)),
        ('7', (0, 0)),
        ('8', (0, 1)),
        ('9', (0, 2)),
    ]);

    generate_lookup_dict(&keypad_forbidden, &keypad_lookup)
}

fn generate_ctrl_lookup() -> LookupDict {
    let keypad_forbidden = (0, 0);
    let keypad_lookup: HashMap<char, (i32, i32)> = HashMap::from([
        ('^', (0, 1)),
        ('A', (0, 2)),
        ('<', (1, 0)),
        ('v', (1, 1)),
        ('>', (1, 2)),
    ]);

    generate_lookup_dict(&keypad_forbidden, &keypad_lookup)
}

fn get_path_combinations(path: &str, lookup: &LookupDict) -> Vec<String> {
    let mut combinations: Vec<String> = Vec::new();

    let mut from = 'A';
    for pos in path.chars() {
        let paths = lookup[&from][&pos].clone();

        let start_len = combinations.len();
        if start_len == 0 {
            combinations = paths;
        } else {
            if paths.len() == 2 {
                for i in 0..start_len {
                    let mut new_comb = combinations[i].clone();
                    new_comb.push_str(paths[1].as_str());
                    combinations.push(new_comb);
                }
            }
            for i in 0..start_len {
                combinations[i].push_str(paths[0].as_str());
            }
        }

        from = pos;
    }
    combinations
}

fn rec_count_moves(
    path: String,
    layer: usize,
    ctrl_lookup: &LookupDict,
    count_lookup: &mut Vec<HashMap<String, usize>>,
) -> usize {
    if let Some(count) = count_lookup[layer].get(&path) {
        return *count;
    }

    let combinations = get_path_combinations(&path, ctrl_lookup);

    if layer == 0 {
        let smallest = combinations.into_iter().map(|c| c.len()).min().unwrap();
        *count_lookup[layer].entry(path).or_default() = smallest;
        smallest
    } else {
        let smallest = combinations
            .into_iter()
            .map(|c| {
                let mut path_parts: Vec<String> = c
                    .split("A")
                    .map(|s| {
                        let mut s = s.to_string();
                        s.push_str("A");
                        s
                    })
                    .collect();
                path_parts.pop();

                path_parts
                    .into_iter()
                    .map(|p| rec_count_moves(p, layer - 1, ctrl_lookup, count_lookup))
                    .sum::<usize>()
            })
            .min()
            .unwrap();

        *count_lookup[layer].entry(path).or_default() = smallest;
        smallest
    }
}

fn task1() {
    let contents = fs::read_to_string(INPUT).unwrap();
    let codes: Vec<&str> = contents.lines().collect();

    let keypad_lookup = generate_keypad_lookup();
    let ctrl_lookup = generate_ctrl_lookup();
    let mut count_lookup: Vec<HashMap<String, usize>> = vec![HashMap::new(); 25];

    let mut score = 0;
    for code in codes {
        let keypad_paths = get_path_combinations(code, &keypad_lookup);

        let best_path_len: usize = keypad_paths
            .iter()
            .map(|keypad_path| {
                let mut path_parts: Vec<String> = keypad_path
                    .split("A")
                    .map(|s| {
                        let mut s = s.to_string();
                        s.push_str("A");
                        s
                    })
                    .collect();
                path_parts.pop();

                path_parts
                    .into_iter()
                    .map(|p| rec_count_moves(p, 1, &ctrl_lookup, &mut count_lookup))
                    .sum()
            })
            .min()
            .unwrap();

        let numeric_part: usize = code[..code.len() - 1].parse().unwrap();

        score += best_path_len * numeric_part;
    }

    println!("task1:\t{score}");
}

fn task2() {
    let contents = fs::read_to_string(INPUT).unwrap();
    let codes: Vec<&str> = contents.lines().collect();

    let keypad_lookup = generate_keypad_lookup();
    let ctrl_lookup = generate_ctrl_lookup();

    let mut count_lookup: Vec<HashMap<String, usize>> = vec![HashMap::new(); 25];

    let mut score = 0;
    for code in codes {
        let keypad_paths = get_path_combinations(code, &keypad_lookup);

        let best_path_len: usize = keypad_paths
            .iter()
            .map(|keypad_path| {
                let mut path_parts: Vec<String> = keypad_path
                    .split("A")
                    .map(|s| {
                        let mut s = s.to_string();
                        s.push_str("A");
                        s
                    })
                    .collect();
                path_parts.pop();

                path_parts
                    .into_iter()
                    .map(|p| rec_count_moves(p, 24, &ctrl_lookup, &mut count_lookup))
                    .sum()
            })
            .min()
            .unwrap();

        let numeric_part: usize = code[..code.len() - 1].parse().unwrap();

        score += best_path_len * numeric_part;
    }

    println!("task2:\t{score}");
}

fn main() {
    task1();
    task2();
}
