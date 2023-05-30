use std::collections::HashMap;
use std::ptr::eq;

///Represents a generic math object
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum Equation {
    Variable(Variable),
    Addition(Vec<Equation>),
    Subtraction(Vec<Equation>),
    Multiplication(Vec<Equation>),
    Division(Box<(Equation, Equation)>),
    Power(Box<(Equation, Equation)>),
    Equals(Box<(Equation, Equation)>),
}
///Represents a single number
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum Variable {
    Integer(i32),
    Rational((i32, i32)),
    Constant(Constant),
    Letter(String),
    Vector(String),
}
///Mathematical constants
#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum Constant {
    PI,
    E,
}

impl Symbol for Equation {
    fn simplify(self) -> Self {
        match self {
            Equation::Variable(variable) => return Equation::Variable(variable),
            Equation::Addition(addition) => return simpilify_addition(addition),
            Equation::Subtraction(subtraction) => return Equation::Subtraction(subtraction),
            Equation::Multiplication(multiplication) => simplify_multiplication(multiplication),
            Equation::Division(division) => return Equation::Division(division),
            Equation::Power(power) => return simplify_power(power),
            Equation::Equals(equation) => {
                return Equation::Equals(Box::new((equation.0.simplify(), equation.1.simplify())))
            }
        }
    }

    fn calculate(self: &Self, values: &HashMap<Variable, f64>) -> f64 {
        match self {
            Equation::Variable(variable) => {
                match variable {
                    Variable::Integer(integer) => return *integer as f64,
                    _ => {}
                }
                return values[&variable];
            }
            Equation::Addition(addition) => {
                return addition.iter().map(|x| x.calculate(&values)).sum()
            }
            Equation::Subtraction(subtraction) => {
                let minus: f64 = subtraction
                    .iter()
                    .skip(1)
                    .map(|x| x.calculate(&values))
                    .sum();
                return subtraction[0].calculate(&values) - minus;
            } // TODO make this in one statement, but rust hates me
            Equation::Multiplication(multiplication) => {
                return multiplication
                    .iter()
                    .map(|x| x.calculate(&values))
                    .product()
            }
            Equation::Division(division) => {
                return division.0.calculate(&values) / division.1.calculate(&values)
            }
            Equation::Power(power) => {
                return power.0.calculate(&values).powf(power.1.calculate(&values))
            }
            Equation::Equals(_) => panic!("Cannot calculate equals"),
        }
    }
}
pub trait Symbol {
    fn simplify(self) -> Self;
    fn calculate(self: &Self, values: &HashMap<Variable, f64>) -> f64;
}

fn simpilify_addition(addition: Vec<Equation>) -> Equation {
    let mut terms: HashMap<Equation, i32> = HashMap::new();
    for equation in addition.iter() {
        let simplified = equation.clone().simplify();
        terms.insert(
            simplified.clone(),
            *terms.get(&simplified).unwrap_or(&0) + 1,
        );
    }
    let mut simplified_addition: Vec<Equation> = Vec::new();
    for (equation, count) in terms.iter() {
        if *count == 1 {
            simplified_addition.push(equation.clone())
        } else {
            simplified_addition.push(
                Equation::Multiplication(vec![
                    Equation::Variable(Variable::Integer(*count)),
                    equation.clone(),
                ])
                .simplify(),
            );
        }
    }

    if simplified_addition.len() == 1 {
        return simplified_addition[0].clone();
    }
    return Equation::Addition(simplified_addition);
}

fn simplify_multiplication(multiplication: Vec<Equation>) -> Equation {
    let mut terms: HashMap<Equation, i32> = HashMap::new();
    for equation in multiplication.iter() {
        let simplified = equation.clone().simplify();
        terms.insert(
            simplified.clone(),
            *terms.get(&simplified).unwrap_or(&0) + 1,
        );
    }

    let mut simplified_multiplication: Vec<Equation> = Vec::new();
    for (equation, count) in terms.iter() {
        if *count == 1 {
            simplified_multiplication.push(equation.clone())
        } else {
            simplified_multiplication.push(
                Equation::Power(Box::new((
                    equation.clone(),
                    Equation::Variable(Variable::Integer(*count)),
                )))
                .simplify(),
            );
        }
    }

    if simplified_multiplication.len() == 1 {
        return simplified_multiplication[0].clone();
    }
    return Equation::Multiplication(simplified_multiplication);
}

fn simplify_power(power: Box<(Equation, Equation)>) -> Equation {
    let base = power.0.simplify();
    let exponent = power.1.simplify();

    match base.clone() {
        Equation::Multiplication(terms) => {
            let mut simplified_power: Vec<Equation> = vec![];
            for term in terms.iter() {
                simplified_power.push(Equation::Power(Box::new((term.clone(), exponent.clone()))));
            }
            return Equation::Multiplication(simplified_power);
        }
        _ => {}
    }

    return Equation::Power(Box::new((base, exponent)));
}
