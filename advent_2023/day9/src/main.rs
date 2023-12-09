use std::fs;

const INPUT: &str = "input.txt";

fn predict_next_val(arr: &Vec<i64>) -> i64 {
    if arr.len() == 1 {
        if arr[0] == 0 {
            return 0;
        }
        panic!("Obtained undefined behaviour!");
    }

    let difs: Vec<_> = arr.windows(2).map(|c| c[1] - c[0]).collect();
    let next = if difs.iter().sum::<i64>() == 0 {
        0
    } else {
        predict_next_val(&difs)
    };

    return arr.last().unwrap() + next;
}

fn predict_prev_val(arr: &Vec<i64>) -> i64 {
    if arr.len() == 1 {
        if arr[0] == 0 {
            return 0;
        }
        panic!("Obtained undefined behaviour!");
    }

    let difs: Vec<_> = arr.windows(2).map(|c| c[1] - c[0]).collect();
    let next = if difs.iter().sum::<i64>() == 0 {
        0
    } else {
        predict_prev_val(&difs)
    };

    return arr.first().unwrap() - next;
}

fn task1() {
    let contents = fs::read_to_string(INPUT).unwrap();
    let readings: Vec<Vec<i64>> = contents
        .lines()
        .map(|l| l.split(' ').map(|d| d.parse::<i64>().unwrap()).collect())
        .collect();

    let predicts: Vec<_> = readings.iter().map(|r| predict_next_val(&r)).collect();
    println!("task1 {:?}", predicts.iter().sum::<i64>());
}

fn task2() {
    let contents = fs::read_to_string(INPUT).unwrap();
    let readings: Vec<Vec<i64>> = contents
        .lines()
        .map(|l| l.split(' ').map(|d| d.parse::<i64>().unwrap()).collect())
        .collect();

    let predicts: Vec<_> = readings.iter().map(|r| predict_prev_val(&r)).collect();
    println!("task2 {:?}", predicts.iter().sum::<i64>());
}

fn main() {
    task1();
    task2();
}
