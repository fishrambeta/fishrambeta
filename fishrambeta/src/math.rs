use std::collections::HashMap;

///Represents a generic math object
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum Equation {
    Variable(Variable),
    Addition(Vec<Equation>),
    Subtraction(Vec<Equation>),
    Multiplication(Vec<Equation>),
    Division(Box<(Equation, Equation)>),
    Power(Box<(Equation, Equation)>),
    Ln(Box<Equation>),
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
            Equation::Addition(addition) => return simplify_addition(addition),
            Equation::Subtraction(subtraction) => return simplify_subtraction(subtraction),
            Equation::Multiplication(multiplication) => simplify_multiplication(multiplication),
            Equation::Division(division) => {
                let numerator = division.0.simplify();
                let denominator = division.1.simplify();
                return if numerator == Equation::Variable(Variable::Integer(0)) {
                    Equation::Variable(Variable::Integer(0))
                } else {
                    Equation::Division(Box::new((numerator, denominator)))
                };
            }
            Equation::Power(power) => return simplify_power(power),
            Equation::Ln(ln) => return Equation::Ln(Box::new(ln.simplify())),
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
                    Variable::Rational(rational) => {
                        return (rational.0 as f64) / (rational.1 as f64)
                    }
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
            Equation::Ln(ln) => return ln.calculate(values).ln(),
            Equation::Equals(_) => panic!("Cannot calculate equals"),
        }
    }

    fn differentiate(self: &Equation, differentiate_to: &Variable) -> Equation {
        match self {
            Equation::Variable(variable) => {
                return if variable == differentiate_to {
                    Equation::Variable(Variable::Integer(1))
                } else {
                    Equation::Variable(Variable::Integer(0))
                }
            }
            Equation::Addition(addition) => {
                return Equation::Addition(
                    addition
                        .iter()
                        .map(|x| x.differentiate(&differentiate_to))
                        .collect::<Vec<_>>(),
                )
            }
            Equation::Subtraction(subtraction) => {
                return Equation::Addition(
                    subtraction
                        .iter()
                        .map(|x| x.differentiate(&differentiate_to))
                        .collect::<Vec<_>>(),
                )
            }
            Equation::Multiplication(multiplication) => {
                return Equation::Addition(
                    multiplication
                        .iter()
                        .map(|x| {
                            let mut multiplication_new: Vec<Equation> = multiplication.clone();
                            multiplication_new.remove(
                                multiplication_new
                                    .iter()
                                    .position(|y| y == x)
                                    .expect("This shouldn't happen"),
                            );
                            multiplication_new.push(x.differentiate(differentiate_to));
                            Equation::Multiplication(multiplication_new)
                        })
                        .collect::<Vec<_>>(),
                )
            }
            Equation::Division(division) => {
                let numerator = Equation::Subtraction(vec![
                    Equation::Multiplication(vec![
                        division.1.clone(),
                        division.0.differentiate(differentiate_to),
                    ]),
                    Equation::Multiplication(vec![
                        division.0.clone(),
                        division.1.differentiate(differentiate_to),
                    ]),
                ]);
                let denominator = Equation::Power(Box::new((
                    division.1.clone(),
                    Equation::Variable(Variable::Integer(2)),
                )));
                return Equation::Division(Box::new((numerator, denominator)));
            }
            Equation::Power(power) => return differentiate_power(power, differentiate_to),
            _ => {
                todo!()
            }
        }
    }
}
pub trait Symbol {
    fn simplify(self) -> Self;
    fn calculate(self: &Self, values: &HashMap<Variable, f64>) -> f64;
    fn differentiate(self: &Self, differentiate_to: &Variable) -> Self;
}

fn simplify_addition(addition: Vec<Equation>) -> Equation {
    let mut terms: HashMap<Equation, i32> = HashMap::new();
    for equation in addition.iter() {
        let simplified = equation.clone().simplify();
        if simplified != Equation::Variable(Variable::Integer(0)) {
            terms.insert(
                simplified.clone(),
                *terms.get(&simplified).unwrap_or(&0) + 1,
            );
        }
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

fn simplify_subtraction(subtraction: Vec<Equation>) -> Equation {
    let mut terms: HashMap<Equation, i32> = HashMap::new();
    let first_term = subtraction[0].clone().simplify();
    let mut has_matched_first_term = false;
    for equation in subtraction.iter().skip(1) {
        let simplified = equation.clone().simplify();
        if !has_matched_first_term && simplified == first_term {
            has_matched_first_term = true;
        } else if simplified != Equation::Variable(Variable::Integer(0)) {
            terms.insert(
                simplified.clone(),
                *terms.get(&simplified).unwrap_or(&0) + 1,
            );
        }
    }
    let mut simplified_subtraction: Vec<Equation> = Vec::new();
    if !has_matched_first_term {
        simplified_subtraction.push(first_term.clone());
    }
    for (equation, count) in terms.iter() {
        if *count == 1 {
            simplified_subtraction.push(equation.clone())
        } else {
            simplified_subtraction.push(
                Equation::Multiplication(vec![
                    Equation::Variable(Variable::Integer(*count)),
                    equation.clone(),
                ])
                .simplify(),
            );
        }
    }

    if simplified_subtraction.len() == 1 {
        return simplified_subtraction[0].clone();
    } else if simplified_subtraction.len() == 0 {
        return Equation::Variable(Variable::Integer(0));
    }
    return Equation::Subtraction(simplified_subtraction);
}

fn simplify_multiplication(multiplication: Vec<Equation>) -> Equation {
    let mut terms: HashMap<Equation, i32> = HashMap::new();
    for equation in multiplication.iter() {
        let simplified = equation.clone().simplify();
        if simplified == Equation::Variable(Variable::Integer(0)) {
            return Equation::Variable(Variable::Integer(0));
        } else if simplified != Equation::Variable(Variable::Integer(1)) {
            terms.insert(
                simplified.clone(),
                *terms.get(&simplified).unwrap_or(&0) + 1,
            );
        }
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

fn differentiate_power(power: &Box<(Equation, Equation)>, differentiate_to: &Variable) -> Equation {
    let first_term = Equation::Power(Box::new((
        power.0.clone(),
        Equation::Subtraction(vec![
            power.0.clone(),
            Equation::Variable(Variable::Integer(1)),
        ]),
    )));
    let g_f_accent = Equation::Multiplication(vec![
        power.0.clone(),
        power.1.differentiate(differentiate_to),
    ]);
    let f_log_g_accent = Equation::Multiplication(vec![
        power.0.clone(),
        Equation::Ln(Box::new(power.0.clone())),
        power.1.differentiate(differentiate_to),
    ]);
    let second_term = Equation::Addition(vec![g_f_accent, f_log_g_accent]);
    return Equation::Multiplication(vec![first_term, second_term]);
}
