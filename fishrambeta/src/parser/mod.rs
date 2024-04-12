use num_rational::Rational64;

use crate::math::{Constant, Equation, Variable};

mod equation;
mod latex;
mod numpy;

pub struct IR {
    name: Vec<char>,
    parameters: Vec<(IR, BracketType)>,
}
impl IR {
    pub fn latex_to_equation(latex: Vec<char>, implicit_multiplication: bool) -> Equation {
        let sanitized_latex = cleanup_latex(latex);
        return Self::latex_to_ir(sanitized_latex, implicit_multiplication, true)
            .unwrap()
            .ir_to_equation();
    }
    pub fn equation_to_latex(equation: Equation, implicit_multiplication: bool) -> String {
        return Self::equation_to_ir(equation)
            .ir_to_latex(implicit_multiplication)
            .into_iter()
            .collect::<String>();
    }
    pub fn equation_to_numpy(equation: Equation, implicit_multiplication: bool) -> String {
        return Self::equation_to_ir(equation)
            .ir_to_numpy(implicit_multiplication)
            .into_iter()
            .collect::<String>();
    }
}
pub enum BracketType {
    None,
    Curly,
    Square,
    Round,
    Angle,
}
impl BracketType {
    pub fn opening_bracket(&self) -> Option<char> {
        return match self {
            Self::None => None,
            Self::Angle => Some('⟨'),
            Self::Curly => Some('{'),
            Self::Square => Some('['),
            Self::Round => Some('('),
        };
    }
    pub fn closing_bracket(&self) -> Option<char> {
        return match self {
            BracketType::None => None,
            BracketType::Curly => Some('}'),
            BracketType::Square => Some(']'),
            BracketType::Round => Some(')'),
            BracketType::Angle => Some('⟩'),
        };
    }
    pub fn is_opening_bracket(char: char) -> bool {
        return char == '{' || char == '[' || char == '(' || char == '⟨';
    }
    pub fn is_closing_bracket(char: char) -> bool {
        return char == '}' || char == ']' || char == ')' || char == '⟩';
    }
    pub fn get_opening_bracket_type(char: char) -> Self {
        return match char {
            '(' => BracketType::Round,
            '[' => BracketType::Square,
            '{' => BracketType::Curly,
            '⟨' => BracketType::Angle,
            _ => BracketType::None,
        };
    }
}
pub fn cleanup_latex(latex: Vec<char>) -> Vec<char> {
    return latex
        .into_iter()
        .collect::<String>()
        .replace("\\cdot", "*")
        .replace(" ", "")
        .replace("\\left", "")
        .replace("\\right", "")
        .chars()
        .collect::<Vec<char>>();
}
