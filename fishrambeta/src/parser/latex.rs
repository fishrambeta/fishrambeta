use crate::math::{Constant, Equation, Variable};
use crate::parser::{BracketType, TopLevelOperators, IR};
use num_rational::Rational64;

impl IR {
    pub fn latex_to_ir(latex: Vec<char>, implicit_multiplication: bool) -> Self {
        todo!()
    }
    pub fn ir_to_latex(mut self, implicit_multiplication: bool) -> Vec<char> {
        todo!()
    }
}
