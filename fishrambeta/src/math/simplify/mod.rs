use crate::math::{
    steps::{
        helpers::{close_step, open_step},
        StepLogger,
    },
    Equation, Variable,
};
use std::collections::BTreeMap;

mod addition;
mod division;
mod logarithm;
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
            return numerical_part;
        }
        open_step(step_logger, &self, Some("Simplify"));
        let simplified = match self {
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
                Equation::Negative(negative) => (*negative).simplify(&mut None),
                Equation::Variable(Variable::Integer(0)) => {
                    Equation::Variable(Variable::Integer(0))
                }
                Equation::Variable(Variable::Integer(integer)) => {
                    Equation::Variable(Variable::Integer(-integer))
                }
                Equation::Variable(Variable::Rational(rational)) => {
                    Equation::Variable(Variable::Rational(-rational))
                }

                negative => Equation::Negative(Box::new(negative.simplify(&mut None))),
            },
            Equation::Addition(addition) => addition::simplify_addition(addition, &mut None),
            Equation::Multiplication(multiplication) => {
                multiplication::simplify_multiplication(multiplication, &mut None)
            }
            Equation::Division(division) => division::simplify_division(*division, &mut None),
            Equation::Power(power) => power::simplify_power(*power, &mut None),
            Equation::Ln(ln) => logarithm::simplify_ln(*ln, &mut None),
            Equation::Sin(sin) => Equation::Sin(Box::new(sin.simplify(&mut None))),
            Equation::Cos(cos) => Equation::Cos(Box::new(cos.simplify(&mut None))),
            Equation::Arcsin(sin) => Equation::Arcsin(Box::new(sin.simplify(&mut None))),
            Equation::Arccos(cos) => Equation::Arccos(Box::new(cos.simplify(&mut None))),
            Equation::Arctan(tan) => Equation::Arctan(Box::new(tan.simplify(&mut None))),
            Equation::Abs(abs) => Equation::Abs(Box::new(abs.simplify(&mut None))),
            Equation::Equals(equation) => Equation::Equals(Box::new((
                equation.0.simplify(&mut None),
                equation.1.simplify(&mut None),
            ))),
            Equation::Derivative(_) => {
                panic!("Derivative cannot be simplified")
            }
        };
        close_step(step_logger, &simplified);
        simplified
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
