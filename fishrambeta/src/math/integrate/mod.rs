use num_rational::Rational64;

use super::{Equation, Variable};

impl Equation {
    pub fn integrate(&self, integrate_to: &Variable) -> Equation {
        let mut equation_to_integrate: Equation = (*self).clone().simplify();
        let fixed_terms = equation_to_integrate.get_factors();
        let mut integrated_equation = Vec::new();

        for fixed_term in fixed_terms.into_iter() {
            println!(
                "{} has factor? {} is constant? {}",
                fixed_term,
                equation_to_integrate.has_factor(&fixed_term),
                fixed_term.term_is_constant(integrate_to)
            );
            if equation_to_integrate.has_factor(&fixed_term)
                && fixed_term.term_is_constant(integrate_to)
            {
                equation_to_integrate = equation_to_integrate.remove_factor(&fixed_term).simplify();
                integrated_equation.push(fixed_term);
            }
        }

        integrated_equation.push(equation_to_integrate.do_integration(integrate_to));

        return Equation::Multiplication(integrated_equation);
    }

    fn do_integration(&self, integrate_to: &Variable) -> Equation {
        println!("Hard integrating: {}", self);
        match self {
            Equation::Addition(addition) => {
                return Equation::Addition(
                    addition.iter().map(|x| x.integrate(integrate_to)).collect(),
                );
            }
            Equation::Variable(Variable::Integer(i)) => {
                return Equation::Multiplication(vec![
                    Equation::Variable(Variable::Integer(*i)),
                    Equation::Variable(integrate_to.clone()),
                ])
            }
            Equation::Variable(Variable::Rational(r)) => {
                return Equation::Multiplication(vec![
                    Equation::Variable(Variable::Rational(*r)),
                    Equation::Variable(integrate_to.clone()),
                ])
            }
            Equation::Variable(v) if v == integrate_to => {
                return Equation::Multiplication(vec![
                    Equation::Variable(Variable::Rational(Rational64::new(1, 2))),
                    Equation::Power(Box::new((
                        Equation::Variable(integrate_to.clone()),
                        Equation::Variable(Variable::Integer(2)),
                    ))),
                ])
            }
            Equation::Power(box (b, n)) if *b == Equation::Variable(integrate_to.clone()) => {
                return Equation::Multiplication(vec![
                    Equation::Division(Box::new((
                        Equation::Variable(Variable::Integer(1)),
                        Equation::Addition(vec![
                            n.clone(),
                            Equation::Variable(Variable::Integer(1)),
                        ]),
                    ))),
                    Equation::Power(Box::new((
                        Equation::Variable(integrate_to.clone()),
                        Equation::Addition(vec![
                            n.clone(),
                            Equation::Variable(Variable::Integer(1)),
                        ]),
                    ))),
                ])
            }
            Equation::Sin(box x) if *x == Equation::Variable(integrate_to.clone()) => {
                return Equation::Negative(Box::new(Equation::Cos(Box::new(Equation::Variable(
                    integrate_to.clone(),
                )))))
            }
            Equation::Cos(box x) if *x == Equation::Variable(integrate_to.clone()) => {
                return Equation::Sin(Box::new(Equation::Variable(integrate_to.clone())))
            }
            _ => todo!(),
        }
    }

    fn term_is_constant(&self, integrate_to: &Variable) -> bool {
        match self {
            Equation::Addition(a) => return a.iter().all(|x| x.term_is_constant(integrate_to)),
            Equation::Multiplication(m) => {
                return m.iter().all(|x| x.term_is_constant(integrate_to))
            }
            Equation::Negative(n) => return n.term_is_constant(integrate_to),
            Equation::Division(d) => {
                return d.0.term_is_constant(integrate_to) && d.1.term_is_constant(integrate_to)
            }
            Equation::Power(p) => {
                return p.0.term_is_constant(integrate_to) && p.1.term_is_constant(integrate_to)
            }
            Equation::Sin(t) => return t.term_is_constant(integrate_to),
            Equation::Cos(t) => return t.term_is_constant(integrate_to),
            Equation::Ln(t) => return t.term_is_constant(integrate_to),
            Equation::Equals(_) => panic!("Equation containing = cannot be integrated"),
            Equation::Variable(v) => return v != integrate_to,
        }
    }
}
