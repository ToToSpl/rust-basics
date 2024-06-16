use indicatif::ProgressIterator;

mod token;
use token::Token;

#[derive(Debug, Clone)]
struct Number {
    value: u32,
    tokens: Vec<Token>,
}

impl Number {
    fn new(value: u32) -> Number {
        Number {
            value,
            tokens: value
                .to_string()
                .chars()
                .map(|c| Token::from_value(c.to_digit(10).unwrap()).unwrap())
                .collect(),
        }
    }

    fn stick_value(&self) -> u32 {
        self.tokens.iter().map(|t| t.stick_value()).sum()
    }
}

#[derive(Debug, Clone)]
struct Addition {
    left: Box<Expression>,
    right: Box<Expression>,
}

impl Addition {
    fn stick_value(&self) -> u32 {
        self.left.stick_value() + self.right.stick_value() + Token::Add.stick_value()
    }

    fn value(&self) -> u32 {
        self.left.value() + self.right.value()
    }

    fn display(&self) -> String {
        let mut output = self.left.display().to_owned();
        output.push_str(" + ");
        output.push_str(&self.right.display());
        output
    }

    fn optimize(&mut self, best_arr: &BestArr) {
        let optimize_left = match *self.left {
            Expression::Number(ref n) => {
                Some(best_arr[n.value as usize].addition_expression.clone())
            }
            Expression::Addition(ref mut n) => {
                n.optimize(best_arr);
                None
            }
            Expression::Multiplication(ref mut n) => {
                n.optimize(best_arr);
                None
            }
        };

        if let Some(left) = optimize_left {
            self.left = Box::new(left);
        }

        let optimize_right = match *self.right {
            Expression::Number(ref n) => {
                Some(best_arr[n.value as usize].addition_expression.clone())
            }
            Expression::Addition(ref mut n) => {
                n.optimize(best_arr);
                None
            }
            Expression::Multiplication(ref mut n) => {
                n.optimize(best_arr);
                None
            }
        };

        if let Some(right) = optimize_right {
            self.right = Box::new(right);
        }
    }
}

#[derive(Debug, Clone)]
struct Multiplication {
    left: Box<Expression>,
    right: Box<Expression>,
}

impl Multiplication {
    fn stick_value(&self) -> u32 {
        self.left.stick_value() + self.right.stick_value() + Token::Multiply.stick_value()
    }

    fn value(&self) -> u32 {
        self.left.value() * self.right.value()
    }

    fn display(&self) -> String {
        let mut output = self.left.display().to_owned();
        output.push_str(" * ");
        output.push_str(&self.right.display());
        output
    }

    fn optimize(&mut self, best_arr: &BestArr) {
        let optimize_left = match *self.left {
            Expression::Number(ref n) => {
                Some(best_arr[n.value as usize].multiplication_expression.clone())
            }
            Expression::Addition(ref mut n) => {
                n.optimize(best_arr);
                None
            }
            Expression::Multiplication(ref mut n) => {
                n.optimize(best_arr);
                None
            }
        };

        if let Some(left) = optimize_left {
            self.left = Box::new(left);
        }

        let optimize_right = match *self.right {
            Expression::Number(ref n) => {
                Some(best_arr[n.value as usize].multiplication_expression.clone())
            }
            Expression::Addition(ref mut n) => {
                n.optimize(best_arr);
                None
            }
            Expression::Multiplication(ref mut n) => {
                n.optimize(best_arr);
                None
            }
        };

        if let Some(right) = optimize_right {
            self.right = Box::new(right);
        }
    }
}

#[derive(Debug, Clone)]
enum Expression {
    Number(Number),
    Addition(Addition),
    Multiplication(Multiplication),
}

impl Expression {
    fn stick_value(&self) -> u32 {
        match &self {
            Expression::Number(n) => n.stick_value(),
            Expression::Addition(n) => n.stick_value(),
            Expression::Multiplication(n) => n.stick_value(),
        }
    }

    fn value(&self) -> u32 {
        match &self {
            Expression::Number(n) => n.value,
            Expression::Addition(n) => n.value(),
            Expression::Multiplication(n) => n.value(),
        }
    }

    fn display(&self) -> String {
        match &self {
            Expression::Number(n) => n.value.to_string(),
            Expression::Addition(n) => n.display(),
            Expression::Multiplication(n) => n.display(),
        }
    }

    fn optimize(&mut self, best_arr: &BestArr) {
        match self {
            Expression::Number(_) => {}
            Expression::Addition(ref mut n) => n.optimize(best_arr),
            Expression::Multiplication(ref mut n) => n.optimize(best_arr),
        };
    }
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

    for factor in factors {
        res.push(Expression::Multiplication(Multiplication {
            left: Box::new(Expression::Number(Number::new(factor.0))),
            right: Box::new(Expression::Number(Number::new(factor.1))),
        }))
    }

    for c in 1..n {
        let multiplication = n - c;
        let factors = binary_factorization(multiplication);

        for factor in factors {
            res.push(Expression::Addition(Addition {
                left: Box::new(Expression::Number(Number::new(c))),
                right: Box::new(Expression::Multiplication(Multiplication {
                    left: Box::new(Expression::Number(Number::new(factor.0))),
                    right: Box::new(Expression::Number(Number::new(factor.1))),
                })),
            }))
        }
    }

    res.push(Expression::Number(Number::new(n)));

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
            addition_expression: Expression::Number(Number::new(0)),
            multiplication_expression: Expression::Number(Number::new(0)),
        },
        BestNumber {
            addition_expression: Expression::Number(Number::new(1)),
            multiplication_expression: Expression::Number(Number::new(1)),
        },
    ];

    let max_number = 100;

    for n in (2..max_number + 1).progress() {
        let mut combinations = expand_number(n);
        combinations.iter_mut().for_each(|c| c.optimize(&best_arr));

        let addition_expression = combinations
            .iter()
            .min_by(|x, y| x.stick_value().cmp(&y.stick_value()))
            .unwrap();

        let multiplication_expression = combinations
            .iter()
            .filter(|e| match e {
                Expression::Number(_) => true,
                Expression::Multiplication(_) => true,
                Expression::Addition(_) => false,
            })
            .min_by(|x, y| x.stick_value().cmp(&y.stick_value()))
            .unwrap();

        best_arr.push(BestNumber {
            addition_expression: addition_expression.clone(),
            multiplication_expression: multiplication_expression.clone(),
        });

        println!(
            "{:?} = {:?}\t\t{:?}",
            n,
            addition_expression.display(),
            addition_expression.stick_value()
        );
    }

    let sum_t = best_arr
        .iter()
        .skip(1)
        .map(|b| b.addition_expression.stick_value())
        .sum::<u32>();

    println!("-------------");
    println!("{:?}", sum_t);
}
