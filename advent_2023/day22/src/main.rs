use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

const INPUT: &str = "input.txt";

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Clone, Debug)]
struct Brick {
    name: String,
    edge_bottom: Point,
    edge_top: Point,
    supports: HashSet<String>,
    supported_by: HashSet<String>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Checker {
    edge_bottom: Point,
    edge_top: Point,
}

impl Brick {
    fn new(line: &str, name: usize) -> Brick {
        let name = name.to_string();
        let edges = line
            .split('~')
            .enumerate()
            .map(|(i, p_s)| {
                let p = p_s
                    .split(',')
                    .map(|s| s.parse::<i64>().unwrap() + i as i64)
                    .collect::<Vec<_>>();
                Point {
                    x: p[0],
                    y: p[1],
                    z: p[2],
                }
            })
            .collect::<Vec<_>>();

        assert!(edges[0].x < edges[1].x);
        assert!(edges[0].y < edges[1].y);
        assert!(edges[0].z < edges[1].z);

        Brick {
            name,
            edge_bottom: edges[0],
            edge_top: edges[1],
            supports: HashSet::new(),
            supported_by: HashSet::new(),
        }
    }

    fn collides(&self, other: &Checker) -> bool {
        self.edge_bottom.x < other.edge_top.x
            && self.edge_top.x > other.edge_bottom.x
            && self.edge_bottom.y < other.edge_top.y
            && self.edge_top.y > other.edge_bottom.y
            && self.edge_bottom.z < other.edge_top.z
            && self.edge_top.z > other.edge_bottom.z
    }

    fn check_bottom_collisions(&self, others: &[Brick]) -> HashSet<String> {
        let mut collides_with = HashSet::new();
        for x in self.edge_bottom.x..self.edge_top.x {
            for y in self.edge_bottom.y..self.edge_top.y {
                let checker = Checker {
                    edge_bottom: Point {
                        x: x as i64,
                        y: y as i64,
                        z: self.edge_bottom.z - 1,
                    },
                    edge_top: Point {
                        x: x as i64 + 1,
                        y: y as i64 + 1,
                        z: self.edge_bottom.z,
                    },
                };

                for other in others {
                    if self.name == other.name {
                        continue;
                    }
                    if other.collides(&checker) {
                        collides_with.insert(other.name.clone());
                    }
                }
            }
        }

        collides_with
    }

    fn place_down(&mut self, others: &mut [Brick]) {
        while self.edge_bottom.z > 0 {
            let bottoms = self.check_bottom_collisions(others);
            if bottoms.len() != 0 {
                for b in &bottoms {
                    others
                        .iter_mut()
                        .find(|o| o.name == *b)
                        .unwrap()
                        .supports
                        .insert(self.name.clone());
                }
                self.supported_by = bottoms;
                return;
            }
            self.edge_bottom.z -= 1;
            self.edge_top.z -= 1;
        }
    }

    fn count_other_fallen(&self, others: &HashMap<String, Brick>) -> usize {
        let mut fallen = 0;
        let mut destroyed = HashSet::from([self.name.clone()]);
        let mut stack = VecDeque::from_iter(self.supports.clone().into_iter());
        'outer: while let Some(b) = stack.pop_front() {
            if destroyed.get(&b).is_some() {
                continue;
            }
            for s_b in &others[&b].supported_by {
                if destroyed.get(s_b).is_none() {
                    continue 'outer;
                }
            }
            fallen += 1;
            destroyed.insert(b.clone());
            for s_b in &others[&b].supports {
                stack.push_back(s_b.clone());
            }
        }

        fallen
    }
}

fn task1() {
    let contents = fs::read_to_string(INPUT).unwrap();
    let mut bricks = contents
        .lines()
        .enumerate()
        .map(|(i, l)| Brick::new(l, i))
        .collect::<Vec<_>>();
    bricks.sort_unstable_by_key(|b| b.edge_bottom.z);

    let mut bricks_placed = Vec::new();
    for b in bricks.iter_mut() {
        b.place_down(&mut bricks_placed);
        bricks_placed.push(b.clone());
    }

    let bricks_placed: HashMap<String, Brick> =
        HashMap::from_iter(bricks_placed.into_iter().map(|b| (b.name.clone(), b)));

    let can_remove = bricks_placed
        .iter()
        .filter(|(_, b)| {
            for sup in &b.supports {
                if bricks_placed.get(sup).unwrap().supported_by.len() == 1 {
                    return false;
                }
            }

            true
        })
        .count();

    println!("task1 {:?}", can_remove);
}

fn task2() {
    let contents = fs::read_to_string(INPUT).unwrap();
    let mut bricks = contents
        .lines()
        .enumerate()
        .map(|(i, l)| Brick::new(l, i))
        .collect::<Vec<_>>();
    bricks.sort_unstable_by_key(|b| b.edge_bottom.z);

    let mut bricks_placed = Vec::new();
    for b in bricks.iter_mut() {
        b.place_down(&mut bricks_placed);
        bricks_placed.push(b.clone());
    }

    let bricks_placed: HashMap<String, Brick> =
        HashMap::from_iter(bricks_placed.into_iter().map(|b| (b.name.clone(), b)));

    // for (_, b) in &bricks_placed {
    //     println!("{:?}", b);
    // }

    let counts = bricks_placed
        .iter()
        .map(|(_, b)| b.count_other_fallen(&bricks_placed))
        .collect::<Vec<_>>();

    println!("task2 {:?}", counts.into_iter().sum::<usize>());
}

fn main() {
    task1();
    task2();
}
