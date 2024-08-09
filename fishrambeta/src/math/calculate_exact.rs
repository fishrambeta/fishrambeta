use crate::math::{Constant, Equation, Variable};
use num::checked_pow;
use num::Signed;

use num_rational::Rational64;

impl Equation {
    #[must_use]
    pub fn calculate_exact(&self) -> Option<Rational64> {
        match self {
            Equation::Variable(variable) => {
                match variable {
                    Variable::Integer(integer) => return Some(Rational64::from(*integer)),
                    Variable::Rational(rational) => return Some(*rational),
                    _ => {}
                }
                None
            }
            Equation::Addition(addition) => {
                return addition.iter().map(super::Equation::calculate_exact).sum()
            }
            Equation::Multiplication(multiplication) => {
                return multiplication
                    .iter()
                    .map(super::Equation::calculate_exact)
                    .product()
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
                Some(numerator.unwrap() / denominator.unwrap())
            }
            Equation::Power(power) => {
                let base = power.0.calculate_exact()?;
                let exponent = power.1.calculate_exact()?;

                if base == 1.into() {
                    return Some(1.into());
                }
                if !exponent.is_integer() {
                    return None;
                }
                let exponent_num: usize = match exponent.to_integer().try_into() {
                    Ok(x) => x,
                    Err(_) => return None,
                };
                let numerator = checked_pow(*base.numer(), exponent_num)?;
                let denominator = checked_pow(*base.denom(), exponent_num)?;
                Some(Rational64::new(numerator, denominator))
            }
            Equation::Abs(abs) => {
                let abs = abs.calculate_exact()?;

                Some(abs.abs())
            }
            Equation::Ln(ln) => {
                if **ln == Equation::Variable(Variable::Constant(Constant::E)) {
                    return Some(1.into());
                }
                None
            }
            _ => None,
        }
    }
}
