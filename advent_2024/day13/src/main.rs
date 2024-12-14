use std::fs;

const INPUT: &str = "input.txt";

#[derive(Debug, Clone, Copy)]
struct Vector<T> {
    x: T,
    y: T,
}

fn get_arcades() -> Vec<(Vector<usize>, Vector<usize>, Vector<usize>)> {
    let contents = fs::read_to_string(INPUT).unwrap();
    let mut lines = contents.lines();
    let mut arcades = Vec::new();

    while let Some(line) = lines.next() {
        if line == "" {
            continue;
        }

        let a_v: Vec<usize> = line
            .chars()
            .skip(12)
            .collect::<String>()
            .split(", Y+")
            .map(|n| n.parse().unwrap())
            .collect();
        let a = Vector {
            x: a_v[0],
            y: a_v[1],
        };

        let b_v: Vec<usize> = lines
            .next()
            .unwrap()
            .chars()
            .skip(12)
            .collect::<String>()
            .split(", Y+")
            .map(|n| n.parse().unwrap())
            .collect();
        let b = Vector {
            x: b_v[0],
            y: b_v[1],
        };

        let price_v: Vec<usize> = lines
            .next()
            .unwrap()
            .chars()
            .skip(9)
            .collect::<String>()
            .split(", Y=")
            .map(|n| n.parse().unwrap())
            .collect();
        let price = Vector {
            x: price_v[0],
            y: price_v[1],
        };

        arcades.push((a, b, price));
    }

    arcades
}

fn calculate_click_count(
    a: Vector<usize>,
    b: Vector<usize>,
    c: Vector<usize>,
) -> Option<(usize, usize)> {
    let j_1_1 = c.y * a.x;
    let j_1_2 = a.y * c.x;
    let j_1 = if j_1_1 >= j_1_2 {
        j_1_1 - j_1_2
    } else {
        return None;
    };
    let j_2_1 = a.x * b.y;
    let j_2_2 = b.x * a.y;
    let j_2 = if j_2_1 >= j_2_2 {
        j_2_1 - j_2_2
    } else {
        return None;
    };
    let j = if j_1 % j_2 == 0 {
        j_1 / j_2
    } else {
        return None;
    };

    let i_1 = if c.x >= j * b.x {
        c.x - j * b.x
    } else {
        return None;
    };
    let i = if i_1 % a.x == 0 {
        i_1 / a.x
    } else {
        return None;
    };

    return Some((i, j));
}

fn task1() {
    let score: usize = get_arcades()
        .iter()
        .filter_map(|&(a, b, prize)| {
            let comb_1 = calculate_click_count(a, b, prize);
            let comb_2 = calculate_click_count(b, a, prize);

            match (comb_1, comb_2) {
                (None, None) => None,
                (Some(c1), None) => Some(3 * c1.0 + 1 * c1.1),
                (None, Some(c2)) => Some(3 * c2.1 + 1 * c2.0),
                (Some(c1), Some(c2)) => {
                    let p_1 = 3 * c1.0 + 1 * c1.1;
                    let p_2 = 3 * c2.1 + 1 * c2.0;
                    Some(usize::min(p_1, p_2))
                }
            }
        })
        .sum();

    println!("task1:\t{score}");
}

fn task2() {
    let score: usize = get_arcades()
        .iter()
        .map(|&(a, b, prize)| {
            (
                a,
                b,
                Vector {
                    x: prize.x + 10_000_000_000_000,
                    y: prize.y + 10_000_000_000_000,
                },
            )
        })
        .filter_map(|(a, b, prize)| {
            let comb_1 = calculate_click_count(a, b, prize);
            let comb_2 = calculate_click_count(b, a, prize);

            match (comb_1, comb_2) {
                (None, None) => None,
                (Some(c1), None) => Some(3 * c1.0 + 1 * c1.1),
                (None, Some(c2)) => Some(3 * c2.1 + 1 * c2.0),
                (Some(c1), Some(c2)) => {
                    let p_1 = 3 * c1.0 + 1 * c1.1;
                    let p_2 = 3 * c2.1 + 1 * c2.0;
                    Some(usize::min(p_1, p_2))
                }
            }
        })
        .sum();

    println!("task2:\t{score}");
}

fn main() {
    task1();
    task2();
}
