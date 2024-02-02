use crate::math::{Constant, Equation, Variable};
use crate::parser::{BracketType, IR};
use num_rational::Rational64;

impl IR {
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
                return Equation::Addition(
                    self.parameters
                        .into_iter()
                        .enumerate()
                        .map(|(index, param)| {
                            if index == 0 {
                                param.0.ir_to_equation()
                            } else {
                                Equation::Negative(Box::new(param.0.ir_to_equation()))
                            }
                        })
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
            ['/'] | ['\\', 'f', 'r', 'a', 'c'] => {
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
            ['='] => {
                return Equation::Equals(Box::new((
                    self.parameters.remove(0).0.ir_to_equation(),
                    self.parameters.remove(0).0.ir_to_equation(),
                )))
            }
            ['\\', 's', 'q', 'r', 't'] => {
                return if self.parameters.len() == 1 {
                    Equation::Power(Box::new((
                        self.parameters.remove(0).0.ir_to_equation(),
                        Equation::Variable(Variable::Rational(Rational64::new(1, 2))),
                    )))
                } else {
                    let sqrt = self.parameters.remove(0).0.ir_to_equation();
                    let mut params = Vec::from([sqrt]);
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
            ['\\', 's', 'i', 'n']
            | ['\\', 'c', 'o', 's']
            | ['\\', 't', 'a', 'n']
            | ['\\', 'l', 'n']
            | ['\\', 'l', 'o', 'g'] => {
                return if self.parameters.len() == 1 {
                    let param = self.parameters.remove(0).0.ir_to_equation();
                    match name[..] {
                        ['\\', 's', 'i', 'n'] => return Equation::Sin(Box::new(param)),
                        ['\\', 'c', 'o', 's'] => return Equation::Cos(Box::new(param)),
                        ['\\', 'l', 'n'] => return Equation::Ln(Box::new(param)),
                        ['\\', 'l', 'o', 'g'] => {
                            return Equation::Division(Box::new((
                                Equation::Ln(Box::new(param)),
                                Equation::Ln(Box::new(Equation::Variable(Variable::Integer(10)))),
                            )))
                        }
                        ['\\', 't', 'a', 'n'] => {
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
                        ['\\', 's', 'i', 'n'] => Equation::Sin(Box::new(param)),
                        ['\\', 'c', 'o', 's'] => Equation::Cos(Box::new(param)),
                        ['\\', 't', 'a', 'n'] => Equation::Division(Box::new((
                            Equation::Sin(Box::new(param.clone())),
                            Equation::Cos(Box::new(param)),
                        ))),
                        ['\\', 'l', 'n'] => Equation::Ln(Box::new(param)),
                        ['\\', 'l', 'o', 'g'] => Equation::Division(Box::new((
                            param,
                            Equation::Ln(Box::new(Equation::Variable(Variable::Integer(10)))),
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
            ['i', 'n', 'v'] => {
                Equation::Negative(Box::new(self.parameters.remove(0).0.ir_to_equation()))
            }
            _ => {
                if self.parameters.len() == 0 {
                    let is_int = self.name.iter().all(|char| char.is_numeric());
                    let is_float = self
                        .name
                        .iter()
                        .all(|char| char.is_numeric() || char == &'.');

                    return if is_int {
                        let expression = self.name.into_iter().collect::<String>();
                        Equation::Variable(Variable::Integer(expression.parse::<i64>().unwrap()))
                    } else if is_float {
                        Self::parse_float(self.name)
                    } else {
                        let expression = self.name.into_iter().collect::<String>();
                        match expression.as_str() {
                            "e" => Equation::Variable(Variable::Constant(Constant::E)),
                            "\\pi" => Equation::Variable(Variable::Constant(Constant::PI)),
                            _ => Equation::Variable(Variable::Letter(expression)),
                        }
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
                                    *ratio.numer(),
                                ))),
                                BracketType::Curly,
                            ),
                            (
                                Self::equation_to_ir(Equation::Variable(Variable::Integer(
                                    *ratio.denom(),
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
            Equation::Ln(core) => {
                return IR {
                    name: vec!['\\', 'l', 'n'],
                    parameters: vec![(Self::equation_to_ir(*core), BracketType::Round)],
                }
            }
            Equation::Equals(core) => {
                let (lhs, rhs) = *core;
                return IR {
                    name: vec!['='],
                    parameters: vec![
                        (Self::equation_to_ir(lhs), BracketType::Curly),
                        (Self::equation_to_ir(rhs), BracketType::Curly),
                    ],
                };
            }
            _ => {
                todo!()
            }
        }
    }
    pub fn parse_float(float: Vec<char>) -> Equation {
        let period_pos = float.iter().position(|c| c == &'.').unwrap();
        let (int, dec) = float.split_at(period_pos);
        let int: String = int.into_iter().collect();
        let mut dec: String = dec.into_iter().collect();
        dec.remove(0);
        let denominator = 10i64.pow(dec.len() as u32);
        let nominator: i64 =
            int.parse::<i64>().unwrap() * denominator + dec.parse::<i64>().unwrap();
        return Equation::Variable(Variable::Rational(Rational64::new(nominator, denominator)));
    }
}
