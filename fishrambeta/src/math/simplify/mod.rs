use crate::math::{steps::StepLogger, Equation, Variable};
use std::collections::BTreeMap;

use super::steps::Step;

mod addition;
mod division;
mod multiplication;
mod power;

impl Equation {
    pub fn simplify_until_complete(self, step_logger: &mut Option<StepLogger>) -> Self {
        let mut equation = self.clone();
        let mut previous = self;
        for _ in 1..5 {
            equation = equation.simplify(step_logger);
            if equation == previous {
                break;
            }
            previous = equation.clone();
        }
        equation
    }

    pub fn simplify_until_complete_with_print(self, step_logger: &mut Option<StepLogger>) -> Self {
        let mut equation = self.clone();
        let mut previous = equation.to_latex();
        for i in 1..5 {
            equation = equation.simplify(step_logger);
            println!("{i}: {equation}, {equation:?}");
            if equation.to_latex() == previous {
                break;
            }
            previous = equation.to_latex();
        }
        equation
    }

    pub(super) fn simplify(self, step_logger: &mut Option<StepLogger>) -> Self {
        let calculated_wrapped = self.calculate_exact();
        if calculated_wrapped.is_some() {
            let calculated = calculated_wrapped.unwrap();
            let numerical_part = if calculated.is_integer() {
                Equation::Variable(Variable::Integer(calculated.to_integer()))
            } else {
                Equation::Variable(Variable::Rational(calculated))
            };
            if let Some(step_logger) = step_logger {
                step_logger.add_step(Step::new(
                    self,
                    numerical_part.clone(),
                    Some("Calculate part consisting of only numbers"),
                ))
            }
            return numerical_part;
        }
        match self {
            Equation::Variable(variable) => match variable {
                Variable::Rational(r) => {
                    if r.is_integer() {
                        Equation::Variable(Variable::Integer(r.to_integer()))
                    } else {
                        Equation::Variable(Variable::Rational(r))
                    }
                }
                variable => Equation::Variable(variable),
            },
            Equation::Negative(negative) => match *negative {
                Equation::Negative(negative) => (*negative).simplify(step_logger),
                Equation::Variable(Variable::Integer(0)) => {
                    Equation::Variable(Variable::Integer(0))
                }
                Equation::Variable(Variable::Integer(integer)) => {
                    Equation::Variable(Variable::Integer(-integer))
                }
                Equation::Variable(Variable::Rational(rational)) => {
                    Equation::Variable(Variable::Rational(-rational))
                }

                negative => Equation::Negative(Box::new(negative.simplify(step_logger))),
            },
            Equation::Addition(addition) => addition::simplify_addition(addition, step_logger),
            Equation::Multiplication(multiplication) => {
                multiplication::simplify_multiplication(multiplication, step_logger)
            }
            Equation::Division(division) => division::simplify_division(*division, step_logger),
            Equation::Power(power) => power::simplify_power(*power, step_logger),
            Equation::Ln(ln) => Equation::Ln(Box::new(ln.simplify(step_logger))),
            Equation::Sin(sin) => Equation::Sin(Box::new(sin.simplify(step_logger))),
            Equation::Cos(cos) => Equation::Cos(Box::new(cos.simplify(step_logger))),
            Equation::Abs(abs) => Equation::Abs(Box::new(abs.simplify(step_logger))),
            Equation::Equals(equation) => Equation::Equals(Box::new((
                equation.0.simplify(step_logger),
                equation.1.simplify(step_logger),
            ))),
            Equation::Derivative(_) => {
                panic!("Derivative cannot be simplified")
            }
        }
    }
}

pub struct EquationBTreeMap(BTreeMap<Equation, Vec<Equation>>);

impl EquationBTreeMap {
    pub fn new() -> EquationBTreeMap {
        EquationBTreeMap(BTreeMap::new())
    }
    pub fn insert_or_push(&mut self, term: Equation, equation: Equation) {
        self.0.entry(term).or_default().push(equation);
    }
}
