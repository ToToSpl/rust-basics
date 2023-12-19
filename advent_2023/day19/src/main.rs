use std::collections::HashMap;
use std::fs;

const INPUT: &str = "input.txt";

#[derive(Clone, Copy, Debug)]
enum Condition {
    Smaller,
    Bigger,
}

#[derive(Clone, Debug)]
enum Action {
    Accept,
    Reject,
    Redirect(String),
}

#[derive(Clone, Debug)]
struct Rule {
    operator: char,
    condition: Condition,
    value: usize,
    action: Action,
}

impl Rule {
    fn new(rule_raw: &str) -> Rule {
        let mut chars = rule_raw.chars();
        let operator = chars.next().unwrap();
        let condition = match chars.next().unwrap() {
            '<' => Condition::Smaller,
            '>' => Condition::Bigger,
            _e => panic!("Unknown charactcer for condition {:?}", _e),
        };
        let rest = chars.as_str().split(':').collect::<Vec<_>>();
        let value = rest[0].parse::<usize>().unwrap();
        let action = match rest[1] {
            "A" => Action::Accept,
            "R" => Action::Reject,
            r => Action::Redirect(r.to_string()),
        };

        Rule {
            operator,
            condition,
            value,
            action,
        }
    }

    fn process(&self, part: &Part) -> Option<Action> {
        let part_val = match self.operator {
            'x' => part.x,
            'm' => part.m,
            'a' => part.a,
            's' => part.s,
            _e => panic!("Unknown operator! {:?}", _e),
        };

        let cmp_res = match self.condition {
            Condition::Smaller => part_val < self.value,
            Condition::Bigger => part_val > self.value,
        };

        if cmp_res {
            Some(self.action.clone())
        } else {
            None
        }
    }

    fn process_range(&self, part: &PartRange) -> Vec<(Option<Action>, PartRange)> {
        let part_val = match self.operator {
            'x' => part.x,
            'm' => part.m,
            'a' => part.a,
            's' => part.s,
            _e => panic!("Unknown operator! {:?}", _e),
        };

        #[derive(Eq, PartialEq, Clone, Copy)]
        enum RangeType {
            Pass((usize, usize)),
            Fail((usize, usize)),
            None,
        }
        use RangeType::*;

        let splitted_range = match self.condition {
            Condition::Smaller => {
                if part_val.1 < self.value {
                    [Pass(part_val), None]
                } else if part_val.0 >= self.value {
                    [Fail(part_val), None]
                } else {
                    [
                        Pass((part_val.0, self.value - 1)),
                        Fail((self.value, part_val.1)),
                    ]
                }
            }
            Condition::Bigger => {
                if part_val.0 > self.value {
                    [Pass(part_val), None]
                } else if part_val.1 <= self.value {
                    [Fail(part_val), None]
                } else {
                    [
                        Fail((part_val.0, self.value)),
                        Pass((self.value + 1, part_val.1)),
                    ]
                }
            }
        };

        splitted_range
            .into_iter()
            .filter(|&r| r != None)
            .map(|r| {
                let val = match r {
                    Pass(v) => v,
                    Fail(v) => v,
                    None => unreachable!(),
                };

                let new_range = match self.operator {
                    'x' => PartRange {
                        x: val,
                        m: part.m,
                        a: part.a,
                        s: part.s,
                    },
                    'm' => PartRange {
                        x: part.x,
                        m: val,
                        a: part.a,
                        s: part.s,
                    },
                    'a' => PartRange {
                        x: part.x,
                        m: part.m,
                        a: val,
                        s: part.s,
                    },
                    's' => PartRange {
                        x: part.x,
                        m: part.m,
                        a: part.a,
                        s: val,
                    },
                    _ => unreachable!(),
                };
                let action = match r {
                    Pass(_) => Some(self.action.clone()),
                    Fail(_) => Option::None,
                    None => unreachable!(),
                };

                (action, new_range)
            })
            .collect::<Vec<_>>()
    }
}

#[derive(Clone, Debug)]
struct WorkFlow {
    name: String,
    rules: Vec<Rule>,
    end: Action,
}

impl WorkFlow {
    fn new(line: &str) -> WorkFlow {
        let splitted = line.split('{').collect::<Vec<_>>();
        let name = splitted[0].to_string();
        let mut rest = {
            let mut r = splitted[1].chars();
            r.next_back();
            r.as_str()
        }
        .split(',');

        let end = match rest.next_back().unwrap() {
            "A" => Action::Accept,
            "R" => Action::Reject,
            r => Action::Redirect(r.to_string()),
        };
        let rules = rest.map(Rule::new).collect::<Vec<_>>();

        WorkFlow { name, rules, end }
    }

    fn process(&self, part: &Part) -> Action {
        for rule in &self.rules {
            if let Some(action) = rule.process(part) {
                return action.clone();
            }
        }
        self.end.clone()
    }

    fn process_range(&self, part: &PartRange) -> Vec<(Action, PartRange)> {
        let mut ranges = vec![*part];
        let mut processed = Vec::new();
        for rule in &self.rules {
            let mut next_ranges = Vec::new();
            for range in &ranges {
                let new_ranges = rule.process_range(range);
                for new_range in new_ranges {
                    if new_range.0.is_some() {
                        processed.push((new_range.0.unwrap(), new_range.1));
                    } else {
                        next_ranges.push(new_range.1);
                    }
                }
            }
            ranges = next_ranges;
        }

        for range in ranges {
            processed.push((self.end.clone(), range));
        }

        processed
    }
}

#[derive(Clone, Copy, Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn new(line: &str) -> Part {
        // aint gonna use regex (stinky)
        let mut chars = line.chars();
        chars.next();
        chars.next_back();
        let vals_raw = chars.as_str().split(',').collect::<Vec<_>>();
        let x = vals_raw[0].to_string()[2..].parse::<usize>().unwrap();
        let m = vals_raw[1].to_string()[2..].parse::<usize>().unwrap();
        let a = vals_raw[2].to_string()[2..].parse::<usize>().unwrap();
        let s = vals_raw[3].to_string()[2..].parse::<usize>().unwrap();

        Part { x, m, a, s }
    }
}

#[derive(Clone, Copy, Debug)]
struct PartRange {
    x: (usize, usize),
    m: (usize, usize),
    a: (usize, usize),
    s: (usize, usize),
}

fn task1() {
    let contents = fs::read_to_string(INPUT).unwrap();
    let lines = contents.lines();

    let (workflows, parts) = {
        let mut workflows: HashMap<String, WorkFlow> = HashMap::new();
        let mut parts = Vec::new();
        {
            let mut line_breaked = false;
            for line in lines {
                if line.len() == 0 {
                    line_breaked = true;
                    continue;
                }
                if line_breaked == false {
                    let workflow = WorkFlow::new(line);
                    workflows.insert(workflow.name.clone(), workflow);
                } else {
                    parts.push(Part::new(line));
                }
            }
        };
        (workflows, parts)
    };

    let mut accepted: Vec<Part> = Vec::new();
    let mut rejected: Vec<Part> = Vec::new();
    for part in &parts {
        let mut result = workflows[&"in".to_string()].process(part);
        while let Action::Redirect(name) = result {
            result = workflows[&name].process(part);
        }
        match result {
            Action::Accept => accepted.push(*part),
            Action::Reject => rejected.push(*part),
            Action::Redirect(_) => panic!("Redirect after while loop"),
        }
    }

    println!(
        "task1 {:?}",
        accepted
            .iter()
            .map(|p| p.x + p.m + p.a + p.s)
            .sum::<usize>()
    );
}

fn task2() {
    let contents = fs::read_to_string(INPUT).unwrap();
    let lines = contents.lines();

    let workflows = {
        let mut workflows: HashMap<String, WorkFlow> = HashMap::new();
        {
            for line in lines {
                if line.len() == 0 {
                    break;
                }
                let workflow = WorkFlow::new(line);
                workflows.insert(workflow.name.clone(), workflow);
            }
        };
        workflows
    };

    let mut accepted: usize = 0;
    let mut stack = vec![(
        "in".to_string(),
        PartRange {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        },
    )];

    while let Some((workflow_name, range)) = stack.pop() {
        let new_ranges = workflows[&workflow_name].process_range(&range);
        for new_range in new_ranges {
            match new_range.0 {
                Action::Redirect(r) => stack.push((r, new_range.1)),
                Action::Accept => {
                    let x = new_range.1.x.1 - new_range.1.x.0 + 1;
                    let m = new_range.1.m.1 - new_range.1.m.0 + 1;
                    let a = new_range.1.a.1 - new_range.1.a.0 + 1;
                    let s = new_range.1.s.1 - new_range.1.s.0 + 1;
                    accepted += x * m * a * s;
                }
                Action::Reject => {}
            }
        }
    }

    println!("task2 {:?}", accepted);
}

fn main() {
    task1();
    task2();
}
