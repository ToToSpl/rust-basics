use std::{
    collections::{HashMap, HashSet},
    fs,
};

const INPUT: &str = "input.txt";

fn task1() {
    let contents = fs::read_to_string(INPUT).unwrap();
    let mut connections: HashMap<String, Vec<String>> = HashMap::new();

    contents.lines().for_each(|l| {
        let con: Vec<String> = l.split("-").map(|c| c.to_string()).collect();
        connections
            .entry(con[0].clone())
            .or_insert(Vec::new())
            .push(con[1].clone());
        connections
            .entry(con[1].to_owned())
            .or_insert(Vec::new())
            .push(con[0].to_owned());
    });

    let mut tri_sets: HashSet<Vec<String>> = HashSet::new();

    for (from, neighs) in &connections {
        for neigh in neighs {
            let neighs_conns = &connections[neigh];

            for neighs_conn in neighs_conns {
                if neighs.contains(neighs_conn) {
                    let mut tri_set = vec![from.clone(), neigh.clone(), neighs_conn.clone()];
                    tri_set.sort();
                    tri_sets.insert(tri_set);
                }
            }
        }
    }

    let score = tri_sets
        .iter()
        .filter_map(|s| s.iter().find(|c| c.chars().nth(0).unwrap() == 't'))
        .count();

    println!("task1:\t{score}");
}

fn task2() {
    let contents = fs::read_to_string(INPUT).unwrap();
    let mut connections: HashMap<String, Vec<String>> = HashMap::new();

    contents.lines().for_each(|l| {
        let con: Vec<String> = l.split("-").map(|c| c.to_string()).collect();
        connections
            .entry(con[0].clone())
            .or_insert(Vec::new())
            .push(con[1].clone());
        connections
            .entry(con[1].to_owned())
            .or_insert(Vec::new())
            .push(con[0].to_owned());
    });

    let mut full_sets: HashSet<Vec<String>> = HashSet::new();

    for (from, neighs) in &connections {
        let overlaping_sets: Vec<Vec<String>> = neighs
            .iter()
            .map(|neigh| {
                let neigh_conns = &connections[neigh];
                neighs
                    .iter()
                    .filter(|n| neigh_conns.contains(n))
                    .map(|n| n.clone())
                    .collect()
            })
            .collect();

        let mut conn_count: HashMap<&String, usize> = HashMap::new();
        for neigh in neighs {
            *conn_count.entry(neigh).or_insert(0) += 1;
        }

        overlaping_sets.iter().for_each(|s| {
            s.iter()
                .for_each(|c| *conn_count.entry(c).or_insert(0) += 1)
        });

        let max_overlap_len = *conn_count.values().max().unwrap();

        let mut max_overlap: Vec<String> = conn_count
            .iter()
            .filter_map(|(&s, &c)| {
                if c == max_overlap_len {
                    Some(s.clone())
                } else {
                    None
                }
            })
            .collect();

        if max_overlap_len == max_overlap.len() {
            max_overlap.push(from.clone());

            max_overlap.sort();

            full_sets.insert(max_overlap);
        }
    }

    let biggest_set = full_sets.into_iter().max_by_key(|s| s.len()).unwrap();
    let passwd: String = biggest_set.join(",");

    println!("task2:\t{passwd}");
}

fn main() {
    task1();
    task2();
}
