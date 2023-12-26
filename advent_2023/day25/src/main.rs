use std::collections::{HashMap, HashSet};
use std::fs;

use priority_queue::PriorityQueue;
use tqdm::tqdm;

const INPUT: &str = "input.txt";

#[derive(Debug)]
struct Graph {
    nodes: HashMap<String, HashSet<String>>,
}

impl Graph {
    fn new(input: &str) -> Graph {
        let contents = fs::read_to_string(input).unwrap();

        let mut nodes = HashMap::new();

        contents.lines().for_each(|l| {
            let [node, connected] = l.split(": ").collect::<Vec<_>>().try_into().unwrap();
            let node = node.to_string();
            let connected = connected.split(" ").collect::<Vec<_>>();

            match nodes.get_mut(&node) {
                Some(_) => {}
                None => {
                    nodes.insert(node.clone(), HashSet::new());
                }
            }

            for c in connected {
                nodes.get_mut(&node).unwrap().insert(c.to_string());
                match nodes.get_mut(c) {
                    None => {
                        nodes.insert(c.to_string(), HashSet::from([node.clone()]));
                    }
                    Some(h) => {
                        h.insert(node.clone());
                    }
                }
            }
        });

        Graph { nodes }
    }

    fn remove_connection(&mut self, conn: &(String, String)) {
        self.nodes.get_mut(&conn.0).unwrap().remove(&conn.1);
        self.nodes.get_mut(&conn.1).unwrap().remove(&conn.0);
    }

    fn path_from_to_others<'a>(&'a self, start: &'a String) -> HashMap<String, Vec<&'a String>> {
        let mut discovered: HashSet<String> = HashSet::new();
        let mut pq = PriorityQueue::new();
        let mut paths = HashMap::new();
        pq.push((start.clone(), vec![start]), 0);
        while let Some((n, d)) = pq.pop() {
            if discovered.contains(&n.0) {
                continue;
            }

            for c in self.nodes.get(&n.0).unwrap() {
                let mut path = n.1.clone();
                path.push(c);
                pq.push((c.clone(), path), d - 1);
            }
            paths.insert(n.0.clone(), n.1);
            discovered.insert(n.0);
        }
        assert!(paths.len() == self.nodes.len());
        paths
    }

    fn group_size(&self, group_member: &String) -> usize {
        let mut stack = vec![group_member];
        let mut discovered: HashSet<&String> = HashSet::new();
        while let Some(n) = stack.pop() {
            if discovered.contains(n) {
                continue;
            }
            for c in self.nodes.get(n).unwrap() {
                stack.push(c);
            }
            discovered.insert(n);
        }
        discovered.len()
    }
}

fn task1() {
    let mut graph = Graph::new(INPUT);

    let connection_importance = {
        let mut connection_importance: HashMap<(&String, &String), usize> = HashMap::new();
        for (n1, _) in tqdm(graph.nodes.iter()) {
            let paths = graph.path_from_to_others(n1);
            for (_n2, path) in &paths {
                path.windows(2).for_each(|p| {
                    if let Some(count) = connection_importance.get(&(p[0], p[1])) {
                        connection_importance.insert((p[0], p[1]), count + 1);
                    } else if let Some(count) = connection_importance.get(&(p[1], p[0])) {
                        connection_importance.insert((p[1], p[0]), count + 1);
                    } else {
                        connection_importance.insert((p[0], p[1]), 1);
                    }
                });
            }
        }
        connection_importance
            .into_iter()
            .map(|c| ((c.0 .0.clone(), c.0 .1.clone()), c.1))
            .collect::<Vec<_>>()
    };

    let mut conns = connection_importance.into_iter().collect::<Vec<_>>();
    conns.sort_unstable_by_key(|c| c.1);
    conns.reverse();

    for conn in conns.iter().take(3) {
        graph.remove_connection(&conn.0);
        println!("{:?}", conn);
    }
    println!("{:?}\n", conns.iter().take(3).map(|c| c.1).sum::<usize>());

    let group1 = graph.group_size(&graph.nodes.iter().nth(0).unwrap().0);
    let group2 = graph.nodes.len() - group1;

    println!("task1 {:?}", group1 * group2);
}

fn main() {
    task1();
}
