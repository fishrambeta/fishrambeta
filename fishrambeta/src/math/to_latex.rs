use crate::math::Equation;
use crate::parser::IR;

impl Equation {
    pub fn to_latex(self: &Self) -> String {
        return IR::equation_to_latex(self.clone(), false);
    }
}
