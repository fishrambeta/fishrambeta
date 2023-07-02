use crate::math::{Equation, Variable};
use std::collections::HashMap;

impl Equation {
    pub fn simplify(self) -> Self {
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

    pub fn get_factors(&self) -> Vec<Equation> {
        return Vec::new();
    }
}

fn simplify_addition(addition: Vec<Equation>) -> Equation {
    let mut terms: HashMap<Equation, i64> = HashMap::new();
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
    let mut terms: HashMap<Equation, i64> = HashMap::new();
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
    let mut terms: HashMap<Equation, i64> = HashMap::new();
    for equation in multiplication.iter() {
        let simplified = equation.clone().simplify();
        if simplified == Equation::Variable(Variable::Integer(0)) {
            return Equation::Variable(Variable::Integer(0));
        } else if simplified != Equation::Variable(Variable::Integer(1)) {
            if let Equation::Power(ref power) = simplified {
                if let Equation::Variable(variable) = &power.1 {
                    if let Variable::Integer(n) = variable {
                        terms.insert(power.0.clone(), *terms.get(&simplified).unwrap_or(&0) + n);
                        continue;
                    }
                }
            }
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

    let more_simplified_multiplication: Equation = Equation::Multiplication(
        simplified_multiplication
            .iter()
            .skip(1)
            .map(|x| x.clone())
            .collect::<Vec<_>>(),
    )
    .multiply_by(&simplified_multiplication[0]); //TODO this performance can be improved by
                                                 //omitting the clone but I don't know how yet

    return more_simplified_multiplication;
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
