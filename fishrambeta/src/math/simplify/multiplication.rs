use super::{Equation, Variable};
use num_rational::Rational64;
use std::collections::BTreeMap;

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
    return new_mult;
}

pub(super) fn simplify_multiplication(multiplication: Vec<Equation>) -> Equation {
    let mut multiplication = flatten_multiplication(multiplication);
    let mut terms: BTreeMap<Equation, Rational64> = BTreeMap::new();
    let mut total_rational_factor: Rational64 = 1.into();

    let mut total_is_negative = false;
    for (index, equation) in multiplication.iter().enumerate() {
        let (term, count) = match equation.clone().simplify() {
            Equation::Variable(Variable::Integer(0)) => {
                return Equation::Variable(Variable::Integer(0));
            }
            Equation::Variable(Variable::Integer(1)) => {
                continue;
            }
            Equation::Variable(Variable::Integer(n)) => {
                total_rational_factor = total_rational_factor * n;
                continue;
            }
            Equation::Variable(Variable::Rational(r)) => {
                total_rational_factor *= Rational64::new(r.0, r.1);
                continue;
            }
            Equation::Negative(negative) => {
                total_is_negative = !total_is_negative;
                (*negative, 1.into())
            }
            Equation::Power(power) => {
                if let Some(n) = power.1.get_number_or_none() {
                    (power.0, n)
                } else {
                    (Equation::Power(power), 1.into())
                }
            }
            Equation::Division(division) => {
                multiplication.remove(index);
                multiplication.push(division.0);
                return Equation::Division(Box::new((
                    Equation::Multiplication(multiplication),
                    division.1,
                )));
            }
            term => (term, 1.into()),
        };
        let previous_count = *terms.get(&term).unwrap_or(&0.into());
        terms.insert(term, previous_count + count);
    }

    let mut simplified_multiplication: Vec<Equation> = Vec::new();

    if total_is_negative {
        total_rational_factor *= -1;
    }
    if total_rational_factor != 1.into() {
        simplified_multiplication.push(
            Equation::Variable(Variable::Rational((
                *total_rational_factor.numer(),
                *total_rational_factor.denom(),
            )))
            .simplify(),
        );
    } 
    for (term, count) in terms {
        simplified_multiplication.push(
            Equation::Power(Box::new((
                term,
                Equation::Variable(Variable::Rational((*count.numer(), *count.denom()))).simplify(),
            )))
            .simplify(),
        );
    }

    if simplified_multiplication.len() == 1 {
        return simplified_multiplication.remove(0);
    }

    return Equation::Multiplication(simplified_multiplication);
}
