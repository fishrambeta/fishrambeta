use crate::parser::{BracketType, IR};

impl IR {
    pub fn latex_to_ir(
        mut latex: Vec<char>,
        implicit_multiplication: bool,
    ) -> Result<Self, ParseError> {
        if Self::calculate_depth_difference(&latex) != 0 {
            return Err(ParseError::InvalidLatex);
        }
        latex = Self::make_implicit_multiplications_explicit(latex, implicit_multiplication);
        let top_level_operators = Self::get_top_level_operators_in_latex(&latex);
        if top_level_operators.any() {
            if top_level_operators.equals.len() > 0 {
                let mut parts = vec![];
                for &eq_position in top_level_operators.equals.iter().rev() {
                    let part = latex[eq_position + 1..].to_vec();
                    latex.truncate(eq_position);
                    parts.push(part);
                }
                parts.reverse(); //This can be removed, but makes testing easier
                let parsed_parts = parts
                    .into_iter()
                    .map(|part| {
                        (
                            Self::latex_to_ir(part, implicit_multiplication).unwrap(),
                            BracketType::None,
                        )
                    })
                    .collect::<Vec<_>>();
                return Ok(IR {
                    name: vec!['='],
                    parameters: parsed_parts,
                });
            }
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
                } else if *char == '+'
                    || *char == '-' && i != 0 && !BracketType::is_opening_bracket(latex[i - 1])
                {
                    //If the thing before the operator is not something that can be added to or subtracted from, this is not an operator
                    operators.additions_and_subtractions.push(i);
                } else if *char == '^' && Self::check_if_caret_is_power(latex, i) {
                    //This can also be used for superscript or integral upper bounds
                    operators.powers.push(i);
                }
            }
        }
        return operators;
    }
    //Checks whether the ^ char is a power or just superscript
    fn check_if_caret_is_power(latex: &Vec<char>, i: usize) -> bool {
        let mut command_buffer = vec![];
        let mut is_building_command_buffer = BracketType::is_closing_bracket(latex[i - 1]);
        let one_if_no_closing_bracket_first = if is_building_command_buffer { 0 } else { 1 };
        let mut depth = 0;
        for i in (0..(i - one_if_no_closing_bracket_first)).rev() {
            if is_building_command_buffer && latex[i] != '\\' {
                command_buffer.push(latex[i]);
            } else if is_building_command_buffer && latex[i] == '\\' {
                break;
            } else if BracketType::is_closing_bracket(latex[i]) {
                depth += 1;
            } else if BracketType::is_opening_bracket(latex[i]) {
                if depth == 0 {
                    if latex[i - 1] != '_' {
                        is_building_command_buffer = true;
                    } else {
                        depth -= 1;
                    }
                } else {
                    depth -= 1;
                }
            } else if depth == -1 {
                is_building_command_buffer = true;
            }
        }
        todo!()
    }
    fn make_implicit_multiplications_explicit(
        latex: Vec<char>,
        implicit_multiplication: bool,
    ) -> Vec<char> {
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
