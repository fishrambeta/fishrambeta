mod calculate;
mod differentiate;
mod simplify;

///Represents a generic math object
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum Equation {
    Variable(Variable),
    Addition(Vec<Equation>),
    Subtraction(Vec<Equation>),
    Multiplication(Vec<Equation>),
    Division(Box<(Equation, Equation)>),
    Power(Box<(Equation, Equation)>),
    Ln(Box<Equation>),
    Equals(Box<(Equation, Equation)>),
}
///Represents a single number
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum Variable {
    Integer(i64),
    Rational((i64, i64)),
    Constant(Constant),
    Letter(String),
    Vector(String),
}
///Mathematical constants
#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum Constant {
    PI,
    E,
}

fn differentiate_power(power: &Box<(Equation, Equation)>, differentiate_to: &Variable) -> Equation {
    let first_term = Equation::Power(Box::new((
        power.0.clone(),
        Equation::Subtraction(vec![
            power.0.clone(),
            Equation::Variable(Variable::Integer(1)),
        ]),
    )));
    let g_f_accent = Equation::Multiplication(vec![
        power.0.clone(),
        power.1.differentiate(differentiate_to),
    ]);
    let f_log_g_accent = Equation::Multiplication(vec![
        power.0.clone(),
        Equation::Ln(Box::new(power.0.clone())),
        power.1.differentiate(differentiate_to),
    ]);
    let second_term = Equation::Addition(vec![g_f_accent, f_log_g_accent]);
    return Equation::Multiplication(vec![first_term, second_term]);
}
