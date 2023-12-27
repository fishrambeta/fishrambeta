use crate::parser::{BracketType, IR};

impl IR {
    pub fn latex_to_ir(
        latex: Vec<char>,
        implicit_multiplication: bool,
    ) -> Result<Self, ParseError> {
        if Self::calculate_depth_difference(&latex) != 0 {
            return Err(ParseError::InvalidLatex);
        }
        let top_level_operators = Self::get_top_level_operators_in_latex(&latex);
        if top_level_operators.any() {
            todo!()
        } else {
            todo!()
        }
    }
    pub fn ir_to_latex(mut self, implicit_multiplication: bool) -> Vec<char> {
        todo!()
    }
    fn calculate_depth_difference(latex: &Vec<char>) -> isize {
        let mut depth = 0;
        for char in latex.iter() {
            if BracketType::is_opening_bracket(*char) {
                depth += 1;
            } else if BracketType::is_closing_bracket(*char) {
                depth -= 1;
            }
        }
        return depth;
    }
    fn get_top_level_operators_in_latex(latex: &Vec<char>) -> TopLevelOperators {
        let mut depth = 0;
        let mut operators = TopLevelOperators {
            powers: vec![],
            multiplications_and_divisions: vec![],
            additions_and_subtractions: vec![],
            equals: vec![],
        };
        for (i, char) in latex.iter().enumerate() {
            if BracketType::is_opening_bracket(*char) {
                depth -= 1;
            } else if BracketType::is_closing_bracket(*char) {
                depth += 1;
            } else if depth == 0 {
                if *char == '=' {
                    operators.equals.push(i)
                } else if *char == '*' || *char == '/' {
                    operators.multiplications_and_divisions.push(i)
                } else if *char == '+' || *char == '-' && i != 0 {
                    //If the thing before the operator is not something that can be added to or subtracted from, this is not an operator
                    todo!()
                } else if *char == '^' {
                    //This can also be used for superscript or integral upper bounds
                    todo!()
                }
            }
        }
        todo!()
    }
}
struct TopLevelOperators {
    powers: Vec<usize>,
    multiplications_and_divisions: Vec<usize>,
    additions_and_subtractions: Vec<usize>,
    equals: Vec<usize>,
}
impl TopLevelOperators {
    pub fn any(&self) -> bool {
        return self.powers.len() > 0
            || self.multiplications_and_divisions.len() > 0
            || self.additions_and_subtractions.len() > 0
            || self.equals.len() > 0;
    }
}
#[derive(Debug)]
pub enum ParseError {
    InvalidLatex,
}
