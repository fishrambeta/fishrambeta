use crate::math::{Equation, Variable};
use std::collections::BTreeMap;

impl Equation {
    pub fn calculate(&self, values: &BTreeMap<Variable, f64>) -> f64 {
        match self {
            Equation::Variable(variable) => {
                match variable {
                    Variable::Integer(integer) => return *integer as f64,
                    Variable::Rational(rational) => {
                        return *(*rational).numer() as f64 / *(*rational).denom() as f64
                    }
                    _ => {}
                }
                values[&variable]
            }
            Equation::Negative(negative) => -negative.calculate(values),
            Equation::Addition(addition) => {
                return addition.iter().map(|x| x.calculate(values)).sum()
            }
            Equation::Multiplication(multiplication) => {
                return multiplication
                    .iter()
                    .map(|x| x.calculate(values))
                    .product()
            }
            Equation::Division(division) => {
                division.0.calculate(values) / division.1.calculate(values)
            }
            Equation::Power(power) => {
                power.0.calculate(values).powf(power.1.calculate(values))
            }
            Equation::Ln(ln) => ln.calculate(values).ln(),
            Equation::Sin(sin) => sin.calculate(values).sin(),
            Equation::Cos(cos) => cos.calculate(values).cos(),
            Equation::Abs(abs) => abs.calculate(values).abs(),
            Equation::Equals(_) => panic!("Cannot calculate equals"),
        }
    }
}
