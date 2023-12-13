use std::fs;
use std::iter::zip;
use std::str::Lines;

const INPUT: &str = "input.txt";

#[derive(Clone)]
struct Note {
    rows: Vec<String>,
    cols: Vec<String>,
    width: usize,
    height: usize,
}

impl Note {
    fn new(lines: &mut Lines) -> Option<Note> {
        let rows: Vec<_> = lines
            .take_while(|l| l.len() > 0)
            .map(|l| l.to_string())
            .collect();
        let height = rows.len();
        if height == 0 {
            return None;
        }

        let width = rows[0].len();

        let mut cols = Vec::new();
        for i in 0..width {
            let mut col = Vec::new();
            for j in 0..height {
                col.push(rows[j].chars().nth(i).unwrap());
            }
            cols.push(String::from_iter(col));
        }

        Some(Note {
            rows,
            cols,
            width,
            height,
        })
    }

    fn find_refl_vertical(&self) -> Vec<usize> {
        let mut refls = Vec::new();
        for middle in 1..self.width {
            let mut count = 0;
            loop {
                let left = middle - 1 - count;
                let right = middle + count;
                if self.cols[left] != self.cols[right] {
                    break;
                }
                if left == 0 || right == self.width - 1 {
                    refls.push(middle);
                    break;
                }
                count += 1;
            }
        }
        refls
    }

    fn find_refl_horizontal(&self) -> Vec<usize> {
        let mut refls = Vec::new();
        for middle in 1..self.height {
            let mut count = 0;
            loop {
                let up = middle - 1 - count;
                let down = middle + count;
                if self.rows[up] != self.rows[down] {
                    break;
                }
                if up == 0 || down == self.height - 1 {
                    refls.push(middle);
                    break;
                }
                count += 1;
            }
        }
        refls
    }

    fn fix_smudge(&mut self) {
        let hor = self.find_refl_horizontal();
        let ver = self.find_refl_vertical();

        for y in 0..self.height {
            for x in 0..self.width {
                let mut temp = self.clone();
                let removed = temp.rows[y].remove(x);
                let c = if removed == '#' { '.' } else { '#' };
                temp.rows[y].insert(x, c);

                assert_eq!(temp.cols[x].remove(y), removed);
                temp.cols[x].insert(y, c);

                let t_hor = temp.find_refl_horizontal();
                if t_hor != hor {
                    if t_hor.len() >= hor.len() {
                        *self = temp;
                        return;
                    }
                }

                let t_ver = temp.find_refl_vertical();
                if t_ver != ver {
                    if t_ver.len() >= ver.len() {
                        *self = temp;
                        return;
                    }
                }
            }
        }

        panic!("Smudge not found!");
    }
}

fn task1() {
    let contents = fs::read_to_string(INPUT).unwrap();
    let mut lines = contents.lines();
    let notes = {
        let mut notes = Vec::new();
        while let Some(note) = Note::new(&mut lines) {
            notes.push(note);
        }
        notes
    };

    let refls = notes
        .iter()
        .map(|n| (n.find_refl_vertical(), n.find_refl_horizontal()))
        .collect::<Vec<_>>();

    let mut sum = 0;
    for refl in &refls {
        if let Some(v) = refl.0.first() {
            sum += v;
        }

        if let Some(h) = refl.1.first() {
            sum += 100 * h;
        }
    }

    println!("task1 {:?}", sum);
}

fn task2() {
    let contents = fs::read_to_string(INPUT).unwrap();
    let mut lines = contents.lines();
    let mut notes = {
        let mut notes = Vec::new();
        while let Some(note) = Note::new(&mut lines) {
            notes.push(note);
        }
        notes
    };

    let refls_with_smudge = notes
        .iter()
        .map(|n| (n.find_refl_vertical(), n.find_refl_horizontal()))
        .collect::<Vec<_>>();

    notes.iter_mut().for_each(|n| n.fix_smudge());

    let refls_without_smudge = notes
        .iter()
        .map(|n| (n.find_refl_vertical(), n.find_refl_horizontal()))
        .collect::<Vec<_>>();

    let mut sum = 0;
    for (with, without) in zip(refls_with_smudge, refls_without_smudge) {
        let filtered_v: Vec<_> = without.0.iter().filter(|v| !with.0.contains(v)).collect();
        let filtered_h: Vec<_> = without.1.iter().filter(|v| !with.1.contains(v)).collect();

        if let Some(v) = filtered_v.first() {
            sum += *v;
        }

        if let Some(h) = filtered_h.first() {
            sum += 100 * *h;
        }
    }

    println!("task2 {:?}", sum);
}

fn main() {
    task1();
    task2();
}
