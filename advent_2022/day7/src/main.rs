const INPUT: &str = "./input.txt";
const CMD_TER: &str = "$";
const CMD_HOME: &str = "/";
const CMD_BACK: &str = "..";
const CMD_DIR: &str = "dir";
const CMD_CD: &str = "cd";

const MAX_DIR_SIZE: usize = 100000;

const DEVICE_SIZE: usize = 70000000;
const UPDATE_SIZE: usize = 30000000;

use std::{cell::RefCell, fs, rc::Rc};

struct DirNode {
    dir_size: usize,
    sum_size: usize,
    name: String,
    files: Vec<File>,
    dirs: Vec<Dir>,
    higher: Option<Dir>,
}
type Dir = Rc<RefCell<DirNode>>;

struct File {
    _size: usize,
    _name: String,
}

impl DirNode {
    fn new(name: &str) -> DirNode {
        DirNode {
            dir_size: 0,
            sum_size: 0,
            name: name.to_string(),
            files: Vec::new(),
            dirs: Vec::new(),
            higher: Option::None,
        }
    }
}

fn dir_new(name: &str) -> Dir {
    Rc::new(RefCell::new(DirNode::new(name)))
}

fn create_dir_tree() -> Dir {
    let root_node = dir_new("/");
    let mut curr_node = Rc::clone(&root_node);

    let contents = fs::read_to_string(INPUT).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.split("\n").filter(|l| l.len() != 0).collect();
    for line in lines {
        let words: Vec<&str> = line.split(" ").collect();
        if words[0] == CMD_TER {
            if words[1] == CMD_CD {
                // CD command
                let dir = words[2];
                if dir == CMD_HOME {
                    curr_node = Rc::clone(&root_node);
                } else if dir == CMD_BACK {
                    let temp = Rc::clone(&curr_node);
                    curr_node = Rc::clone(&temp.borrow().higher.as_ref().unwrap());
                } else {
                    let mut next_node = Rc::clone(&curr_node);
                    for d in &curr_node.borrow_mut().dirs {
                        if dir == d.borrow().name {
                            next_node = Rc::clone(d);
                            break;
                        }
                    }
                    curr_node = next_node;
                }
            }
        } else {
            // output from LS
            if words[0] == CMD_DIR {
                // words[1] is a dir
                let new_node = dir_new(words[1]);
                new_node.borrow_mut().higher = Some(Rc::clone(&curr_node));
                curr_node.borrow_mut().dirs.push(Rc::clone(&new_node));
            } else {
                // words[1] is a file
                let file_size = words[0].parse::<usize>().unwrap();
                curr_node.borrow_mut().dir_size += file_size;
                curr_node.borrow_mut().files.push(File {
                    _size: file_size,
                    _name: words[0].to_string(),
                });
            }
        }
    }
    create_dir_tree_recur(&root_node);

    Rc::clone(&root_node)
}

fn create_dir_tree_recur(node: &Dir) -> usize {
    let mut deep_size = 0;
    for d in &node.borrow().dirs {
        deep_size += create_dir_tree_recur(d);
    }

    let val = node.borrow().dir_size;
    node.borrow_mut().sum_size = val + deep_size;
    node.borrow().sum_size
}

fn calc_size_rec(node: &Dir, found_size: &mut usize) {
    for d in &node.borrow().dirs {
        calc_size_rec(d, found_size);
    }

    if node.borrow().sum_size <= MAX_DIR_SIZE {
        *found_size += node.borrow().sum_size;
    }
}

fn task1(root_node: &Dir) -> usize {
    let mut found_size = 0;
    calc_size_rec(&root_node, &mut found_size);
    found_size
}

fn find_size_rec(node: &Dir, required_space: usize, found_dir_size: &mut usize) {
    if node.borrow().sum_size >= required_space {
        let div = node.borrow().sum_size - required_space;
        if div < (*found_dir_size - required_space) {
            *found_dir_size = node.borrow().sum_size;
        }
        for d in &node.borrow().dirs {
            find_size_rec(d, required_space, found_dir_size);
        }
    }
}

fn task2(root_node: &Dir) -> usize {
    let mut found_dir_size = root_node.borrow().sum_size;
    let required_space = UPDATE_SIZE - (DEVICE_SIZE - root_node.borrow().sum_size);
    find_size_rec(root_node, required_space, &mut found_dir_size);
    found_dir_size
}

fn main() {
    let root_dir = create_dir_tree();
    println!("files size: {:?}", root_dir.borrow().sum_size);

    let task1 = task1(&root_dir);
    println!("{:?}", task1);

    let task2 = task2(&root_dir);
    println!("{:?}", task2);
}
