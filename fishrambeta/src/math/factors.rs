use crate::math::{Equation, Variable};

impl Equation {
    fn has_factor(self: Equation, factor: Equation) -> bool {
        if self == factor {
            return true;
        }

        match self {
            Equation::Power(power) => return power.0.has_factor(factor),
            Equation::Multiplication(multiplication) => {
                return multiplication
                    .iter()
                    .any(|x| x.clone().has_factor(factor.clone()))
            }
            Equation::Addition(addition) => {
                return addition
                    .iter()
                    .all(|x| x.clone().has_factor(factor.clone()))
            }
            Equation::Subtraction(subtraction) => {
                return subtraction
                    .iter()
                    .all(|x| x.clone().has_factor(factor.clone()))
            }
            _ => return false,
        }
    }
}
