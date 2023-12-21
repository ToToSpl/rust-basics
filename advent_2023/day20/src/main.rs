use std::collections::{HashMap, VecDeque};
use std::fs;

const INPUT: &str = "input.txt";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Pulse {
    Low,
    High,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Signal {
    from: String,
    to: String,
    pulse: Pulse,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcaster,
}

#[derive(Clone, Debug)]
struct Module {
    module_type: ModuleType,
    name: String,
    state: HashMap<String, Pulse>,
    input_stack: Vec<Signal>,
    output_modules: Vec<String>,
}

impl Module {
    fn new(line: &str) -> Module {
        let parts = line.split(" -> ").collect::<Vec<_>>();
        let mut first_part = parts[0].chars();
        let (module_type, name) = match first_part.next().unwrap() {
            'b' => (ModuleType::Broadcaster, "broadcaster".to_string()),
            '%' => (ModuleType::FlipFlop, first_part.as_str().to_string()),
            '&' => (ModuleType::Conjunction, first_part.as_str().to_string()),
            _e => panic!("Unknown moduleType {:?}", _e),
        };

        let output_modules = parts[1]
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        Module {
            module_type,
            name,
            state: HashMap::new(),
            input_stack: Vec::new(),
            output_modules,
        }
    }

    fn update_state(&mut self, modules: &HashMap<String, Module>) {
        match self.module_type {
            ModuleType::FlipFlop => {
                self.state.insert("internal".to_string(), Pulse::Low);
            }
            ModuleType::Broadcaster => {}
            ModuleType::Conjunction => {
                for (name, module) in modules {
                    if *name == self.name {
                        continue;
                    }
                    if module.output_modules.contains(&self.name) {
                        self.state.insert(name.clone(), Pulse::Low);
                    }
                }
            }
        }
    }

    fn update_step(&mut self) -> Vec<Signal> {
        if self.input_stack.len() == 0 {
            return vec![];
        }
        let input_signal = self.input_stack.pop().unwrap();
        match self.module_type {
            ModuleType::Broadcaster => self
                .output_modules
                .iter()
                .map(|m| Signal {
                    from: self.name.clone(),
                    to: m.clone(),
                    pulse: input_signal.pulse,
                })
                .collect::<Vec<_>>(),

            ModuleType::FlipFlop => {
                if input_signal.pulse == Pulse::High {
                    return vec![];
                }
                let new_state = if self.state["internal"] == Pulse::Low {
                    Pulse::High
                } else {
                    Pulse::Low
                };
                self.state.insert("internal".to_string(), new_state);
                self.output_modules
                    .iter()
                    .map(|m| Signal {
                        from: self.name.clone(),
                        to: m.clone(),
                        pulse: new_state,
                    })
                    .collect::<Vec<_>>()
            }

            ModuleType::Conjunction => {
                self.state.insert(input_signal.from, input_signal.pulse);
                let pulse = if self.state.iter().filter(|&(_, &p)| p == Pulse::Low).count() == 0 {
                    Pulse::Low
                } else {
                    Pulse::High
                };

                self.output_modules
                    .iter()
                    .map(|m| Signal {
                        from: self.name.clone(),
                        to: m.clone(),
                        pulse,
                    })
                    .collect::<Vec<_>>()
            }
        }
    }
}

fn task1() {
    let contents = fs::read_to_string(INPUT).unwrap();
    let modules = contents.lines().map(Module::new).collect::<Vec<_>>();
    let mut modules: HashMap<String, Module> =
        HashMap::from_iter(modules.into_iter().map(|m| (m.name.clone(), m)));
    let modules_to_compare = modules.clone();

    for (_name, module) in &mut modules {
        module.update_state(&modules_to_compare);
    }

    let start_event = Signal {
        from: String::from("button "),
        to: String::from("broadcaster"),
        pulse: Pulse::Low,
    };

    let mut high_signal_count: usize = 0;
    let mut low_signal_count: usize = 0;

    for _ in 0..1000 {
        let mut event_stack = VecDeque::from([start_event.clone()]);
        while let Some(event) = event_stack.pop_front() {
            // println!("{:} -{:?}-> {:}", event.from, event.pulse, event.to);
            if event.pulse == Pulse::High {
                high_signal_count += 1;
            } else {
                low_signal_count += 1;
            }

            if let Some(module) = modules.get_mut(&event.to) {
                module.input_stack.push(event);
            }

            for (_, module) in &mut modules {
                let events = module.update_step();
                for event in events {
                    event_stack.push_back(event);
                }
            }
        }
        // println!("");
    }

    println!("\nHigh: {:?}", high_signal_count);
    println!("Low: {:?}\n", low_signal_count);

    println!("task1 {:?}", high_signal_count * low_signal_count);
}

fn task2() {
    // task2 is easier to be done on paper/hardcode some stuff than in code ;_;
    // input to this task is designed as such, that brute force will take ages and yet brute force
    // is the only solution for general input.
    let contents = fs::read_to_string(INPUT).unwrap();
    let modules = contents.lines().map(Module::new).collect::<Vec<_>>();
    let mut modules: HashMap<String, Module> =
        HashMap::from_iter(modules.into_iter().map(|m| (m.name.clone(), m)));
    let modules_to_compare = modules.clone();

    for (_name, module) in &mut modules {
        module.update_state(&modules_to_compare);
    }

    let start_event = Signal {
        from: String::from("button"),
        to: String::from("broadcaster"),
        pulse: Pulse::Low,
    };

    let mut counter_triggers: HashMap<String, usize> = HashMap::from([
        (String::from("sr"), 0),
        (String::from("sn"), 0),
        (String::from("rf"), 0),
        (String::from("vq"), 0),
    ]);

    let destination = String::from("hp");

    let mut count = 0;
    loop {
        let mut event_stack = VecDeque::from([start_event.clone()]);
        count += 1;
        while let Some(event) = event_stack.pop_front() {
            if event.to == destination {
                if event.pulse == Pulse::High {
                    counter_triggers.insert(event.from.clone(), count);
                }
            }

            if let Some(module) = modules.get_mut(&event.to) {
                module.input_stack.push(event);
            }

            for (_, module) in &mut modules {
                let events = module.update_step();
                for event in events {
                    event_stack.push_back(event);
                }
            }
        }
        if counter_triggers.iter().filter(|&(_, &c)| c == 0).count() == 0 {
            break;
        }
    }

    println!("{:?}", counter_triggers);
    println!(
        "task2 {:?}",
        counter_triggers.iter().fold(1, |acc, (_, c)| acc * c)
    )
}

fn main() {
    task1();
    task2();
}
