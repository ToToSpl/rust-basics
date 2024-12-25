use std::{cell::RefCell, collections::HashMap, fs, rc::Rc};

const INPUT: &str = "input.txt";

#[derive(PartialEq, Eq)]
enum Operation {
    And,
    Or,
    Xor,
}
impl Operation {
    fn from_str(input: &str) -> Operation {
        use Operation::*;
        match input {
            "AND" => And,
            "OR" => Or,
            "XOR" => Xor,
            _e => panic!("cannot parse '{_e}' to operation"),
        }
    }

    fn perform(&self, a: u8, b: u8) -> u8 {
        use Operation::*;
        match self {
            And => a & b,
            Or => a | b,
            Xor => a ^ b,
        }
    }
}

struct Wire {
    name: String,
    value: Option<u8>,
    from: Option<Rc<Box<Gate>>>,
    to: Vec<Rc<Box<Gate>>>,
}

struct Gate {
    operation: Operation,
    input_1: Rc<RefCell<Wire>>,
    input_2: Rc<RefCell<Wire>>,
    output: Rc<RefCell<Wire>>,
}

impl Gate {
    fn process(&self) -> bool {
        let mut output = (*self.output).borrow_mut();

        if output.value.is_some() {
            return false;
        }

        let value_1 = (*self.input_1).borrow().value;
        let value_2 = (*self.input_2).borrow().value;

        match (value_1, value_2) {
            (Some(value_1), Some(value_2)) => {
                let output_value = self.operation.perform(value_1, value_2);
                output.value = Some(output_value);
                true
            }
            _ => false,
        }
    }
}

fn create_logic_system(input: &str) -> (HashMap<String, Rc<RefCell<Wire>>>, Vec<Rc<Box<Gate>>>) {
    let contents = fs::read_to_string(input).unwrap();
    let mut lines = contents.lines();

    let mut wires: HashMap<String, Rc<RefCell<Wire>>> = HashMap::new();
    let mut gates: Vec<Rc<Box<Gate>>> = Vec::new();

    for line in lines.by_ref() {
        if line.len() == 0 {
            break;
        }

        let parts: Vec<&str> = line.split(": ").collect();
        let name = parts[0].to_string();
        let value: u8 = parts[1].parse().unwrap();

        wires.insert(
            name.clone(),
            Rc::new(RefCell::new(Wire {
                name: name.to_owned(),
                value: Some(value),
                from: None,
                to: Vec::new(),
            })),
        );
    }

    for line in lines {
        let parts: Vec<&str> = line.split(" ").collect();

        let input_1_name = parts[0].to_string();
        let operation = Operation::from_str(parts[1]);
        let input_2_name = parts[2].to_string();
        let output_name = parts[4].to_string();

        wires
            .entry(input_1_name.clone())
            .or_insert(Rc::new(RefCell::new(Wire {
                name: input_1_name.clone(),
                value: None,
                from: None,
                to: Vec::new(),
            })));

        wires
            .entry(input_2_name.clone())
            .or_insert(Rc::new(RefCell::new(Wire {
                name: input_2_name.clone(),
                value: None,
                from: None,
                to: Vec::new(),
            })));

        wires
            .entry(output_name.clone())
            .or_insert(Rc::new(RefCell::new(Wire {
                name: output_name.clone(),
                value: None,
                from: None,
                to: Vec::new(),
            })));

        let gate = Rc::new(Box::new(Gate {
            operation,
            input_1: wires[&input_1_name].clone(),
            input_2: wires[&input_2_name].clone(),
            output: wires[&output_name].clone(),
        }));

        (*wires[&input_1_name]).borrow_mut().to.push(gate.clone());
        (*wires[&input_2_name]).borrow_mut().to.push(gate.clone());
        (*wires[&output_name]).borrow_mut().from = Some(gate.clone());

        gates.push(gate);
    }

    (wires, gates)
}

fn run_system(gates: &Vec<Rc<Box<Gate>>>) {
    loop {
        let mut any_update = false;
        for gate in gates {
            let was_updated = gate.process();
            if was_updated {
                any_update = true;
            }
        }

        if !any_update {
            break;
        }
    }
}

fn task1() {
    let (wires, gates) = create_logic_system(INPUT);

    run_system(&gates);

    let mut wires: Vec<Rc<RefCell<Wire>>> = wires
        .values()
        .filter_map(|w| {
            if (**w).borrow().name.starts_with('z') {
                Some(w.clone())
            } else {
                None
            }
        })
        .collect();
    wires.sort_by_key(|w| (**w).borrow().name.clone());

    let mut output: usize = 0;

    for wire in wires.into_iter().rev() {
        let wire = (*wire).borrow();
        let value = wire.value.unwrap() as usize;

        output = (output << 1) | value;
    }

    println!("task1:\t{output}");
}

struct GateDesc {
    id: usize,
    inputs: Vec<String>,
    output: String,
    operation: Operation,
}

fn task2() {
    let contents = fs::read_to_string(INPUT).unwrap();
    let mut lines = contents.lines();

    let mut gates: Vec<GateDesc> = Vec::new();

    for line in lines.by_ref() {
        if line.len() == 0 {
            break;
        }
    }

    for (id, line) in lines.enumerate() {
        let parts: Vec<&str> = line.split(" ").collect();

        let input_1_name = parts[0].to_string();
        let operation = Operation::from_str(parts[1]);
        let input_2_name = parts[2].to_string();
        let output_name = parts[4].to_string();

        gates.push(GateDesc {
            id,
            operation,
            inputs: vec![input_1_name, input_2_name],
            output: output_name,
        });
    }

    // found by hand using system below
    let switched_pairs: Vec<(String, String)> = vec![
        ("z08".to_string(), "vvr".to_string()),
        ("bkr".to_string(), "rnq".to_string()),
        ("z28".to_string(), "tfb".to_string()),
        ("z39".to_string(), "mqh".to_string()),
    ];

    for (p1, p2) in &switched_pairs {
        let g1 = gates.iter_mut().find(|g| g.output == *p1).unwrap();
        g1.output = p2.clone();
        let g1_id = g1.id;
        let g2 = gates
            .iter_mut()
            .find(|g| g.id != g1_id && g.output == *p2)
            .unwrap();
        g2.output = p1.clone();
    }

    let mut carry_in_name: String;
    // find carry out from half adder in zeroth bit
    {
        let input_1_name = "x00".to_string();
        let input_2_name = "y00".to_string();
        let output_name = "z00".to_string();

        let Some(_xor_gate) = gates.iter().find(|g| {
            g.operation == Operation::Xor
                && g.inputs.contains(&input_1_name)
                && g.inputs.contains(&input_2_name)
                && g.output == output_name
        }) else {
            panic!("could not find xor gate for inputs {input_1_name} {input_2_name}");
        };

        let Some(and_gate) = gates.iter().find(|g| {
            g.operation == Operation::And
                && g.inputs.contains(&input_1_name)
                && g.inputs.contains(&input_2_name)
        }) else {
            panic!("could not find and gate for inputs {input_1_name} {input_2_name}");
        };

        carry_in_name = and_gate.output.clone();
    }
    // for rest of bits find their carry out and check for any mistake in the circuit
    for i in 1..45 {
        let input_1_name = format!("x{:0width$}", i, width = 2);
        let input_2_name = format!("y{:0width$}", i, width = 2);
        let output_name = format!("z{:0width$}", i, width = 2);

        let Some(xor_1_gate) = gates.iter().find(|g| {
            g.operation == Operation::Xor
                && g.inputs.contains(&input_1_name)
                && g.inputs.contains(&input_2_name)
        }) else {
            panic!("could not find xor gate for inputs {input_1_name} {input_2_name}");
        };

        let xor_1_output = &xor_1_gate.output;

        let Some(_xor_2_gate) = gates.iter().find(|g| {
            g.operation == Operation::Xor
                && g.inputs.contains(xor_1_output)
                && g.inputs.contains(&carry_in_name)
                && g.output == output_name
        }) else {
            panic!(
                "could not find xor gate for inputs {xor_1_output} {carry_in_name} at layer {i}"
            );
        };

        let Some(and_1_gate) = gates.iter().find(|g| {
            g.operation == Operation::And
                && g.inputs.contains(xor_1_output)
                && g.inputs.contains(&carry_in_name)
        }) else {
            panic!(
                "could not find and gate for inputs {xor_1_output} {carry_in_name} at layer {i}"
            );
        };

        let Some(and_2_gate) = gates.iter().find(|g| {
            g.operation == Operation::And
                && g.inputs.contains(&input_1_name)
                && g.inputs.contains(&input_2_name)
        }) else {
            panic!("could not find and gate for inputs {input_1_name} {input_2_name}");
        };

        let Some(or_gate) = gates.iter().find(|g| {
            g.operation == Operation::Or
                && g.inputs.contains(&and_1_gate.output)
                && g.inputs.contains(&and_2_gate.output)
        }) else {
            panic!(
                "could not find and gate for inputs {:} {:} at layer {i}",
                and_1_gate.output, and_2_gate.output
            );
        };

        carry_in_name = or_gate.output.clone();
    }

    if carry_in_name != "z45" {
        panic!("final carry out is not z45 but {carry_in_name}");
    }

    let mut output = switched_pairs
        .into_iter()
        .map(|(p1, p2)| vec![p1, p2])
        .flatten()
        .collect::<Vec<String>>();
    output.sort();

    let output = output.join(",");

    println!("task2:\t{output}");
}

fn main() {
    task1();
    task2();
}
