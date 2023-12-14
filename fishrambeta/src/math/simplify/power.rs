use super::{Equation, Variable};
use num_rational::Rational64;
use num_rational::Ratio;

pub(super) fn simplify_power(power: Box<(Equation, Equation)>) -> Equation {
    let base = power.0.simplify();
    let exponent = power.1.simplify();

    if let Some(n) = exponent.get_number_or_none() {
        if n == 1.into() {
            return base;
        } else if n == 0.into() {
            return Equation::Variable(Variable::Integer(1));
        }
    }

    match base {
        Equation::Multiplication(terms) => {
            let mut simplified_power: Vec<Equation> = vec![];
            for term in terms.into_iter() {
                simplified_power.push(Equation::Power(Box::new((term, exponent.clone()))));
            }
            return Equation::Multiplication(simplified_power);
        }
        Equation::Power(ref power) => {
            if let Some(e1) = exponent.get_number_or_none() {
                if let Some(e2) = power.1.get_number_or_none() {
                    return Equation::Power(Box::new((
                        power.0.clone(),
                        Equation::Variable(Variable::Rational(e1 * e2)),
                    )));
                }
            }
        }
        Equation::Division(ref division) => {
            if let Some(number) = division.0.get_number_or_none() {
                return Equation::Division(Box::new((
                    Equation::Power(Box::new((
                        Equation::Variable(Variable::Rational(number)),
                        exponent.clone(),
                    ))),
                    Equation::Power(Box::new((division.1.clone(), exponent))),
                )));
            }
        }
        _ => {}
    }

    return Equation::Power(Box::new((base, exponent)));
}
