use super::{Equation, EquationBTreeMap, Variable};
use crate::math::steps::StepLogger;
use num_rational::Rational64;

fn flatten_multiplication(multiplication: Vec<Equation>) -> Vec<Equation> {
    let mut new_mult = vec![];
    for term in multiplication {
        match term {
            Equation::Multiplication(m) => {
                new_mult.append(&mut flatten_multiplication(m));
            }
            other => new_mult.push(other),
        };
    }
    new_mult
}

fn distribute_terms(multiplication: &[Equation], addition: Vec<Equation>) -> Equation {
    let mut new_addition = Vec::new();
    for addition_term in addition {
        let mut new_multiplication = multiplication.to_owned();
        new_multiplication.push(addition_term);
        new_addition.push(Equation::Multiplication(new_multiplication));
    }
    Equation::Addition(new_addition)
}

pub(super) fn simplify_multiplication(
    multiplication: Vec<Equation>,
    step_logger: &mut Option<StepLogger>,
) -> Equation {
    let mut multiplication = flatten_multiplication(multiplication);
    let mut terms: EquationBTreeMap = EquationBTreeMap::new();
    let mut total_rational_factor: Rational64 = 1.into();

    let mut total_is_negative = false;
    for (index, equation) in multiplication.iter().enumerate() {
        let (term, count) = match equation.clone().simplify(step_logger) {
            Equation::Variable(Variable::Integer(0)) => {
                return Equation::Variable(Variable::Integer(0));
            }
            Equation::Variable(Variable::Integer(1)) => {
                continue;
            }
            Equation::Variable(Variable::Integer(n)) => {
                total_rational_factor *= n;
                continue;
            }
            Equation::Variable(Variable::Rational(r)) => {
                total_rational_factor *= r;
                continue;
            }
            Equation::Negative(negative) => {
                total_is_negative = !total_is_negative;
                (*negative, Equation::Variable(Variable::Integer(1)))
            }
            Equation::Power(power) => {
                if let Equation::Division(division) = power.0 {
                    terms.insert_or_push(division.0, power.1.clone());
                    terms.insert_or_push(division.1, Equation::Negative(Box::new(power.1)));
                    continue;
                }
                (power.0, power.1)
            }
            Equation::Division(division) => {
                multiplication.remove(index);
                multiplication.push(division.0);
                return Equation::Division(Box::new((
                    Equation::Multiplication(multiplication),
                    division.1,
                )));
            }
            Equation::Addition(addition) => {
                multiplication.remove(index);
                return distribute_terms(&multiplication, addition);
            }
            term => (term, Equation::Variable(Variable::Integer(1))),
        };
        terms.insert_or_push(term, count);
    }

    let mut simplified_multiplication: Vec<Equation> = Vec::new();

    if total_is_negative {
        total_rational_factor *= -1;
    }
    if total_rational_factor != 1.into() || terms.0.is_empty() {
        simplified_multiplication.push(
            Equation::Variable(Variable::Rational(total_rational_factor)).simplify(step_logger),
        );
    }

    for (term, count) in terms.0 {
        simplified_multiplication.push(
            Equation::Power(Box::new((
                term,
                Equation::Addition(count).simplify(step_logger),
            )))
            .simplify(step_logger),
        );
    }

    if simplified_multiplication.len() == 1 {
        return simplified_multiplication.remove(0);
    }

    Equation::Multiplication(simplified_multiplication)
}
