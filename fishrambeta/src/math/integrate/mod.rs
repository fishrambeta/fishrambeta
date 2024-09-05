use super::{steps::StepLogger, Equation, Variable};
use num_rational::Rational64;

mod bogointegrate;
mod rational;

impl Equation {
    pub fn integrate(
        &self,
        integrate_to: &Variable,
        step_logger: &mut Option<StepLogger>,
    ) -> Equation {
        if let Some(step_logger) = step_logger {
            step_logger.open_step(self.clone(), Some("Integrate"))
        }
        let mut equation_to_integrate: Equation = (*self).clone().simplify(step_logger);
        let fixed_terms = equation_to_integrate.get_factors();
        let mut integrated_equation = Vec::new();

        for fixed_term in fixed_terms {
            if equation_to_integrate.has_factor(&fixed_term) && fixed_term.is_constant(integrate_to)
            {
                equation_to_integrate = equation_to_integrate
                    .remove_factor(&fixed_term)
                    .simplify(step_logger);
                integrated_equation.push(fixed_term);
            }
        }

        #[allow(clippy::never_loop)]
        loop {
            println!("Equation to integrate: {equation_to_integrate}");
            if let Some(integrated_term) =
                equation_to_integrate.standard_integrals(integrate_to, step_logger)
            {
                integrated_equation.push(integrated_term);
                break;
            }

            if equation_to_integrate.is_rational(integrate_to) {
                equation_to_integrate.integrate_rational(integrate_to, step_logger);
                break;
            }

            integrated_equation.push(equation_to_integrate.bogointegrate(integrate_to));
            break;
        }
        let result = Equation::Multiplication(integrated_equation);
        if let Some(step_logger) = step_logger {
            step_logger.close_step(result.clone());
        }
        result
    }

    fn standard_integrals(
        &self,
        integrate_to: &Variable,
        step_logger: &mut Option<StepLogger>,
    ) -> Option<Equation> {
        if let Some(step_logger) = step_logger {
            step_logger.open_step(self.clone(), Some("Apply standard integral"))
        }
        let result = match self {
            Equation::Addition(addition) => {
                if let Some(step_logger) = step_logger {
                    step_logger.set_message("Use addition rule for derivatives")
                }
                Some(Equation::Addition(
                    addition
                        .iter()
                        .map(|x| x.integrate(integrate_to, step_logger))
                        .collect(),
                ))
            }
            Equation::Variable(Variable::Integer(i)) => Some(Equation::Multiplication(vec![
                Equation::Variable(Variable::Integer(*i)),
                Equation::Variable(integrate_to.clone()),
            ])),
            Equation::Variable(Variable::Rational(r)) => Some(Equation::Multiplication(vec![
                Equation::Variable(Variable::Rational(*r)),
                Equation::Variable(integrate_to.clone()),
            ])),
            Equation::Variable(v) if v == integrate_to => Some(Equation::Multiplication(vec![
                Equation::Variable(Variable::Rational(Rational64::new(1, 2))),
                Equation::Power(Box::new((
                    Equation::Variable(integrate_to.clone()),
                    Equation::Variable(Variable::Integer(2)),
                ))),
            ])),
            Equation::Power(ref power) if power.0 == Equation::Variable(integrate_to.clone()) => {
                Some(Equation::Multiplication(vec![
                    Equation::Division(Box::new((
                        Equation::Variable(Variable::Integer(1)),
                        Equation::Addition(vec![
                            power.1.clone(),
                            Equation::Variable(Variable::Integer(1)),
                        ]),
                    ))),
                    Equation::Power(Box::new((
                        Equation::Variable(integrate_to.clone()),
                        Equation::Addition(vec![
                            power.1.clone(),
                            Equation::Variable(Variable::Integer(1)),
                        ]),
                    ))),
                ]))
            }
            Equation::Sin(ref x) if **x == Equation::Variable(integrate_to.clone()) => {
                Some(Equation::Negative(Box::new(Equation::Cos(Box::new(
                    Equation::Variable(integrate_to.clone()),
                )))))
            }
            Equation::Cos(ref x) if **x == Equation::Variable(integrate_to.clone()) => Some(
                Equation::Sin(Box::new(Equation::Variable(integrate_to.clone()))),
            ),
            Equation::Arcsin(ref _x) => todo!(),
            Equation::Arccos(ref _x) => todo!(),
            Equation::Arctan(ref _x) => todo!(),
            _ => None,
        };
        if let Some(step_logger) = step_logger {
            match result {
                Some(ref result) => step_logger.close_step(result.clone()),
                None => step_logger.cancel_step(),
            }
        }
        result
    }
}
