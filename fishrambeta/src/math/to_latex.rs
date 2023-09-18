use crate::math::Equation;
use crate::parser::IR;
use std::fmt;

impl Equation {
    pub fn to_latex(self: &Self) -> String {
        return IR::equation_to_latex(self.clone(), false);
    }
}

impl fmt::Display for Equation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_latex())
    }
}
