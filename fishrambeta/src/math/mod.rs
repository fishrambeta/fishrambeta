use num_rational::Rational64;

mod calculate;
mod compare;
mod differentiate;
mod equation_system;
mod error_analysis;
mod factors;
mod function_types;
mod integrate;
mod multiply_by;
mod polynomial;
mod simplify;
pub mod steps;
mod taylor_series;

///Represents a generic math object
#[derive(Eq, PartialEq, Hash, Clone, Debug, Ord, PartialOrd)]
pub enum Equation {
    Variable(Variable),
    Negative(Box<Equation>),
    Addition(Vec<Equation>),
    Multiplication(Vec<Equation>),
    Division(Box<(Equation, Equation)>),
    Power(Box<(Equation, Equation)>),
    Ln(Box<Equation>),
    Equals(Box<(Equation, Equation)>),
    Sin(Box<Equation>),
    Cos(Box<Equation>),
    Arcsin(Box<Equation>),
    Arccos(Box<Equation>),
    Arctan(Box<Equation>),
    Abs(Box<Equation>),
    Derivative((Box<(Equation, Equation)>, bool)),
}
///Represents a single number
#[derive(Eq, PartialEq, Hash, Clone, Debug, Ord, PartialOrd)]
pub enum Variable {
    Integer(i64),
    Rational(Rational64),
    Constant(Constant),
    Letter(String),
    Vector(String),
}
///Mathematical constants
#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug, Ord, PartialOrd)]
pub enum Constant {
    PI,
    E,
}

impl Equation {
    fn get_number_or_none(&self) -> Option<Rational64> {
        match self {
            Equation::Variable(Variable::Integer(n)) => Some((*n).into()),
            Equation::Variable(Variable::Rational(r)) => Some(*r),
            _ => None,
        }
    }
    fn get_integer_or_none(&self) -> Option<i64> {
        match self {
            Equation::Variable(Variable::Integer(n)) => Some(*n),
            _ => None,
        }
    }
}
