use crate::math::polynomial::Polynomial;
use crate::math::Equation;
use crate::parser::IR;
use std::fmt;

impl Equation {
    pub fn to_latex(&self) -> String {
        IR::equation_to_latex(self.clone(), false)
    }
    pub fn to_numpy(&self) -> String {
        IR::equation_to_numpy(self.clone(), false)
    }
}

impl fmt::Display for Equation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_latex())
    }
}

impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_latex())
    }
}
