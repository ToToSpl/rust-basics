const INPUT: &str = "./input.txt";

use std::fs;

fn task(unique_len: usize) -> i32 {
    let contents = fs::read_to_string(INPUT).expect("Should have been able to read the file");
    let mut input = contents.chars();
    input.next_back(); // remove '\n' at the end
    let mut buffer: Vec<u32> = vec![0; unique_len];
    for i in 0..(unique_len - 1) {
        buffer[i] = 0b1 << (input.next().unwrap() as u8 - 'a' as u8);
    }

    let mut pointer = unique_len - 2;
    let mut counter = unique_len - 1;

    for c in input {
        pointer = (pointer + 1) % unique_len;
        counter += 1;
        buffer[pointer] = 0b1 << (c as u8 - 'a' as u8);

        let mut bitmask = 0;
        for i in 0..unique_len {
            bitmask |= buffer[i];
        }

        if bitmask.count_ones() == unique_len as u32 {
            return counter as i32;
        }
    }
    -1
}

fn main() {
    let task1 = task(4);
    println!("{:?}", task1);

    let task2 = task(14);
    println!("{:?}", task2);
}
