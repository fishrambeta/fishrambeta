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
        let mut equation_to_integrate: Equation = (*self).clone().simplify(step_logger);
        let fixed_terms = equation_to_integrate.get_factors();
        let mut integrated_equation = Vec::new();

        for fixed_term in fixed_terms {
            if equation_to_integrate.has_factor(&fixed_term)
                && fixed_term.term_is_constant(integrate_to)
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
            if let Some(integrated_term) = equation_to_integrate.standard_integrals(integrate_to, step_logger) {
                integrated_equation.push(integrated_term);
                break;
            }

            if equation_to_integrate.is_rational_function(integrate_to) {
                equation_to_integrate.integrate_rational(integrate_to, step_logger);
                break;
            }

            integrated_equation.push(equation_to_integrate.bogointegrate(integrate_to));
            break;
        }

        Equation::Multiplication(integrated_equation)
    }

    fn standard_integrals(&self, integrate_to: &Variable, step_logger: &mut Option<StepLogger>) -> Option<Equation> {
        return match self {
            Equation::Addition(addition) => Some(Equation::Addition(
                addition
                    .iter()
                    .map(|x| x.integrate(integrate_to, step_logger))
                    .collect(),
            )),
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
            _ => None,
        };
    }

    pub fn term_is_constant(&self, integrate_to: &Variable) -> bool {
        match self {
            Equation::Addition(a) => a.iter().all(|x| x.term_is_constant(integrate_to)),
            Equation::Multiplication(m) => m.iter().all(|x| x.term_is_constant(integrate_to)),
            Equation::Negative(n) => n.term_is_constant(integrate_to),
            Equation::Division(d) => {
                d.0.term_is_constant(integrate_to) && d.1.term_is_constant(integrate_to)
            }
            Equation::Power(p) => {
                p.0.term_is_constant(integrate_to) && p.1.term_is_constant(integrate_to)
            }
            Equation::Sin(t) => t.term_is_constant(integrate_to),
            Equation::Cos(t) => t.term_is_constant(integrate_to),
            Equation::Ln(t) => t.term_is_constant(integrate_to),
            Equation::Equals(_) => panic!("Equation containing = cannot be integrated"),
            Equation::Variable(v) => v != integrate_to,
            Equation::Abs(a) => a.term_is_constant(integrate_to),
            Equation::Derivative(_) => {
                panic!("Derivative cannot be integrated")
            }
        }
    }

    fn is_polynomial(&self, integrate_to: &Variable) -> bool {
        match self {
            Equation::Addition(a) => return a.iter().all(|x| x.is_polynomial(integrate_to)),
            Equation::Power(p) => {
                (p.0 == Equation::Variable(integrate_to.clone()))
                    && (p.1.get_integer_or_none().is_some())
            }
            Equation::Negative(n) => n.is_polynomial(integrate_to),
            Equation::Variable(_) => true,
            Equation::Multiplication(m) => return m.iter().all(|x| x.is_polynomial(integrate_to)),
            _ => self.term_is_constant(integrate_to),
        }
    }

    fn is_rational_function(&self, integrate_to: &Variable) -> bool {
        self.term_is_rational_function(integrate_to, false)
    }

    fn term_is_rational_function(&self, integrate_to: &Variable, is_in_division: bool) -> bool {
        if self.is_polynomial(integrate_to) {
            return true;
        }
        if is_in_division {
            return self.is_polynomial(integrate_to);
        }

        match self {
            Equation::Addition(a) => {
                return a
                    .iter()
                    .all(|x| x.term_is_rational_function(integrate_to, is_in_division))
            }
            Equation::Multiplication(m) => {
                return m
                    .iter()
                    .all(|x| x.term_is_rational_function(integrate_to, is_in_division))
            }
            Equation::Division(d) => {
                d.0.term_is_rational_function(integrate_to, true)
                    && d.1.term_is_rational_function(integrate_to, true)
            }
            misc => misc.term_is_constant(integrate_to),
        }
    }
}
