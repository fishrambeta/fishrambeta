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
            return if top_level_operators.equals.len() > 0 {
                let mut parts = vec![];
                for &eq_position in top_level_operators.equals.iter().rev() {
                    let part = latex[eq_position + 1..].to_vec();
                    latex.truncate(eq_position);
                    parts.push(part);
                }
                parts.push(latex);
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
                Ok(IR {
                    name: vec!['='],
                    parameters: parsed_parts,
                })
            } else if top_level_operators.additions_and_subtractions.len() > 0 {
                let mut parts = vec![];
                for &operator_position in
                top_level_operators.additions_and_subtractions.iter().rev()
                {
                    let operator = latex[operator_position];
                    let part = latex[(operator_position + 1)..].to_vec();
                    latex.truncate(operator_position);
                    parts.push((part, operator));
                }
                parts.push((latex, '+'));
                parts.reverse(); //This can be removed, but makes testing easier
                let parsed_parts = parts
                    .into_iter()
                    .map(|part| {
                        if part.1 == '+' {
                            (
                                Self::latex_to_ir(part.0, implicit_multiplication).unwrap(),
                                BracketType::None,
                            )
                        } else {
                            (
                                IR {
                                    name: vec!['i', 'n', 'v'],
                                    parameters: vec![(
                                        Self::latex_to_ir(part.0, implicit_multiplication).unwrap(),
                                        BracketType::None,
                                    )],
                                },
                                BracketType::None,
                            )
                        }
                    })
                    .collect::<Vec<_>>();
                Ok(IR {
                    name: vec!['+'],
                    parameters: parsed_parts,
                })
            } else if top_level_operators.multiplications_and_divisions.len() > 0 {
                let mut parts = vec![];
                for &operator_position in top_level_operators
                    .multiplications_and_divisions
                    .iter()
                    .rev()
                {
                    let operator = latex[operator_position];
                    let part = latex[(operator_position + 1)..].to_vec();
                    latex.truncate(operator_position);
                    parts.push((part, operator));
                }
                parts.push((latex, '*'));
                parts.reverse(); //This can be removed, but makes testing easier
                let parsed_parts = parts
                    .into_iter()
                    .map(|part| {
                        if part.1 == '*' {
                            (
                                Self::latex_to_ir(part.0, implicit_multiplication).unwrap(),
                                BracketType::Curly,
                            )
                        } else {
                            (
                                IR {
                                    name: vec!['^'],
                                    parameters: vec![
                                        (
                                            Self::latex_to_ir(part.0, implicit_multiplication)
                                                .unwrap(),
                                            BracketType::Curly,
                                        ),
                                        (
                                            IR {
                                                name: vec!['i', 'n', 'v'],
                                                parameters: vec![(
                                                    IR {
                                                        name: vec!['1'],
                                                        parameters: vec![],
                                                    },
                                                    BracketType::None,
                                                )],
                                            },
                                            BracketType::None,
                                        ),
                                    ],
                                },
                                BracketType::None,
                            )
                        }
                    })
                    .collect::<Vec<_>>();
                Ok(IR {
                    name: vec!['*'],
                    parameters: parsed_parts,
                })
            } else {
                let mut parts = vec![];
                for eq_position in top_level_operators.powers {
                    let part = latex[eq_position + 1..].to_vec();
                    latex.truncate(eq_position);
                    parts.push(part);
                }
                parts.push(latex);
                parts.reverse(); //This can be removed, but makes testing easier
                let parsed_parts = parts
                    .into_iter()
                    .map(|part| {
                        (
                            Self::latex_to_ir(part, implicit_multiplication).unwrap(),
                            BracketType::Curly,
                        )
                    })
                    .collect::<Vec<_>>();
                Ok(IR {
                    name: vec!['^'],
                    parameters: parsed_parts,
                })
            };
        } else {
            if latex[0] == '\\' {
                latex.remove(0);
                let mut command = vec!['\\'];
                while latex.len() > 0 && !BracketType::is_opening_bracket(latex[0]) {
                    command.push(latex.remove(0))
                }
                let mut parameters = vec![];
                while latex.len() > 0 {
                    let mut depth = 1;
                    let mut next_param = vec![];
                    let bracket_type = BracketType::get_opening_bracket_type(latex.remove(0));
                    loop {
                        let next = latex.remove(0);
                        if BracketType::is_opening_bracket(next) {
                            depth += 1;
                            next_param.push(next)
                        } else if BracketType::is_closing_bracket(next) {
                            depth -= 1;
                            if depth != 0 {
                                next_param.push(next)
                            } else {
                                parameters.push((
                                    Self::latex_to_ir(next_param, implicit_multiplication).unwrap(),
                                    bracket_type,
                                ));
                                next_param = vec![];
                                break;
                            }
                        } else {
                            next_param.push(next);
                        }
                    }
                }
                return Ok(Self {
                    name: command,
                    parameters,
                });
            } else {
                if latex.iter().all(|char| char.is_alphabetic()) {
                    return Ok(Self {
                        name: latex,
                        parameters: vec![],
                    });
                } else if latex.iter().all(|char| char.is_numeric() || *char == '.') {
                    return Ok(Self {
                        name: latex,
                        parameters: vec![],
                    });
                } else if BracketType::is_opening_bracket(latex[0])
                    && BracketType::is_closing_bracket(latex[latex.len() - 1])
                {
                    latex.remove(latex.len() - 1);
                    latex.remove(0);
                    return Self::latex_to_ir(latex, implicit_multiplication);
                } else if latex[0] == '-' {
                    latex.remove(0);
                    return Ok(Self {
                        name: vec!['i', 'n', 'v'],
                        parameters: vec![(
                            Self::latex_to_ir(latex, implicit_multiplication).unwrap(),
                            BracketType::Curly,
                        )],
                    });
                }
                todo!();
            }
        }
    }
    pub fn ir_to_latex(mut self, implicit_multiplication: bool) -> Vec<char> {
        let mut latex = vec![];
        match self.name[..] {
            ['\\', 'f', 'r', 'a', 'c']
            | ['\\', 'c', 'o', 's']
            | ['\\', 's', 'i', 'n']
            | ['\\', 't', 'a', 'n']
            | ['\\', 'v', 'e', 'c']
            | ['\\', 'l', 'n'] => {
                latex.append(&mut self.name);
                for param in self.parameters {
                    if let Some(opening_bracket) = param.1.opening_bracket() {
                        latex.push(opening_bracket)
                    };
                    latex.append(&mut param.0.ir_to_latex(implicit_multiplication));
                    if let Some(closing_bracket) = param.1.closing_bracket() {
                        latex.push(closing_bracket)
                    };
                }
            }
            ['\\', 'i', 'n', 'v'] => {
                while self.parameters.len() > 1 {
                    let next = self.parameters.remove(0);
                    if let Some(opening_bracket) = next.1.opening_bracket() {
                        latex.push(opening_bracket)
                    };
                    latex.append(&mut next.0.ir_to_latex(implicit_multiplication));
                    if let Some(closing_bracket) = next.1.closing_bracket() {
                        latex.push(closing_bracket)
                    };
                    latex.push('-');
                }
                if let Some(opening_bracket) = self.parameters[0].1.opening_bracket() {
                    latex.push(opening_bracket)
                };
                let closing_bracket = self.parameters[0].1.closing_bracket();
                latex.append(
                    &mut self
                        .parameters
                        .remove(0)
                        .0
                        .ir_to_latex(implicit_multiplication),
                );
                if closing_bracket.is_some() {
                    latex.push(closing_bracket.unwrap())
                };
            }
            ['+'] | ['-'] | ['*'] | ['='] | ['^'] => {
                while self.parameters.len() > 1 {
                    let next = self.parameters.remove(0);
                    if let Some(opening_bracket) = next.1.opening_bracket() {
                        latex.push(opening_bracket)
                    };
                    latex.append(&mut next.0.ir_to_latex(implicit_multiplication));
                    if let Some(closing_bracket) = next.1.closing_bracket() {
                        latex.push(closing_bracket)
                    };
                    latex.append(&mut self.name)
                }
                if let Some(bracket) = self.parameters[0].1.opening_bracket() {
                    latex.push(bracket)
                };
                let closing_bracket = self.parameters[0].1.closing_bracket();
                latex.append(
                    &mut self
                        .parameters
                        .remove(0)
                        .0
                        .ir_to_latex(implicit_multiplication),
                );
                if closing_bracket.is_some() {
                    latex.push(closing_bracket.unwrap())
                };
            }
            _ => {
                if self.name[0] == '\\' {
                    todo!(
                        "Unsupported ir_to_latex IR: {}",
                        self.name.into_iter().collect::<String>()
                    )
                } else {
                    latex.append(&mut self.name);
                    for param in self.parameters {
                        if let Some(bracket) = param.1.opening_bracket() {
                            latex.push(bracket)
                        };
                        latex.append(&mut param.0.ir_to_latex(implicit_multiplication));
                        if let Some(bracket) = param.1.closing_bracket() {
                            latex.push(bracket)
                        };
                    }
                }
            }
        }
        return latex;
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
    fn check_if_caret_is_power(latex: &Vec<char>, pos: usize) -> bool {
        let mut parameter_count = 0;
        let mut command = vec![];
        let mut depth = if BracketType::is_closing_bracket(latex[pos - 1]) {
            1
        } else {
            0
        };
        for i in (0..(pos - depth)).rev() {
            if depth == 0 && !BracketType::is_closing_bracket(latex[i]) {
                if i == 0 && latex[i] != '\\' {
                    command = vec![];
                    break;
                }
                if !latex[i].is_alphabetic() {
                    if latex[i] != '\\' {
                        command = vec![];
                    }
                    break;
                } else if latex[i].is_alphabetic() {
                    command.push(latex[i])
                }
            } else if depth == 0 && BracketType::is_opening_bracket(latex[i + 1]) {
                parameter_count += 1;
            }
            if BracketType::is_closing_bracket(latex[i]) {
                depth += 1;
            } else if BracketType::is_opening_bracket(latex[i]) {
                depth -= 1;
            }
        }
        return if command.len() == 0 {
            true
        } else {
            command.reverse();
            Self::get_parameter_count(&command) == parameter_count
        };
    }
    fn make_implicit_multiplications_explicit(
        mut latex: Vec<char>,
        implicit_multiplication: bool,
    ) -> Vec<char> {
        if implicit_multiplication {
            //Add multiplication signs where two letters are next to eachother, but don't do it in commands
            let mut new_latex: Vec<char> = vec![];
            let mut multiplication_insertion_suspended = false;
            for char in latex {
                if char == '\\' {
                    if let Some(prev) = new_latex.last()
                        && !multiplication_insertion_suspended {
                        if *prev != '*' { new_latex.push('*') }
                    }
                    multiplication_insertion_suspended = true
                } else if multiplication_insertion_suspended && !char.is_alphabetic() {
                    multiplication_insertion_suspended = false
                } else if let Some(prev) = new_latex.last()
                    && !multiplication_insertion_suspended
                {
                    if prev.is_alphabetic() && char.is_alphabetic() {
                        new_latex.push('*');
                    } else if (prev.is_alphabetic() && char.is_numeric())
                        || prev.is_numeric() && char.is_alphabetic()
                    {
                        new_latex.push('*')
                    }
                }
                new_latex.push(char);
            }
            latex = new_latex;
        }
        //Add multiplications between closing and opening brackets
        let mut new_latex = vec![];
        let mut building_command = false;
        let mut command = vec![];
        let mut depth = 0;
        let mut parameter_count = 0;
        for char in latex {
            //Some commands have multiple params
            if BracketType::is_opening_bracket(char) {
                depth += 1;
            } else if BracketType::is_closing_bracket(char) {
                depth -= 1;
            }
            if depth == 0 {
                if char == '\\' {
                    building_command = true;
                } else if building_command && !char.is_alphabetic() {
                    building_command = false;
                    parameter_count = Self::get_parameter_count(&command);
                } else if building_command {
                    command.push(char);
                } else {
                    command = vec![]
                }
            }
            if depth <= 1
                && BracketType::is_opening_bracket(char)
                && let Some(&prev) = new_latex.last()
            {
                if BracketType::is_closing_bracket(prev) {
                    if parameter_count == 0 {
                        new_latex.push('*')
                    } else {
                        parameter_count -= 1
                    }
                }
            }
            new_latex.push(char);
        }
        return new_latex;
    }
    fn get_parameter_count(command: &Vec<char>) -> u32 {
        let command = command.iter().collect::<String>();
        return match command.as_str() {
            "tan" | "cos" | "sin" => 0,
            "vec" => 1,
            "frac" => 2,
            _ => {
                todo!("{} has no specified parameter count", command)
            }
        };
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
