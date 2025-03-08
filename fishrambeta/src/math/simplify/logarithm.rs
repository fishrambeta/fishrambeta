use super::{Equation, Variable};
use crate::math::steps::StepLogger;
use num_rational::Rational64;

pub(super) fn simplify_ln(ln: Equation, step_logger: &mut Option<StepLogger>) -> Equation {
    match ln {
        Equation::Power(power) => {
            let (base, exponent) = *power;
            Equation::Multiplication(vec![
                exponent.simplify(step_logger),
                Equation::Ln(Box::new(base.simplify(step_logger))),
            ])
        }
        Equation::Division(division) => {
            let (numer, denom) = *division;
            Equation::Addition(vec![
                Equation::Ln(Box::new(numer.simplify(step_logger))),
                Equation::Negative(Box::new(Equation::Ln(Box::new(
                    denom.simplify(step_logger),
                )))),
            ])
        }
        other => Equation::Ln(Box::new(other.simplify(step_logger))),
    }
}
