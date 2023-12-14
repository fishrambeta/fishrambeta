use crate::math::{Constant, Equation, Variable};
use num::checked_pow;
use num::Signed;
use num_rational::Ratio;
use num_rational::Rational64;

impl Equation {
    pub fn calculate_exact(self: &Self) -> Option<Rational64> {
        match self {
            Equation::Variable(variable) => {
                match variable {
                    Variable::Integer(integer) => return Some(Rational64::from(*integer)),
                    Variable::Rational(rational) => return Some(*rational),
                    _ => {}
                }
                return None;
            }
            Equation::Addition(addition) => {
                return addition.iter().map(|x| x.calculate_exact()).sum()
            }
            Equation::Multiplication(multiplication) => {
                return multiplication.iter().map(|x| x.calculate_exact()).product()
            }
            Equation::Division(division) => {
                let numerator = division.0.calculate_exact();
                let denominator = division.1.calculate_exact();
                if numerator.is_none() || denominator.is_none() {
                    return None;
                }
                if denominator.unwrap() == 0.into() {
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
                let base_num = base.unwrap();
                if base_num == 1.into() {
                    return Some(1.into());
                }
                let exponent_num: usize = match exponent.unwrap().to_integer().try_into() {
                    Ok(x) => x,
                    Err(_) => return None,
                };
                let numerator = match checked_pow(*base_num.numer(), exponent_num) {
                    Some(x) => x,
                    None => return None,
                };
                let denominator = match checked_pow(*base_num.denom(), exponent_num) {
                    Some(x) => x,
                    None => return None,
                };
                return Some(Rational64::new(numerator, denominator));
            }
            Equation::Abs(abs) => {
                let abs = abs.calculate_exact();
                if abs.is_none() {
                    return None;
                }
                return Some(abs.unwrap().abs());
            }
            Equation::Ln(ln) => {
                if **ln == Equation::Variable(Variable::Constant(Constant::E)) {
                    return Some(1.into());
                }
                return None;
            }
            _ => return None,
        }
    }
}
