use std::fs;

const INPUT: &str = "input.test.txt";

#[derive(Clone, Copy, Debug)]
struct Vector<T> {
    x: T,
    y: T,
    z: T,
}

impl<T> Vector<T> {
    fn new(l: &str) -> Vector<T>
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
        T: Copy,
    {
        let xyz = l
            .split(",")
            .map(|t| t.split_whitespace().nth(0).unwrap().parse::<T>().unwrap())
            .collect::<Vec<_>>();
        Vector {
            x: xyz[0],
            y: xyz[1],
            z: xyz[2],
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Path {
    p: Vector<i64>,
    v: Vector<i64>,
}

#[derive(Clone, Copy, Debug)]
struct Intersection {
    t1: f64,
    t2: f64,
    p: Vector<f64>,
}

fn paths_intersection(a: &Path, b: &Path) -> Option<Intersection> {
    let denom = b.v.y * a.v.x - a.v.y * b.v.x;
    if denom == 0 {
        return None;
    }
    let d1 = b.p.x - a.p.x;
    let d2 = a.p.y - b.p.y;
    let numer1 = b.v.y * d1 + b.v.x * d2;
    let numer2 = a.v.y * d1 + a.v.x * d2;
    let t1 = numer1 as f64 / denom as f64;
    let t2 = numer2 as f64 / denom as f64;
    Some(Intersection {
        t1,
        t2,
        p: Vector {
            x: a.p.x as f64 + a.v.x as f64 * t1,
            y: a.p.y as f64 + a.v.y as f64 * t1,
            z: 0.0,
        },
    })
}

fn task1() {
    let contents = fs::read_to_string(INPUT).unwrap();
    let paths = contents
        .lines()
        .map(|l| {
            let vecs = l.split(" @ ").map(Vector::<i64>::new).collect::<Vec<_>>();
            Path {
                p: vecs[0],
                v: vecs[1],
            }
        })
        .collect::<Vec<_>>();

    let mut intersections = Vec::new();
    for i in 0..paths.len() {
        let a = &paths[i];
        for j in i + 1..paths.len() {
            intersections.push(paths_intersection(a, &paths[j]));
        }
    }

    let range: (f64, f64) = (200000000000000.0, 400000000000000.0);

    let filtered = intersections
        .iter()
        .filter_map(|i| *i)
        .filter(|i| {
            i.t1 >= 0.0
                && i.t2 >= 0.0
                && range.0 <= i.p.x
                && i.p.x <= range.1
                && range.0 <= i.p.y
                && i.p.y <= range.1
        })
        .collect::<Vec<_>>();

    println!("task1 {:?}", filtered.len());
}

fn main() {
    task1();
    // task2();
}
