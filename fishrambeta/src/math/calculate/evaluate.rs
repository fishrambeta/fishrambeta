use super::{Equation, Variable};

impl Equation {
    pub fn evaluate(self, variable: &Variable, replacement: &Equation) -> Equation {
        match self {
            Equation::Variable(t) => {
                if t == *variable {
                    replacement.clone()
                } else {
                    Equation::Variable(t)
                }
            }
            Equation::Negative(t) => {
                Equation::Negative(Box::new(t.evaluate(variable, replacement)))
            }
            Equation::Addition(t) => Equation::Addition(
                t.into_iter()
                    .map(|t| t.evaluate(variable, replacement))
                    .collect(),
            ),
            Equation::Multiplication(t) => Equation::Multiplication(
                t.into_iter()
                    .map(|t| t.evaluate(variable, replacement))
                    .collect(),
            ),
            Equation::Division(t) => Equation::Division(Box::new((
                t.0.evaluate(variable, replacement),
                t.1.evaluate(variable, replacement),
            ))),
            Equation::Power(t) => Equation::Power(Box::new((
                t.0.evaluate(variable, replacement),
                t.1.evaluate(variable, replacement),
            ))),
            Equation::Ln(t) => Equation::Ln(Box::new(t.evaluate(variable, replacement))),
            Equation::Sin(t) => Equation::Sin(Box::new(t.evaluate(variable, replacement))),
            Equation::Cos(t) => Equation::Cos(Box::new(t.evaluate(variable, replacement))),
            Equation::Arcsin(t) => Equation::Arcsin(Box::new(t.evaluate(variable, replacement))),
            Equation::Arccos(t) => Equation::Arccos(Box::new(t.evaluate(variable, replacement))),
            Equation::Arctan(t) => Equation::Arctan(Box::new(t.evaluate(variable, replacement))),
            Equation::Abs(t) => Equation::Abs(Box::new(t.evaluate(variable, replacement))),
            Equation::Derivative(_) => panic!(),
            Equation::Equals(_) => panic!(),
        }
    }
}
