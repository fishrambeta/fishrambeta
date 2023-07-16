use crate::math::{Equation, Variable};
use num_rational::Rational64;

impl Equation {
    pub fn calculate_exact(self: &Self) -> Option<Rational64> {
        match self {
            Equation::Variable(variable) => {
                match variable {
                    Variable::Integer(integer) => return Some(Rational64::from(*integer)),
                    Variable::Rational(rational) => {
                        return Some(Rational64::new(rational.0, rational.1))
                    }
                    _ => {}
                }
                return None;
            }
            Equation::Addition(addition) => {
                return addition.iter().map(|x| x.calculate_exact()).sum()
            }
            Equation::Subtraction(subtraction) => {
                let minus: Option<Rational64> = subtraction
                    .iter()
                    .skip(1)
                    .map(|x| x.calculate_exact())
                    .sum();
                let plus: Option<Rational64> = subtraction[0].calculate_exact();
                if minus.is_none() || plus.is_none() {
                    return None;
                }
                return Some(plus.unwrap() - minus.unwrap());
            } // TODO make this in one statement, but rust hates me
            Equation::Multiplication(multiplication) => {
                return multiplication.iter().map(|x| x.calculate_exact()).product()
            }
            Equation::Division(division) => {
                let numerator = division.0.calculate_exact();
                let denominator = division.1.calculate_exact();
                if numerator.is_none() || denominator.is_none() {
                    return None;
                }
                return Some(numerator.unwrap() / denominator.unwrap());
            }
            Equation::Power(power) => {
                let base = power.0.calculate_exact();
                let exponent = power.1.calculate_exact();
                if base.is_none() || exponent.is_none() {
                    return None;
                }
                if !exponent.unwrap().is_integer() {
                    return None;
                }
                return Some(
                    base.unwrap()
                        .pow(exponent.unwrap().to_integer().try_into().unwrap()),
                );
            }
            _ => None,
        }
    }
}
