use rand::Rng;
use std::collections::HashMap;

fn main() {
    // create random array of integers from -10 to 10
    let mut arr: [i32; 15] = [0; 15];
    for i in 0..arr.len() {
        arr[i] = rand::thread_rng().gen_range(-11..11);
    }

    let mut vector = Vec::from(arr);
    vector.sort();
    println!("vector: {:?}", vector);

    let median: f32 = if vector.len() % 2 == 0 {
        let twice = (vector[vector.len() / 2 - 1] + vector[vector.len() / 2]) as f32;
        twice / 2.0
    } else {
        vector[vector.len() / 2] as f32
    };

    // modal using hash map
    let mut hash = HashMap::new();
    let mut max_count = 0;
    let mut modal = vector[0];
    for e in vector {
        let c = match hash.get(&e) {
            Some(i) => i + 1,
            None => 1,
        };
        hash.insert(e, c);
        if c > max_count {
            max_count = c;
            modal = e;
        }
    }

    println!("modal: {:?} with count {:?}", modal, max_count);
    println!("median: {:?}", median);
}
