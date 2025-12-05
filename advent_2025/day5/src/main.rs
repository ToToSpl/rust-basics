use std::fs;

const INPUT: &str = "input.txt";
// const INPUT: &str = "input.test.txt";

fn load_ranges_and_ids() -> (Vec<(u64, u64)>, Vec<u64>) {
    let contents = fs::read_to_string(INPUT).unwrap();

    let mut lines = contents.lines().into_iter();
    let mut ranges = Vec::new();
    let mut ids = Vec::new();

    loop {
        let line = lines.next().unwrap();
        if line.len() == 0 {
            break;
        }

        let range: Vec<&str> = line.split('-').collect();
        ranges.push((range[0].parse().unwrap(), range[1].parse().unwrap()));
    }

    for line in lines {
        ids.push(line.parse().unwrap());
    }

    (ranges, ids)
}

fn task1() {
    let (ranges, ids) = load_ranges_and_ids();

    let mut available_ingredients = 0;

    for id in ids {
        for &(l, h) in &ranges {
            if id >= l && id <= h {
                available_ingredients += 1;
                break;
            }
        }
    }

    println!("task1:\t{available_ingredients}");
}

fn task2() {
    let (mut ranges, _ids) = load_ranges_and_ids();

    let mut new_ranges = Vec::new();

    while !ranges.is_empty() {
        let mut summed_range = ranges.pop().unwrap();

        'outer: loop {
            for i in 0..ranges.len() {
                let r = ranges[i];
                let mut changed = false;

                if r.0 < summed_range.0 && r.1 >= summed_range.0 - 1 {
                    summed_range.0 = r.0;
                    changed = true;
                }

                if r.1 > summed_range.1 && r.0 <= summed_range.1 + 1 {
                    summed_range.1 = r.1;
                    changed = true;
                }

                let in_range = r.0 >= summed_range.0 && r.1 <= summed_range.1;

                if changed || in_range {
                    ranges.remove(i);
                    continue 'outer;
                }
            }

            break;
        }

        new_ranges.push(summed_range);
    }

    let mut ids_sum = 0;

    for (l, h) in new_ranges {
        ids_sum += h - l + 1;
    }

    println!("task2:\t{ids_sum}");
}

fn main() {
    task1();
    task2();
}
