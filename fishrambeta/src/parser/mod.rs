use num::Rational64;

use crate::math::{Constant, Equation, Variable};
use std::fmt;

pub mod formatters;

impl Equation {
    pub fn from_latex(latex: &str, implicit_multiplication: bool) -> Equation {
        //Cleanup steps
        let mut cleaned_latex = latex
            .replace("\\left(", "(")
            .replace("\\right)", ")")
            .replace("\\cdot", "*");
        if !implicit_multiplication {
            cleaned_latex = cleaned_latex.replace(' ', "");
        }

        Equation::from_latex_internal(&cleaned_latex, implicit_multiplication)
    }

    fn from_latex_internal(latex: &str, implicit_multiplication: bool) -> Equation {
        if let Some((a, b)) = split_latex_at_operator(latex, &'=') {
            return Equation::Equals(Box::new((
                Equation::from_latex_internal(a, implicit_multiplication),
                Equation::from_latex_internal(b, implicit_multiplication),
            )));
        }

        if let Some((a, b)) = split_latex_at_operator(latex, &'+') {
            return Equation::Addition(vec![
                Equation::from_latex_internal(a, implicit_multiplication),
                Equation::from_latex_internal(b, implicit_multiplication),
            ]);
        }

        if let Some((a, b)) = split_latex_at_operator(latex, &'-') {
            if latex.strip_prefix("-").is_none() {
                return Equation::Addition(vec![
                    Equation::from_latex_internal(a, implicit_multiplication),
                    Equation::Negative(Box::new(Equation::from_latex_internal(
                        b,
                        implicit_multiplication,
                    ))),
                ]);
            }
        }

        if let Some((a, b)) = split_latex_at_operator(latex, &'*') {
            return Equation::Multiplication(vec![
                Equation::from_latex_internal(a, implicit_multiplication),
                Equation::from_latex_internal(b, implicit_multiplication),
            ]);
        }

        if let Some((a, b)) = split_latex_at_operator(latex, &'/') {
            return Equation::Division(Box::new((
                Equation::from_latex_internal(a, implicit_multiplication),
                Equation::from_latex_internal(b, implicit_multiplication),
            )));
        }

        if let Ok(num) = latex.parse::<i64>() {
            return Equation::Variable(Variable::Integer(num));
        }

        if let Some(stripped) = latex.strip_prefix("-") {
            return Equation::Negative(Box::new(Equation::from_latex_internal(
                stripped,
                implicit_multiplication,
            )));
        }

        if let Some((left, right)) = latex.split_once('.') {
            if let (Ok(left_num), Ok(right_num)) = (left.parse::<i64>(), right.parse::<i64>()) {
                assert!(right_num >= 0);
                let log = right.len();
                let denom = 10_i64.pow(log.try_into().unwrap());
                let numer = left_num * denom + right_num;
                return Equation::Variable(Variable::Rational((numer, denom).into()));
            }
        }

        if let Some(parameters) = parse_latex_with_command(latex, "\\frac") {
            assert_eq!(parameters.len(), 2);
            return Equation::Division(Box::new((
                Equation::from_latex_internal(parameters[0], implicit_multiplication),
                Equation::from_latex_internal(parameters[1], implicit_multiplication),
            )));
        }

        if is_in_redundant_brackets(latex) {
            return Equation::from_latex_internal(
                &latex[1..latex.len() - 1],
                implicit_multiplication,
            );
        }

        if let Some((a, b)) = split_latex_at_operator(latex, &'^') {
            if implicit_multiplication {
                let mut a_variables = split_into_variables(a);
                let mut b_variables = split_into_variables(b);

                let a_stripped = a_variables
                    .pop()
                    .expect("Left side of power cannot be empty");
                let b_stripped = b_variables.remove(0);

                let mut multiplication_parts: Vec<_> = a_variables
                    .into_iter()
                    .chain(b_variables)
                    .map(|latex| Equation::from_latex_internal(latex, implicit_multiplication))
                    .collect();

                multiplication_parts.push(Equation::Power(Box::new((
                    Equation::from_latex_internal(a_stripped, implicit_multiplication),
                    Equation::from_latex_internal(b_stripped, implicit_multiplication),
                ))));
                return Equation::Multiplication(multiplication_parts);
            }
            return Equation::Power(Box::new((
                Equation::from_latex_internal(a, implicit_multiplication),
                Equation::from_latex_internal(b, implicit_multiplication),
            )));
        }

        if let Some(parameters) = parse_latex_with_command(latex, "\\sqrt") {
            assert_eq!(parameters.len(), 1);
            return Equation::Power(Box::new((
                Equation::from_latex_internal(parameters[0], implicit_multiplication),
                Equation::Variable(Variable::Rational(Rational64::new(1, 2))),
            )));
        }

        if let Some(parameters) = parse_latex_with_command(latex, "\\sin") {
            assert_eq!(parameters.len(), 1);
            return Equation::Sin(Box::new(Equation::from_latex_internal(
                parameters[0],
                implicit_multiplication,
            )));
        }

        if let Some(parameters) = parse_latex_with_command(latex, "\\cos") {
            assert_eq!(parameters.len(), 1);
            return Equation::Cos(Box::new(Equation::from_latex_internal(
                parameters[0],
                implicit_multiplication,
            )));
        }

        if let Some(parameters) = parse_latex_with_command(latex, "\\tan") {
            assert_eq!(parameters.len(), 1);
            return Equation::Division(Box::new((
                Equation::Sin(Box::new(Equation::from_latex_internal(
                    parameters[0],
                    implicit_multiplication,
                ))),
                Equation::Cos(Box::new(Equation::from_latex_internal(
                    parameters[0],
                    implicit_multiplication,
                ))),
            )));
        }

        if let Some(parameters) = parse_latex_with_command(latex, "\\arcsin") {
            assert_eq!(parameters.len(), 1);
            return Equation::Arcsin(Box::new(Equation::from_latex_internal(
                parameters[0],
                implicit_multiplication,
            )));
        }

        if let Some(parameters) = parse_latex_with_command(latex, "\\arccos") {
            assert_eq!(parameters.len(), 1);
            return Equation::Arccos(Box::new(Equation::from_latex_internal(
                parameters[0],
                implicit_multiplication,
            )));
        }

        if let Some(parameters) = parse_latex_with_command(latex, "\\arctan") {
            assert_eq!(parameters.len(), 1);
            return Equation::Arctan(Box::new(Equation::from_latex_internal(
                parameters[0],
                implicit_multiplication,
            )));
        }

        if let Some(parameters) = parse_latex_with_command(latex, "\\ln") {
            assert_eq!(parameters.len(), 1);
            return Equation::Ln(Box::new(Equation::from_latex_internal(
                parameters[0],
                implicit_multiplication,
            )));
        }

        let variable = if implicit_multiplication {
            let mut variables = split_into_variables(latex);
            if variables.len() > 1 {
                return Equation::Multiplication(
                    variables
                        .into_iter()
                        .map(|variable| {
                            Equation::from_latex_internal(variable, implicit_multiplication)
                        })
                        .collect(),
                );
            }
            variables.remove(0)
        } else {
            latex
        };

        match variable {
            "\\pi" => Equation::Variable(Variable::Constant(Constant::PI)),
            "e" => Equation::Variable(Variable::Constant(Constant::E)),
            letter => Equation::Variable(Variable::Letter(letter.to_string())),
        }
    }
}

fn is_in_redundant_brackets(latex: &str) -> bool {
    if !is_opening_bracket(latex.chars().next().expect("Latex string cannot be empty"))
        || !is_closing_bracket(latex.chars().last().expect("Latex string cannot be empty"))
    {
        return false;
    }
    let mut current_depth = 0;
    let length = latex.len();
    for (i, c) in latex.chars().enumerate() {
        if is_opening_bracket(c) {
            current_depth += 1;
        }
        if is_closing_bracket(c) {
            current_depth -= 1;
        }

        if current_depth == 0 {
            return i + 1 == length;
        }
    }
    panic!("Brackets are unbalanced in latex")
}

fn parse_latex_with_command<'a>(latex: &'a str, command: &'a str) -> Option<Vec<&'a str>> {
    if !latex.starts_with(command) {
        return None;
    }
    let stripped_latex = &latex[command.len()..];

    let mut current_depth = 0;
    let mut parameter_indices: Vec<usize> = vec![];
    for (i, c) in stripped_latex.chars().enumerate() {
        if current_depth == 0 {
            parameter_indices.push(i);
        }

        if is_opening_bracket(c) {
            current_depth += 1;
        }
        if is_closing_bracket(c) {
            current_depth -= 1;
        }
    }

    let mut parameters: Vec<&str> = vec![];
    for i in 0..parameter_indices.len() - 1 {
        parameters.push(&stripped_latex[parameter_indices[i] + 1..parameter_indices[i + 1] - 1]);
    }
    parameters.push(
        &stripped_latex
            [parameter_indices[parameter_indices.len() - 1] + 1..stripped_latex.len() - 1],
    );
    Some(parameters)
}

impl fmt::Display for Equation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_latex())
    }
}

fn split_latex_at_operator<'a>(latex: &'a str, operator: &'a char) -> Option<(&'a str, &'a str)> {
    let mut current_depth = 0;
    let mut right_start = latex.len();
    for (i, c) in latex.chars().rev().enumerate() {
        if is_opening_bracket(c) {
            current_depth += 1;
        }
        if is_closing_bracket(c) {
            current_depth -= 1;
        }

        if c == *operator && current_depth == 0 {
            right_start = right_start - i - 1;
            break;
        }
    }
    if right_start == latex.len() {
        None
    } else {
        Some((&latex[..right_start], &latex[right_start + 1..]))
    }
}

fn split_into_variables(latex: &str) -> Vec<&str> {
    let mut variables = Vec::new();
    let mut split: Vec<&str> = Vec::new();
    let mut remaining_latex = latex;
    while let Some((a, b)) = split_latex_at_operator(remaining_latex, &' ') {
        split.push(b);
        remaining_latex = a;
    }
    if !remaining_latex.is_empty() {
        split.push(remaining_latex);
    }
    for part in split.into_iter().rev() {
        let mut i = 0;
        while i < part.len() {
            let next_i = i + get_index_of_next_variable_end(&part[i..]);
            variables.push(&part[i..next_i]);
            i = next_i;
        }
    }
    variables
}

fn get_index_of_next_variable_end(latex: &str) -> usize {
    let mut variable_type = VariableType::None;
    let mut depth = 0;
    for (i, c) in latex.chars().enumerate() {
        if is_opening_bracket(c) {
            depth += 1;
        }
        if is_closing_bracket(c) {
            depth -= 1;
        }
        if depth != 0 {
            continue;
        }
        match variable_type {
            VariableType::None => {
                if c == '\\' {
                    variable_type = VariableType::Command;
                    continue;
                }
                if c.is_ascii_digit() || c == '.' {
                    variable_type = VariableType::Number;
                    continue;
                }
                variable_type = VariableType::Letter;
            }
            VariableType::Command => {
                if c == '_' {
                    variable_type = VariableType::LetterWithSubscript;
                    continue;
                }
                if c.is_ascii_digit() || c == '.' || c == '\\' {
                    return i;
                }
            }
            VariableType::Number => {
                if !(c.is_ascii_digit() || c == '.') {
                    return i;
                }
            }
            VariableType::Letter => {
                if c == '_' {
                    variable_type = VariableType::LetterWithSubscript;
                    continue;
                }
                return i;
            }
            VariableType::LetterWithSubscript => {
                return i + 1;
            }
        }
    }
    latex.len()
}

fn is_opening_bracket(c: char) -> bool {
    ['(', '{'].contains(&c)
}
fn is_closing_bracket(c: char) -> bool {
    [')', '}'].contains(&c)
}

#[derive(Debug)]
enum VariableType {
    None,
    Command,
    Number,
    Letter,
    LetterWithSubscript,
}
