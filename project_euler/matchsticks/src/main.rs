use indicatif::ProgressIterator;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Multiply,
}

impl Operation {
    fn stick_value() -> u32 {
        2
    }

    fn display(&self) -> String {
        match self {
            Operation::Add => String::from(" + "),
            Operation::Multiply => String::from(" * "),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Number {
    value: u32,
    stick_value: u32,
}

impl Number {
    fn new(value: u32) -> Number {
        let stick_value = value
            .to_string()
            .chars()
            .map(|c| match c {
                '0' => 6,
                '1' => 2,
                '2' => 5,
                '3' => 5,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 3,
                '8' => 7,
                '9' => 6,
                _ => panic!("unexpected char {c}"),
            })
            .sum();

        Number { value, stick_value }
    }
}

#[derive(Debug, Clone, Copy)]
enum Token {
    Operation(Operation),
    Number(Number),
}

impl Token {
    fn stick_value(&self) -> u32 {
        match self {
            Token::Number(n) => n.stick_value,
            Token::Operation(_) => Operation::stick_value(),
        }
    }
}

#[derive(Debug, Clone)]
struct Expression {
    tokens: Vec<Token>,
}

impl Expression {
    fn new(tokens: Vec<Token>) -> Expression {
        Expression { tokens }
    }

    fn stick_value(&self) -> u32 {
        self.tokens.iter().map(|t| t.stick_value()).sum()
    }

    fn display(&self) -> String {
        self.tokens
            .iter()
            .map(|t| match t {
                Token::Operation(o) => o.display(),
                Token::Number(n) => n.value.to_string(),
            })
            .collect::<Vec<String>>()
            .join("")
    }

    fn is_multiplication_replaceable(&self) -> bool {
        if self
            .tokens
            .iter()
            .find(|t| match t {
                Token::Number(_) => false,
                Token::Operation(o) => match o {
                    Operation::Add => true,
                    Operation::Multiply => false,
                },
            })
            .is_some()
        {
            false
        } else {
            true
        }
    }

    fn optimize(&mut self, bestArr: &BestArr) {}
}

/// returns vec with possible positive multiplications allowing to obtain number (ignoring 1 * n)
fn binary_factorization(n: u32) -> Vec<(u32, u32)> {
    let mut res = vec![];

    let root = f32::sqrt(n as f32).floor() as u32 + 1;
    for i in 2..root {
        if n % i != 0 {
            continue;
        }

        res.push((i, n / i))
    }

    res
}

/// expands number to form a * b + c in form of expression. returns all possible combination for
/// positive a, b and c
fn expand_number(n: u32) -> Vec<Expression> {
    let mut res = vec![];

    let factors = binary_factorization(n);

    for (a, b) in factors {
        res.push(Expression::new(vec![
            Token::Number(Number::new(a)),
            Token::Operation(Operation::Multiply),
            Token::Number(Number::new(b)),
        ]));
    }

    for c in 1..n {
        let multiplication = n - c;
        let factors = binary_factorization(multiplication);

        for (a, b) in factors {
            res.push(Expression::new(vec![
                Token::Number(Number::new(a)),
                Token::Operation(Operation::Multiply),
                Token::Number(Number::new(b)),
                Token::Operation(Operation::Add),
                Token::Number(Number::new(c)),
            ]));
        }
    }

    res.push(Expression::new(vec![Token::Number(Number::new(n))]));

    res
}

type BestArr = Vec<BestNumber>;

struct BestNumber {
    addition_expression: Expression,
    multiplication_expression: Expression,
}

fn main() {
    let mut best_arr: BestArr = vec![
        BestNumber {
            addition_expression: Expression::new(vec![Token::Number(Number::new(0))]),
            multiplication_expression: Expression::new(vec![Token::Number(Number::new(0))]),
        },
        BestNumber {
            addition_expression: Expression::new(vec![Token::Number(Number::new(1))]),
            multiplication_expression: Expression::new(vec![Token::Number(Number::new(1))]),
        },
    ];

    let max_number = 28;

    // for n in (2..max_number + 1).progress() {
    for n in 2..max_number + 1 {
        let mut combinations = expand_number(n);

        combinations.iter_mut().for_each(|c| c.optimize(&best_arr));

        let addition_expression = combinations
            .iter()
            .min_by(|x, y| x.stick_value().cmp(&y.stick_value()))
            .unwrap();

        println!(
            "{:?} = {:?}\t\t{:?}",
            n,
            addition_expression.display(),
            addition_expression.stick_value()
        );

        best_arr.push(BestNumber {
            addition_expression: addition_expression.to_owned(),
            multiplication_expression: combinations
                .iter()
                .filter(|e| e.is_multiplication_replaceable())
                .min_by(|x, y| x.stick_value().cmp(&y.stick_value()))
                .unwrap()
                .to_owned(),
        });
    }

    let sum_t = best_arr
        .iter()
        .skip(1)
        .map(|b| b.addition_expression.stick_value())
        .sum::<u32>();

    println!("-------------");
    println!("{:?}", sum_t);
}
