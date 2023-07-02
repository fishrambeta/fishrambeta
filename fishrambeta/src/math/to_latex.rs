use crate::math::{Equation, Variable};

impl Equation {
    pub fn to_latex(self: &Self) -> String {
        match self {
            Equation::Variable(variable) => match variable {
                Variable::Letter(letter) => return letter.to_string(),
                Variable::Vector(vector) => return vector.to_string(),
                Variable::Integer(integer) => return integer.to_string(),
                _ => todo!(),
            },
            Equation::Addition(addition) => {
                return format!(
                    "({})",
                    addition
                        .into_iter()
                        .map(|x| x.to_latex())
                        .collect::<Vec<_>>()
                        .join("+")
                )
            }
            Equation::Subtraction(subtraction) => {
                return format!(
                    "({})",
                    subtraction
                        .into_iter()
                        .map(|x| x.to_latex())
                        .collect::<Vec<_>>()
                        .join("-")
                )
            }
            Equation::Multiplication(multiplication) => {
                return format!(
                    "({})",
                    multiplication
                        .into_iter()
                        .map(|x| x.to_latex())
                        .collect::<Vec<_>>()
                        .join("*")
                )
            }
            Equation::Power(power) => {
                return format!("{{{}}}^{{{}}}", power.0.to_latex(), power.1.to_latex())
            }
            Equation::Ln(ln) => return format!("\\ln{{{}}}", ln.to_latex()),
            Equation::Division(division) => {
                return format!(
                    "\\frac{{{}}}{{{}}}",
                    division.0.to_latex(),
                    division.1.to_latex()
                )
            }
            Equation::Equals(equals) => {
                return format!("{}={}", equals.0.to_latex(), equals.1.to_latex())
            }
        }
    }
}
