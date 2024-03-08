use crate::math::{Equation, Variable};

impl Equation {
    pub fn differentiate(self: &Equation, differentiate_to: &Variable) -> Equation {
        match self {
            Equation::Variable(variable) => {
                return if variable == differentiate_to {
                    Equation::Variable(Variable::Integer(1))
                } else {
                    Equation::Variable(Variable::Integer(0))
                }
            }
            Equation::Negative(negative) => {
                return Equation::Negative(Box::new(negative.differentiate(differentiate_to)))
            }
            Equation::Addition(addition) => {
                return Equation::Addition(
                    addition
                        .iter()
                        .map(|x| x.differentiate(&differentiate_to))
                        .collect::<Vec<_>>(),
                )
            }
            Equation::Multiplication(multiplication) => {
                return Equation::Addition(
                    multiplication
                        .iter()
                        .map(|x| {
                            let mut multiplication_new: Vec<Equation> = multiplication.clone();
                            multiplication_new.remove(
                                multiplication_new
                                    .iter()
                                    .position(|y| y == x)
                                    .expect("This shouldn't happen"),
                            );
                            multiplication_new.push(x.differentiate(differentiate_to));
                            Equation::Multiplication(multiplication_new)
                        })
                        .collect::<Vec<_>>(),
                )
            }
            Equation::Division(division) => {
                let numerator = Equation::Addition(vec![
                    Equation::Multiplication(vec![
                        division.1.clone(),
                        division.0.differentiate(differentiate_to),
                    ]),
                    Equation::Negative(Box::new(Equation::Multiplication(vec![
                        division.0.clone(),
                        division.1.differentiate(differentiate_to),
                    ]))),
                ]);
                let denominator = Equation::Power(Box::new((
                    division.1.clone(),
                    Equation::Variable(Variable::Integer(2)),
                )));
                return Equation::Division(Box::new((numerator, denominator)));
            }
            Equation::Power(power) => return differentiate_power(power, differentiate_to),
            Equation::Ln(ln) => {
                if ln.clone().simplify_until_complete() == Equation::Variable(Variable::Integer(0))
                {
                    //TODO:
                    //this can probably be done better
                    return Equation::Variable(Variable::Integer(0));
                }
                return Equation::Division(Box::new((
                    ln.differentiate(differentiate_to),
                    (**ln).clone(),
                )));
            }
            Equation::Sin(sin) => {
                return Equation::Multiplication(vec![
                    sin.differentiate(differentiate_to),
                    Equation::Cos(sin.clone()),
                ])
            }
            Equation::Cos(sin) => {
                return Equation::Negative(Box::new(Equation::Multiplication(vec![
                    sin.differentiate(differentiate_to),
                    Equation::Sin(sin.clone()),
                ])))
            }
            Equation::Equals(equals) => {
                return Equation::Equals(Box::new((
                    equals.0.differentiate(differentiate_to),
                    equals.1.differentiate(differentiate_to),
                )))
            }
            Equation::Abs(abs) => {
                return Equation::Division(Box::new((
                    Equation::Multiplication(vec![
                        *abs.clone(),
                        abs.differentiate(differentiate_to),
                    ]),
                    Equation::Abs(abs.clone()),
                )))
            }
            Equation::Derivative(_) => {
                panic!("Cannot differentiate derivative")
            }
        }
    }
}

fn differentiate_power(power: &Box<(Equation, Equation)>, differentiate_to: &Variable) -> Equation {
    let first_term = Equation::Power(Box::new((
        power.0.clone(),
        Equation::Addition(vec![
            power.1.clone(),
            Equation::Variable(Variable::Integer(-1)),
        ]),
    )));
    let g_f_accent = Equation::Multiplication(vec![
        power.1.clone(),
        power.0.differentiate(differentiate_to),
    ]);
    let f_log_g_accent = Equation::Multiplication(vec![
        power.0.clone(),
        Equation::Ln(Box::new(power.0.clone())),
        power.1.differentiate(differentiate_to),
    ]);
    let second_term = Equation::Addition(vec![g_f_accent, f_log_g_accent]);
    return Equation::Multiplication(vec![first_term, second_term]);
}
