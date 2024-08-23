use super::steps::StepLogger;
use crate::math::{Equation, Variable};

impl Equation {
    pub fn differentiate(
        self: &Equation,
        differentiate_to: &Variable,
        step_logger: &mut Option<StepLogger>,
    ) -> Equation {
        if let Some(step_logger) = step_logger {
            step_logger.open_step(self.clone(), Some("Differentiate"))
        }
        let derivative = match self {
            Equation::Variable(variable) => {
                if variable == differentiate_to {
                    Equation::Variable(Variable::Integer(1))
                } else {
                    Equation::Variable(Variable::Integer(0))
                }
            }
            Equation::Negative(negative) => Equation::Negative(Box::new(
                negative.differentiate(differentiate_to, step_logger),
            )),
            Equation::Addition(addition) => {
                if let Some(step_logger) = step_logger {
                    step_logger.set_message("Differentiate by applying the sum rule");
                }
                Equation::Addition(
                    addition
                        .iter()
                        .map(|x| x.differentiate(differentiate_to, step_logger))
                        .collect::<Vec<_>>(),
                )
            }
            Equation::Multiplication(multiplication) => Equation::Addition(
                multiplication
                    .iter()
                    .map(|x| {
                        if let Some(step_logger) = step_logger {
                            step_logger.set_message("Differentiate by applying the product rule")
                        }
                        let mut multiplication_new: Vec<Equation> = multiplication.clone();
                        multiplication_new.remove(
                            multiplication_new
                                .iter()
                                .position(|y| y == x)
                                .expect("This shouldn't happen"),
                        );
                        multiplication_new.push(x.differentiate(differentiate_to, step_logger));
                        Equation::Multiplication(multiplication_new).simplify(step_logger)
                    })
                    .collect::<Vec<_>>(),
            ),
            Equation::Division(division) => {
                if let Some(step_logger) = step_logger {
                    step_logger.set_message("Differentiate by applying the quotient rule")
                }
                let numerator = Equation::Addition(vec![
                    Equation::Multiplication(vec![
                        division.1.clone(),
                        division.0.differentiate(differentiate_to, step_logger),
                    ]),
                    Equation::Negative(Box::new(Equation::Multiplication(vec![
                        division.0.clone(),
                        division.1.differentiate(differentiate_to, step_logger),
                    ]))),
                ]);
                let denominator = Equation::Power(Box::new((
                    division.1.clone(),
                    Equation::Variable(Variable::Integer(2)),
                )));
                Equation::Division(Box::new((numerator, denominator)))
            }
            Equation::Power(power) => {
                if let Some(step_logger) = step_logger {
                    step_logger.set_message("Differentiate by applying the power rule")
                }
                differentiate_power(power, differentiate_to, step_logger).simplify(step_logger)
            }
            Equation::Ln(ln) => {
                if let Some(step_logger) = step_logger {
                    step_logger.set_message("Differentiate by using the chain rule")
                }
                if ln.clone().simplify_until_complete(&mut None)
                    == Equation::Variable(Variable::Integer(0))
                {
                    //TODO:
                    //this can probably be done better
                    Equation::Variable(Variable::Integer(0));
                }
                Equation::Division(Box::new((
                    ln.differentiate(differentiate_to, step_logger),
                    (**ln).clone(),
                )))
            }
            Equation::Sin(sin) => {
                if let Some(step_logger) = step_logger {
                    step_logger.set_message("Differentiate by using the chain rule")
                };
                Equation::Multiplication(vec![
                    sin.differentiate(differentiate_to, step_logger),
                    Equation::Cos(sin.clone()),
                ])
            }
            Equation::Cos(sin) => {
                if let Some(step_logger) = step_logger {
                    step_logger.set_message("Differentiate by using the chain rule")
                };
                Equation::Negative(Box::new(Equation::Multiplication(vec![
                    sin.differentiate(differentiate_to, step_logger),
                    Equation::Sin(sin.clone()),
                ])))
            }
            Equation::Equals(equals) => Equation::Equals(Box::new((
                equals.0.differentiate(differentiate_to, step_logger),
                equals.1.differentiate(differentiate_to, step_logger),
            ))),
            Equation::Abs(abs) => Equation::Division(Box::new((
                Equation::Multiplication(vec![
                    *abs.clone(),
                    abs.differentiate(differentiate_to, step_logger),
                ]),
                Equation::Abs(abs.clone()),
            ))),
            Equation::Derivative(_) => {
                panic!("Cannot differentiate derivative")
            }
        };
        if let Some(step_logger) = step_logger {
            step_logger.close_step(derivative.clone())
        }
        derivative
    }
}

fn differentiate_power(
    power: &(Equation, Equation),
    differentiate_to: &Variable,
    step_logger: &mut Option<StepLogger>,
) -> Equation {
    let first_term = Equation::Power(Box::new((
        power.0.clone(),
        Equation::Addition(vec![
            power.1.clone(),
            Equation::Variable(Variable::Integer(-1)),
        ]),
    )));
    let g_f_accent = Equation::Multiplication(vec![
        power.1.clone(),
        power.0.differentiate(differentiate_to, step_logger),
    ]);
    let f_log_g_accent = Equation::Multiplication(vec![
        power.0.clone(),
        Equation::Ln(Box::new(power.0.clone())),
        power.1.differentiate(differentiate_to, step_logger),
    ]);
    let second_term = Equation::Addition(vec![g_f_accent, f_log_g_accent]);
    Equation::Multiplication(vec![first_term, second_term])
}
