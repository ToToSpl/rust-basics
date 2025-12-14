use std::{collections::HashMap, fs};

const INPUT: &str = "input.txt";

fn load_graph() -> HashMap<String, Vec<String>> {
    fs::read_to_string(INPUT)
        .unwrap()
        .lines()
        .map(|l| {
            let mut v = l.split(": ");
            let key = v.next().unwrap().to_string();
            let vals = v
                .next()
                .unwrap()
                .split(" ")
                .map(|s| s.to_string())
                .collect();

            (key, vals)
        })
        .collect()
}

fn route_recur(
    graph: &HashMap<String, Vec<String>>,
    current: &String,
    end: &String,
    forbidden: &Vec<String>,
    lookup: &mut HashMap<String, u64>,
) -> u64 {
    if forbidden.contains(current) {
        return 0;
    }

    if current == end {
        return 1;
    }

    if let Some(&v) = lookup.get(current) {
        return v;
    }

    let mut sum = 0;
    for n in graph.get(current).unwrap() {
        sum += route_recur(graph, n, end, forbidden, lookup)
    }

    lookup.insert(current.clone(), sum);

    sum
}

fn task1() {
    let graph = load_graph();

    let paths = route_recur(
        &graph,
        &"you".to_string(),
        &"out".to_string(),
        &vec![],
        &mut HashMap::new(),
    );

    println!("task1:\t{paths}");
}

fn task2() {
    let graph = load_graph();

    macro_rules! route {
        ($start: expr, $end: expr, $forbidden: expr) => {
            route_recur(
                &graph,
                &$start.to_string(),
                &$end.to_string(),
                &$forbidden,
                &mut HashMap::new(),
            )
        };
    }

    let svr_to_fft = route!("svr", "fft", vec!["out".into(), "dac".into()]);
    let fft_to_dac = route!("fft", "dac", vec!["out".into(), "svr".into()]);
    let dac_to_out = route!("dac", "out", vec!["fft".into(), "svr".into()]);

    let svr_to_dac = route!("svr", "dac", vec!["out".into(), "fft".into()]);
    let dac_to_fft = route!("dac", "fft", vec!["out".into(), "svr".into()]);
    let fft_to_out = route!("fft", "out", vec!["dac".into(), "svr".into()]);

    let path_1 = svr_to_fft * fft_to_dac * dac_to_out;
    let path_2 = svr_to_dac * dac_to_fft * fft_to_out;

    let sum = path_1 + path_2;

    println!("task2:\t{sum}");
}

fn main() {
    task1();
    task2();
}
