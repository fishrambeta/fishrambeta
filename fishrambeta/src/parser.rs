use crate::math::{Constant, Equation, Variable};

pub struct IR {
    name: Vec<char>,
    parameters: Vec<(IR, BracketType)>,
}
impl IR {
    pub fn latex_to_equation(latex: Vec<char>, implicit_multiplication: bool) -> Equation {
        if !Self::calculate_depth_difference(&latex) == 0 {
            panic!("Invalid latex");
        }
        let mut sanitized_latex = cleanup_latex(latex);
        return Self::latex_to_ir(sanitized_latex, implicit_multiplication).ir_to_equation();
    }
    pub fn equation_to_latex(equation: Equation, implicit_multiplication: bool) -> String {
        return Self::equation_to_ir(equation)
            .ir_to_latex(implicit_multiplication)
            .into_iter()
            .collect::<String>();
    }
    pub fn latex_to_ir(latex: Vec<char>, implicit_multiplication: bool) -> Self {
        let mut latex = latex;
        while latex[0] == '+' {
            latex.remove(0);
        }
        let top_level_operators =
            Self::get_operators_in_top_level_from_latex(&latex, implicit_multiplication);
        if top_level_operators.any() {
            return if top_level_operators.additions_and_subtractions.len() > 0 {
                let (lhs, rhs) = latex.split_at(top_level_operators.additions_and_subtractions[0]);
                let (lhs, mut rhs) = (lhs.to_vec(), rhs.to_vec());
                let operator = rhs.remove(0);
                if lhs.len() == 0 && operator == '-' {
                    IR {
                        name: vec!['\\', 'i', 'n', 'v'],
                        parameters: vec![(
                            Self::latex_to_ir(rhs, implicit_multiplication),
                            BracketType::Round,
                        )],
                    }
                } else {
                    IR {
                        name: vec![operator],
                        parameters: vec![
                            (
                                Self::latex_to_ir(lhs, implicit_multiplication),
                                BracketType::None,
                            ),
                            (
                                Self::latex_to_ir(rhs, implicit_multiplication),
                                BracketType::None,
                            ),
                        ],
                    }
                }
            } else if top_level_operators.multiplications_and_divisions.len() > 0 {
                let (lhs, rhs) =
                    latex.split_at(top_level_operators.multiplications_and_divisions[0]);
                let (lhs, mut rhs) = (lhs.to_vec(), rhs.to_vec());
                let operator = rhs.remove(0);
                IR {
                    name: vec![operator],
                    parameters: vec![
                        (
                            Self::latex_to_ir(lhs, implicit_multiplication),
                            BracketType::None,
                        ),
                        (
                            Self::latex_to_ir(rhs, implicit_multiplication),
                            BracketType::None,
                        ),
                    ],
                }
            } else {
                let mut parts = vec![];
                for power in top_level_operators.powers {
                    let (lhs, rhs) = latex.split_at(power);
                    let (lhs, mut rhs) = (lhs.to_vec(), rhs.to_vec());
                    rhs.remove(0);
                    latex = rhs;
                    parts.push(lhs);
                }
                parts.push(latex);
                Self {
                    name: vec!['^'],
                    parameters: parts
                        .into_iter()
                        .map(|parts| {
                            (
                                Self::latex_to_ir(parts, implicit_multiplication),
                                BracketType::None,
                            )
                        })
                        .collect::<Vec<_>>(),
                }
            };
        } else {
            if BracketType::is_opening_bracket(latex[0])
                && BracketType::is_closing_bracket(latex[latex.len() - 1])
            {
                let bracket_type = BracketType::get_opening_bracket_type(latex.remove(0));
                latex.remove(latex.len() - 1);
                return Self::latex_to_ir(latex, implicit_multiplication);
            }
            if latex.starts_with(&['\\']) {
                latex.remove(0);
                let mut command = vec![];
                loop {
                    if latex[0] == '{'
                        || latex[0] == '('
                        || latex[0] == '['
                        || latex[0] == '^'
                        || latex[0] == '_'
                        || latex[0] == '\\'
                    {
                        break;
                    }
                    command.push(latex.remove(0));
                    if latex.len() == 0 {
                        break;
                    }
                }
                if command == ['i', 'n', 't'] {
                    let (superscript, subscript) =
                        Self::get_super_and_subscript(&mut latex, implicit_multiplication);
                    todo!();
                } else if command == ['f', 'r', 'a', 'c'] {
                    let mut params = vec![];
                    if !BracketType::is_opening_bracket(latex[0]) {
                        panic!("Invalid fraction");
                    }
                    let first = Self::get_first_parameter(&mut latex, implicit_multiplication);
                    params.push((first, BracketType::Curly));
                    params.push((
                        Self::get_first_parameter(&mut latex, implicit_multiplication),
                        BracketType::Curly,
                    ));
                    let fraction = Self {
                        name: vec!['f', 'r', 'a', 'c'],
                        parameters: params,
                    };
                    if latex.len() == 0 {
                        return fraction;
                    } else {
                        let other_ir = Self::latex_to_ir(latex, implicit_multiplication);
                        return Self {
                            name: vec!['*'],
                            parameters: vec![
                                (fraction, BracketType::Curly),
                                (other_ir, BracketType::Curly),
                            ],
                        };
                    }
                } else if command == ['s', 'q', 'r', 't'] {
                    let parameters = vec![(
                        Self::get_first_parameter(&mut latex, implicit_multiplication),
                        BracketType::Curly,
                    )];
                    let sqrt = Self {
                        name: command.to_vec(),
                        parameters,
                    };
                    return if latex.len() == 0 {
                        sqrt
                    } else {
                        let other_ir = Self::latex_to_ir(latex, implicit_multiplication);
                        Self {
                            name: command.to_vec(),
                            parameters: vec![
                                (sqrt, BracketType::None),
                                (other_ir, BracketType::None),
                            ],
                        }
                    };
                } else if command == ['s', 'i', 'n']
                    || command == ['c', 'o', 's']
                    || command == ['t', 'a', 'n']
                {
                    let parameters = vec![(
                        Self::get_first_parameter(&mut latex, implicit_multiplication),
                        BracketType::Curly,
                    )];
                    return Self {
                        name: command.to_vec(),
                        parameters,
                    };
                } else {
                    if latex.len() == 0 {
                        let mut slash_command = vec!['\\'];
                        slash_command.append(&mut command);
                        return Self {
                            name: slash_command,
                            parameters: vec![],
                        };
                    } else {
                        //TODO!
                        let command = Self {
                            parameters: vec![],
                            name: command,
                        };
                        return Self {
                            name: vec!['*'],
                            parameters: vec![
                                (command, BracketType::Round),
                                (
                                    Self::latex_to_ir(latex, implicit_multiplication),
                                    BracketType::Round,
                                ),
                            ],
                        };
                    }
                }
            } else if latex.contains(&'\\') {
                todo!();
            } else if latex.contains(&'{')
                || latex.contains(&'(')
                || latex.contains(&'[')
                || latex.contains(&'⟨')
            {
                if BracketType::is_opening_bracket(latex[0])
                    && BracketType::is_closing_bracket(latex[latex.len() - 1])
                {
                    latex.remove(0);
                    latex.remove(latex.len() - 1);
                    return Self::latex_to_ir(latex, implicit_multiplication);
                }
                todo!()
            } else if latex.iter().any(|char| char.is_numeric()) {
                if latex.iter().any(|char| !char.is_numeric()) {
                    todo!()
                } else {
                    return IR {
                        name: latex,
                        parameters: vec![],
                    };
                }
            } else if implicit_multiplication {
                let letters = latex
                    .into_iter()
                    .map(|c| {
                        (
                            Self {
                                name: vec![c],
                                parameters: vec![],
                            },
                            BracketType::None,
                        )
                    })
                    .collect();
                return Self {
                    name: vec!['*'],
                    parameters: letters,
                };
            } else {
                return IR {
                    name: latex,
                    parameters: vec![],
                };
            }
        }
    }
    pub fn ir_to_latex(mut self, implicit_multiplication: bool) -> Vec<char> {
        let name = self.name.clone();
        let mut return_data = vec![];
        match name[..] {
            ['+'] | ['-'] | ['*'] => {
                return_data.push(self.parameters[0].1.opening_bracket());
                let closing_bracket = self.parameters[0].1.closing_bracket();
                return_data.append(&mut Self::ir_to_latex(
                    self.parameters.remove(0).0,
                    implicit_multiplication,
                ));
                return_data.push(closing_bracket);
                while self.parameters.len() > 0 {
                    return_data.push(self.name[0]); // The operator
                    return_data.push(self.parameters[0].1.opening_bracket());
                    let closing_bracket = self.parameters[0].1.closing_bracket();
                    return_data.append(&mut Self::ir_to_latex(
                        self.parameters.remove(0).0,
                        implicit_multiplication,
                    ));
                    return_data.push(closing_bracket);
                }
            }
            ['^'] => {
                if self.parameters.len() != 2 {
                    panic!("Invalid power, not two parameters");
                }
                let opening_bracket = self.parameters[0].1.opening_bracket();
                let closing_bracket = self.parameters[0].1.closing_bracket();
                let mut data = vec![opening_bracket];
                data.append(
                    &mut self
                        .parameters
                        .remove(0)
                        .0
                        .ir_to_latex(implicit_multiplication),
                );
                data.push(closing_bracket);
                let opening_bracket = self.parameters[0].1.opening_bracket();
                let closing_bracket = self.parameters[0].1.closing_bracket();
                data.append(&mut self.name);
                data.push(opening_bracket);
                data.append(
                    &mut self
                        .parameters
                        .remove(0)
                        .0
                        .ir_to_latex(implicit_multiplication),
                );
                data.push(closing_bracket);
                return data;
            }
            ['\\', 'f', 'r', 'a', 'c'] => {
                if self.parameters.len() != 2 {
                    panic!("Invalid power, not two parameters");
                }
                let opening_bracket = self.parameters[0].1.opening_bracket();
                let closing_bracket = self.parameters[0].1.closing_bracket();
                let mut data = self.name;
                data.push(opening_bracket);
                data.append(
                    &mut self
                        .parameters
                        .remove(0)
                        .0
                        .ir_to_latex(implicit_multiplication),
                );
                data.push(closing_bracket);
                let opening_bracket = self.parameters[0].1.opening_bracket();
                let closing_bracket = self.parameters[0].1.closing_bracket();
                data.push(opening_bracket);
                data.append(
                    &mut self
                        .parameters
                        .remove(0)
                        .0
                        .ir_to_latex(implicit_multiplication),
                );
                data.push(closing_bracket);
                return data;
            }
            ['\\', 'i', 'n', 'v'] => {
                if self.parameters.len() == 1 {
                    let mut result = vec!['-'];
                    result.push(self.parameters[0].1.opening_bracket());
                    let closing_bracket = self.parameters[0].1.closing_bracket();
                    result.append(
                        &mut self
                            .parameters
                            .remove(0)
                            .0
                            .ir_to_latex(implicit_multiplication),
                    );
                    result.push(closing_bracket);
                    return result;
                }
                panic!();
            }
            _ => {
                if self.parameters.len() == 0 {
                    return self.name;
                } else {
                    let mut string = self.name.into_iter().collect::<Vec<_>>();
                    for parameter in self.parameters {
                        string.push(parameter.1.opening_bracket());
                        string.append(&mut Self::ir_to_latex(parameter.0, implicit_multiplication));
                        string.push(parameter.1.closing_bracket())
                    }
                    return string;
                }
            }
        }
        return return_data;
    }
    pub fn ir_to_equation(mut self) -> Equation {
        let name = self.name.clone();
        match name[..] {
            ['+'] => {
                return Equation::Addition(
                    self.parameters
                        .into_iter()
                        .map(|param| param.0.ir_to_equation())
                        .collect::<Vec<_>>(),
                );
            }
            ['-'] => {
                return Equation::Subtraction(
                    self.parameters
                        .into_iter()
                        .map(|param| param.0.ir_to_equation())
                        .collect::<Vec<_>>(),
                );
            }
            ['*'] => {
                return Equation::Multiplication(
                    self.parameters
                        .into_iter()
                        .map(|param| param.0.ir_to_equation())
                        .collect::<Vec<_>>(),
                );
            }
            ['/'] => {
                return if self.parameters.len() != 2 {
                    let actual_division = Equation::Division(Box::new((
                        self.parameters.remove(0).0.ir_to_equation(),
                        self.parameters.remove(0).0.ir_to_equation(),
                    )));
                    let mut params = Vec::from([actual_division]);
                    params.append(
                        &mut self
                            .parameters
                            .into_iter()
                            .map(|param| param.0.ir_to_equation())
                            .collect::<Vec<_>>(),
                    );
                    Equation::Multiplication(params)
                } else {
                    Equation::Division(Box::new((
                        self.parameters.remove(0).0.ir_to_equation(),
                        self.parameters.remove(0).0.ir_to_equation(),
                    )))
                }
            }
            ['^'] => {
                return if self.parameters.len() != 2 {
                    let actual_power = Equation::Power(Box::new((
                        self.parameters.remove(0).0.ir_to_equation(),
                        self.parameters.remove(0).0.ir_to_equation(),
                    )));
                    let mut params = Vec::from([actual_power]);
                    params.append(
                        &mut self
                            .parameters
                            .into_iter()
                            .map(|param| param.0.ir_to_equation())
                            .collect::<Vec<_>>(),
                    );
                    Equation::Multiplication(params)
                } else {
                    Equation::Power(Box::new((
                        self.parameters.remove(0).0.ir_to_equation(),
                        self.parameters.remove(0).0.ir_to_equation(),
                    )))
                }
            }
            ['s', 'q', 'r', 't'] => {
                if self.parameters.len() == 1 {
                    return Equation::Power(Box::new((
                        self.parameters.remove(0).0.ir_to_equation(),
                        Equation::Variable(Variable::Rational((1, 2))),
                    )));
                } else {
                    let sqrt = Equation::Power(Box::new((
                        self.parameters.remove(0).0.ir_to_equation(),
                        Equation::Variable(Variable::Rational((1, 2))),
                    )));
                    let mut params = Vec::from([sqrt]);
                    params.append(
                        &mut self
                            .parameters
                            .into_iter()
                            .map(|param| param.0.ir_to_equation())
                            .collect::<Vec<_>>(),
                    );
                    return Equation::Multiplication(params);
                }
            }
            ['f', 'r', 'a', 'c'] => {
                if self.parameters.len() == 2 {
                    let first = self.parameters.remove(0).0.ir_to_equation();
                    let second = self.parameters.remove(0).0.ir_to_equation();

                    return Equation::Division(Box::new((first, second)));
                } else {
                    let frac = Equation::Division(Box::new((
                        self.parameters.remove(0).0.ir_to_equation(),
                        self.parameters.remove(0).0.ir_to_equation(),
                    )));
                    let mut params = Vec::from([frac]);
                    params.append(
                        &mut self
                            .parameters
                            .into_iter()
                            .map(|param| param.0.ir_to_equation())
                            .collect::<Vec<_>>(),
                    );
                    return Equation::Multiplication(params);
                }
            }
            ['s', 'i', 'n'] | ['c', 'o', 's'] | ['t', 'a', 'n'] => {
                return if self.parameters.len() == 1 {
                    let param = self.parameters.remove(0).0.ir_to_equation();
                    match name[..] {
                        ['s', 'i', 'n'] => return Equation::Sin(Box::new(param)),
                        ['c', 'o', 's'] => return Equation::Cos(Box::new(param)),
                        ['t', 'a', 'n'] => {
                            return Equation::Division(Box::new((
                                Equation::Sin(Box::new(param.clone())),
                                Equation::Cos(Box::new(param)),
                            )))
                        }
                        _ => {
                            panic!()
                        }
                    }
                } else {
                    let param = self.parameters.remove(0).0.ir_to_equation();
                    let gonio = match name[..] {
                        ['s', 'i', 'n'] => Equation::Sin(Box::new(param)),
                        ['c', 'o', 's'] => Equation::Cos(Box::new(param)),
                        ['t', 'a', 'n'] => Equation::Division(Box::new((
                            Equation::Sin(Box::new(param.clone())),
                            Equation::Cos(Box::new(param)),
                        ))),
                        _ => {
                            panic!()
                        }
                    };
                    let mut params = Vec::from([gonio]);
                    params.append(
                        &mut self
                            .parameters
                            .into_iter()
                            .map(|param| param.0.ir_to_equation())
                            .collect::<Vec<_>>(),
                    );
                    Equation::Multiplication(params)
                }
            }
            ['\\', 'i', 'n', 'v'] => {
                Equation::Negative(Box::new(self.parameters.remove(0).0.ir_to_equation()))
            }
            _ => {
                if self.parameters.len() == 0 {
                    let is_numeric = self.name.iter().all(|char| char.is_numeric());
                    let expression = self.name.into_iter().collect::<String>();
                    return if is_numeric {
                        Equation::Variable(Variable::Integer(expression.parse::<i64>().unwrap()))
                    } else {
                        Equation::Variable(Variable::Letter(expression))
                    };
                } else {
                    todo!();
                }
            }
        }
    }
    pub fn equation_to_ir(equation: Equation) -> Self {
        match equation {
            Equation::Variable(variable) => {
                return match variable {
                    Variable::Letter(letter) => IR {
                        name: letter.chars().collect::<Vec<char>>(),
                        parameters: vec![],
                    },
                    Variable::Integer(integer) => IR {
                        name: integer.to_string().chars().collect::<Vec<char>>(),
                        parameters: vec![],
                    },
                    Variable::Vector(vector) => IR {
                        name: format!("\\vec{{{}}}", vector)
                            .chars()
                            .collect::<Vec<char>>(),
                        parameters: vec![],
                    },
                    Variable::Rational(ratio) => IR {
                        name: vec!['\\', 'f', 'r', 'a', 'c'],
                        parameters: vec![
                            (
                                Self::equation_to_ir(Equation::Variable(Variable::Integer(
                                    ratio.0,
                                ))),
                                BracketType::Curly,
                            ),
                            (
                                Self::equation_to_ir(Equation::Variable(Variable::Integer(
                                    ratio.1,
                                ))),
                                BracketType::Curly,
                            ),
                        ],
                    },
                    Variable::Constant(constant) => match constant {
                        Constant::PI => IR {
                            name: vec!['\\', 'p', 'i'],
                            parameters: vec![],
                        },
                        Constant::E => IR {
                            name: vec!['e'],
                            parameters: vec![],
                        },
                    },
                }
            }
            Equation::Multiplication(eq) => {
                return IR {
                    name: vec!['*'],
                    parameters: eq
                        .into_iter()
                        .map(|subeq| (Self::equation_to_ir(subeq), BracketType::Round))
                        .collect(),
                }
            }
            Equation::Power(data) => {
                let (lower, upper) = *data;
                IR {
                    name: vec!['^'],
                    parameters: vec![
                        (Self::equation_to_ir(lower), BracketType::Round),
                        (Self::equation_to_ir(upper), BracketType::Curly),
                    ],
                }
            }
            Equation::Addition(eqs) => {
                return IR {
                    name: vec!['+'],
                    parameters: eqs
                        .into_iter()
                        .map(|eq| (Self::equation_to_ir(eq), BracketType::Round))
                        .collect(),
                }
            }
            Equation::Subtraction(eqs) => {
                return IR {
                    name: vec!['-'],
                    parameters: eqs
                        .into_iter()
                        .map(|eq| (Self::equation_to_ir(eq), BracketType::Round))
                        .collect(),
                }
            }
            Equation::Division(div) => {
                let (lhs, rhs) = *div;
                return IR {
                    name: vec!['\\', 'f', 'r', 'a', 'c'],
                    parameters: vec![
                        (Self::equation_to_ir(lhs), BracketType::Curly),
                        (Self::equation_to_ir(rhs), BracketType::Curly),
                    ],
                };
            }
            Equation::Cos(cos) => {
                return IR {
                    name: vec!['\\', 'c', 'o', 's'],
                    parameters: vec![(Self::equation_to_ir(*cos), BracketType::Round)],
                }
            }
            Equation::Sin(sin) => {
                return IR {
                    name: vec!['\\', 's', 'i', 'n'],
                    parameters: vec![(Self::equation_to_ir(*sin), BracketType::Round)],
                }
            }
            Equation::Negative(core) => {
                return IR {
                    name: vec!['\\', 'i', 'n', 'v'],
                    parameters: vec![(Self::equation_to_ir(*core), BracketType::Round)],
                }
            }
            _ => {
                todo!()
            }
        }
    }
    ///Checks for the operators within the latex with the highest priority in the top level
    fn get_operators_in_top_level_from_latex(
        latex: &Vec<char>,
        implicit_multiplication: bool,
    ) -> TopLevelOperators {
        let mut depth = 0;
        let mut powers = vec![];
        let mut multiplications_and_divisions = vec![];
        let mut additions_and_subtractions = vec![];
        for (i, char) in latex.iter().enumerate() {
            if char == &'{' || char == &'(' || char == &'[' {
                depth += 1;
            } else if char == &'}' || char == &')' || char == &']' {
                depth -= 1;
            } else if depth == 0 {
                match char {
                    '+' | '-' => {
                        additions_and_subtractions.push(i);
                    }
                    '*' | '/' => {
                        multiplications_and_divisions.push(i);
                    }
                    '^' => {
                        let is_power = Self::check_if_caret_is_power(latex, i);
                        let is_top_level =
                            Self::check_if_power_is_top_level(latex, i, implicit_multiplication);
                        if is_power && is_top_level {
                            powers.push(i);
                        }
                    }
                    _ => {}
                }
            }
        }
        return TopLevelOperators {
            powers,
            multiplications_and_divisions,
            additions_and_subtractions,
        };
    }
    ///Because the ^ character is ambiguous in latex between powers and superscript, this has to be checked
    fn check_if_caret_is_power(latex: &Vec<char>, caret: usize) -> bool {
        let mut chars_until_command_start = vec![];
        for i in (0..caret).rev() {
            if latex[i] != '\\' {
                chars_until_command_start.push(latex[i]);
            } else {
                break;
            }
        }
        chars_until_command_start.reverse();
        if chars_until_command_start.contains(&'{') {
            let position = unsafe {
                chars_until_command_start
                    .iter()
                    .enumerate()
                    .find(|&char| char.1 == &'{')
                    .unwrap_unchecked()
                    .0
            };
            if position > 0 && chars_until_command_start[position - 1] != '_' {
                return true;
            }
            if chars_until_command_start[0..position].contains(&'{') {
                return true;
            }
        };
        let subscript_position = chars_until_command_start
            .iter()
            .enumerate()
            .find(|&char| char.1 == &'_');
        let command = if let Some(pos) = subscript_position {
            chars_until_command_start[0..pos.0]
                .iter()
                .collect::<String>()
        } else {
            chars_until_command_start.into_iter().collect::<String>()
        };
        println!("{}", command);
        if &*command == "int" {
            return false;
        }
        return true;
    }
    //A power in a power is not a top level operator, this function checks whether that is the case
    fn check_if_power_is_top_level(
        latex: &Vec<char>,
        caret: usize,
        implicit_multiplication: bool,
    ) -> bool {
        let mut i = caret - 1;
        while i > 0 {
            if latex[i] == '^' {
                let mut part_between = latex[i..caret].to_vec();
                part_between.remove(0);
                if part_between.len() == 1 {
                    return false;
                }
                return !Self::check_if_part_is_single_expression(
                    part_between,
                    implicit_multiplication,
                );
            }
            i -= 1;
        }
        return true;
    }
    ///Checks if a part inbetween two carets is a single expresion
    pub fn check_if_part_is_single_expression(
        part: Vec<char>,
        implicit_multiplication: bool,
    ) -> bool {
        if Self::calculate_depth_difference(&part) != 0 {
            return false;
        } else if BracketType::is_opening_bracket(part[0])
            && BracketType::is_closing_bracket(part[0])
        {
            return true;
        }
        if !implicit_multiplication {
            return false;
        }
        for char in part.iter() {
            if !char.is_alphabetic() {
                return false;
            }
        }
        return true;
    }
    //Requires latex to start with either _ or ^, otherwise, will return only None
    pub fn get_super_and_subscript(
        latex: &mut Vec<char>,
        implicit_multiplication: bool,
    ) -> (Option<Vec<char>>, Option<Vec<char>>) {
        let (mut superscript, mut subscript) = (None, None);
        for _ in 0..1 {
            match latex[0] {
                '_' => {
                    latex.remove(0);
                    //let no_brackets = latex[0] != '{';
                    let no_brackets = !BracketType::is_opening_bracket(latex[0]);
                    let mut depth = if no_brackets { 1 } else { 0 };
                    if BracketType::is_opening_bracket(latex[0]) {
                        latex.remove(0);
                    }
                    let mut subscript_buffer = vec![];
                    if !no_brackets {
                        while depth > 0 {
                            let next = latex.remove(0);
                            if BracketType::is_opening_bracket(next) {
                                depth += 1;
                            } else if BracketType::is_closing_bracket(next) {
                                depth -= 1;
                            }
                            if depth != 0 || no_brackets {
                                subscript_buffer.push(next);
                            } else {
                                break;
                            }
                        }
                    } else {
                        if !implicit_multiplication {
                            subscript_buffer.push(latex.remove(0));
                        } else {
                            todo!() //No brackets but implicit multiplication
                        }
                    }
                    subscript = Some(subscript_buffer);
                }
                '^' => {
                    latex.remove(0);
                    let no_brackets = BracketType::is_opening_bracket(latex[0]);
                    let mut depth = if no_brackets { 1 } else { 0 };
                    if BracketType::is_opening_bracket(latex[0]) {
                        latex.remove(0);
                    }
                    let mut superscript_buffer = vec![];
                    if !no_brackets {
                        while depth > 0 {
                            let next = latex.remove(0);
                            if BracketType::is_opening_bracket(next) {
                                depth += 1;
                            } else if BracketType::is_closing_bracket(next) {
                                depth -= 1;
                            }
                            if depth != 0 || no_brackets {
                                superscript_buffer.push(next);
                            } else {
                                break;
                            }
                        }
                    } else {
                        todo!() //NOBRACKETS
                    }
                    superscript = Some(superscript_buffer);
                }
                _ => {}
            }
        }
        return (superscript, subscript);
    }
    pub fn calculate_depth_difference(latex: &Vec<char>) -> i32 {
        let mut depth_diff = 0;
        for char in latex.iter() {
            if BracketType::is_opening_bracket(*char) {
                depth_diff += 1
            }
            if BracketType::is_closing_bracket(*char) {
                depth_diff -= 1;
            }
        }
        return depth_diff;
    }
    ///Get the first parameter (brackets required)
    pub fn get_first_parameter(latex: &mut Vec<char>, implicit_multiplication: bool) -> Self {
        let bracket_type = BracketType::get_opening_bracket_type(latex.remove(0));
        let mut parameter = vec![];
        let mut depth = 1;
        while depth > 0 {
            if BracketType::is_opening_bracket(latex[0]) {
                depth += 1;
            } else if BracketType::is_closing_bracket(latex[0]) {
                depth -= 1;
            }
            parameter.push(latex.remove(0));
        }
        parameter.remove(parameter.len() - 1);
        return Self::latex_to_ir(parameter, implicit_multiplication);
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
    pub fn opening_bracket(&self) -> char {
        return match self {
            Self::None => ' ',
            Self::Angle => '⟨',
            Self::Curly => '{',
            Self::Square => '[',
            Self::Round => '(',
        };
    }
    pub fn closing_bracket(&self) -> char {
        return match self {
            BracketType::None => ' ',
            BracketType::Curly => '}',
            BracketType::Square => ']',
            BracketType::Round => ')',
            BracketType::Angle => '⟩',
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
struct TopLevelOperators {
    powers: Vec<usize>,
    multiplications_and_divisions: Vec<usize>,
    additions_and_subtractions: Vec<usize>,
}
impl TopLevelOperators {
    pub fn any(&self) -> bool {
        return self.powers.len() > 0
            || self.multiplications_and_divisions.len() > 0
            || self.additions_and_subtractions.len() > 0;
    }
}
#[cfg(test)]
mod test {
    #[test]
    fn test_check_if_caret_is_power() {
        assert_eq!(
            super::IR::check_if_caret_is_power(&"\\int^10{a}{b}".chars().collect::<Vec<char>>(), 4),
            false
        );
        assert_eq!(
            super::IR::check_if_caret_is_power(
                &"\\frac{a}{b}^10".chars().collect::<Vec<char>>(),
                11
            ),
            true
        );
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
