use std::collections::HashMap;
use std::io;

fn main() {
    let mut hash_db: HashMap<String, Vec<String>> = HashMap::new();
    let mut input_buffer = String::new();
    loop {
        print_options();

        // read input from the user
        io::stdin().read_line(&mut input_buffer).unwrap();
        let input_words: Vec<&str> = input_buffer.trim_end().split(" ").collect();

        // switch to keyword
        match input_words[0] {
            "Add" => add_registry(&mut hash_db, &input_words),
            "Show" => show_dep(&hash_db, &input_words),
            "ShowAll" => show_all(&hash_db),
            _ => println!("Wrong keyword!"),
        }

        input_buffer.clear();
    }
}

fn show_dep(db: &HashMap<String, Vec<String>>, words: &Vec<&str>) {
    if words.len() != 2 {
        println!("Wrong parameters!");
        return;
    }

    match db.get(words[1]) {
        Some(t) => {
            println!("{:}:", words[1]);
            for s in t {
                println!("\t{:}", s);
            }
        }
        None => println!("This department is empty."),
    }
}

fn show_all(db: &HashMap<String, Vec<String>>) {
    for (dep, names) in db {
        println!("Department {:}:", dep);
        for name in names {
            println!("\t{:}", name);
        }
    }
}

fn add_registry(db: &mut HashMap<String, Vec<String>>, words: &Vec<&str>) {
    if words.len() != 4 {
        println!("Wrong parameters!");
        return;
    }

    let dep = words[3].clone();
    let name = words[1].clone();

    db.entry(dep.to_string())
        .and_modify(|v| v.push(name.to_string()))
        .or_insert(vec![name.to_string()]);
}

fn print_options() {
    println!("Available options:");
    println!("Add: ex: 'Add Emma to Engineering'");
    println!("Show: ex: 'Show Engineering'");
    println!("ShowAll: ex: 'ShowAll'");
    println!("");
}
