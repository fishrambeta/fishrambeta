use std::collections::HashMap;

use super::Equation;
use super::Variable;

#[derive(Debug)]
pub struct LinearEquationSystem {
    unknown_variables: Vec<Variable>,
    augmented_matrix: Vec<Vec<Equation>>, // Each Vec<Equation> is a row
}

impl Equation {
    fn linear_part(self, variable: &Variable) -> Equation {
        match self {
            Equation::Negative(n) => Equation::Negative(Box::new(n.linear_part(variable))),
            Equation::Addition(addition) => Equation::Addition(
                addition
                    .into_iter()
                    .map(|x| x.linear_part(variable))
                    .collect(),
            ),
            Equation::Multiplication(mut m) => {
                match m
                    .iter()
                    .enumerate()
                    .find(|term| !term.1.is_constant(variable))
                {
                    Some((index, _term)) => {
                        let term = m.remove(index);
                        m.push(term.linear_part(variable));
                        Equation::Multiplication(m)
                    }
                    None => Equation::Variable(Variable::Integer(0)),
                }
            }
            Equation::Variable(v) if &v == variable => Equation::Variable(Variable::Integer(1)),
            _ => Equation::Variable(Variable::Integer(0)),
        }
    }

    fn constant_part(self, variables: &Vec<Variable>) -> Equation {
        if variables.iter().all(|variable| self.is_constant(variable)) {
            return self;
        }
        match self {
            Equation::Variable(v) => {
                if variables.contains(&v) {
                    Equation::Variable(Variable::Integer(0))
                } else {
                    Equation::Variable(v)
                }
            }
            Equation::Negative(n) => Equation::Negative(Box::new(n.constant_part(variables))),
            Equation::Addition(a) => Equation::Addition(
                a.into_iter()
                    .map(|term| term.constant_part(variables))
                    .collect(),
            ),
            Equation::Multiplication(m) => Equation::Multiplication(
                m.into_iter()
                    .map(|term| term.constant_part(variables))
                    .collect(),
            ),
            Equation::Division(d) => Equation::Division(Box::new((
                d.0.constant_part(variables),
                d.1.constant_part(variables),
            ))),
            Equation::Power(p) => Equation::Power(Box::new((
                p.0.constant_part(variables),
                p.1.constant_part(variables),
            ))),
            Equation::Ln(p) => Equation::Ln(Box::new(p.constant_part(variables))),
            Equation::Sin(p) => Equation::Sin(Box::new(p.constant_part(variables))),
            Equation::Cos(p) => Equation::Cos(Box::new(p.constant_part(variables))),
            Equation::Abs(p) => Equation::Abs(Box::new(p.constant_part(variables))),
            Equation::Equals(_) => panic!("Cannot get constant part of equals"),
            Equation::Derivative(_) => panic!("Cannot get constant part of derivative"),
        }
    }
}

impl LinearEquationSystem {
    pub fn from_equals_equations(
        equals_equations: Vec<Equation>,
        unknown_variables: Vec<Variable>,
    ) -> Self {
        let equations: Vec<_> = equals_equations
            .into_iter()
            .map(|equation| {
                if let Equation::Equals(equals) = equation {
                    (equals.0, equals.1)
                } else {
                    panic!("All equations must be equals type to construct system of equations")
                }
            })
            .collect();
        LinearEquationSystem::from_equation_tuples(equations, unknown_variables)
    }

    pub fn from_equation_tuples(
        equations: Vec<(Equation, Equation)>,
        unknown_variables: Vec<Variable>,
    ) -> Self {
        equations.iter().for_each(|(a, b)| {
            unknown_variables.iter().for_each(|v| {
                assert!(a.is_linear(v), "Equation wasn't linear");
                assert!(b.is_linear(v), "Equation wasn't linear")
            })
        });

        let augmented_matrix: Vec<Vec<_>> = equations
            .into_iter()
            .map(|(a, b)| Equation::Addition(vec![a, Equation::Negative(Box::new(b))]))
            .map(|equation| {
                unknown_variables
                    .iter()
                    .map(|variable| equation.clone().linear_part(variable).simplify(&mut None))
                    .chain(std::iter::once(Equation::Negative(Box::new(
                        equation
                            .clone()
                            .constant_part(&unknown_variables)
                            .simplify(&mut None),
                    ))))
                    .collect()
            })
            .collect();
        LinearEquationSystem {
            unknown_variables,
            augmented_matrix,
        }
    }

    pub fn solve(self) -> HashMap<Variable, Equation> {
        let mut a = self.augmented_matrix;

        let m = a.len();
        let n = a[0].len();

        for r in 0..m {
            for i in r + 1..m {
                let mut q = r + 1;
                while a[r][r] == Equation::Variable(Variable::Integer(0)) {
                    if q == a.len() {
                        panic!("Cannot solve system")
                    }
                    a.swap(r, q);
                    q += 1;
                }
                let multiplication_factor = Equation::Negative(Box::new(Equation::Division(
                    Box::new((a[i][r].clone(), a[r][r].clone())),
                )))
                .simplify(&mut None);
                for j in 1..n {
                    a[i][j] = Equation::Addition(vec![
                        a[i][j].clone(),
                        Equation::Multiplication(vec![
                            a[r][j].clone(),
                            multiplication_factor.clone(),
                        ]),
                    ])
                    .simplify(&mut None)
                    .simplify(&mut None)
                }
                a[i][r] = Equation::Variable(Variable::Integer(0));
            }
        }

        for r in (0..m).rev() {
            for i in 0..r {
                let multiplication_factor = Equation::Negative(Box::new(Equation::Division(
                    Box::new((a[i][r].clone(), a[r][r].clone())),
                )))
                .simplify(&mut None);
                for j in 1..n {
                    a[i][j] = Equation::Addition(vec![
                        a[i][j].clone(),
                        Equation::Multiplication(vec![
                            a[r][j].clone(),
                            multiplication_factor.clone(),
                        ]),
                    ])
                    .simplify(&mut None)
                    .simplify(&mut None)
                }
                a[i][r] = Equation::Variable(Variable::Integer(0));
            }
        }

        let results = a.into_iter().enumerate().map(|(i, mut row)| {
            Equation::Division(Box::new((row.remove(row.len() - 1), row.remove(i))))
                .simplify(&mut None)
        });
        self.unknown_variables.into_iter().zip(results).collect()
    }
}

#[cfg(test)]
mod tests {
    use num::Rational64;

    use super::*;
    use crate::tests::approx_equal;

    #[test]
    fn test_equation_system() {
        let x = Variable::Letter("x".to_string());
        let y = Variable::Letter("y".to_string());
        let z = Variable::Letter("z".to_string());
        let variables = vec![x.clone(), y.clone(), z.clone()];

        assert!(approx_equal(
            Equation::from_latex("4*x*\\sin(4)+y+c+9+\\sin(c)", false).linear_part(&x),
            Equation::from_latex("4*\\sin(4)", false)
        ));
        assert!(approx_equal(
            Equation::from_latex("4*x*\\sin(4)+y+c+9+\\sin(c)", false).linear_part(&y),
            Equation::from_latex("1", false)
        ));
        assert!(approx_equal(
            Equation::from_latex("4*x*\\sin(4)+y+c+9+\\sin(c)", false).constant_part(&variables),
            Equation::from_latex("c+9+\\sin(c)", false)
        ));

        assert!(approx_equal(
            Equation::from_latex("c*(5+3*x)+y", false).linear_part(&x),
            Equation::from_latex("3*c", false)
        ));
        assert!(approx_equal(
            Equation::from_latex("c*(5+3*x)+y", false).linear_part(&y),
            Equation::from_latex("1", false)
        ));
        assert!(approx_equal(
            Equation::from_latex("c*(5+3*x)+y", false).constant_part(&variables),
            Equation::from_latex("5*c", false)
        ));

        assert!(approx_equal(
            Equation::from_latex("(-x)+y-2", false).linear_part(&x),
            Equation::from_latex("-1", false)
        ));

        let equation_1 = Equation::from_latex("x+2*y+z=6", false);
        let equation_2 = Equation::from_latex("(-x)+y+z=3", false);
        let equation_3 = Equation::from_latex("(-x)+y=2", false);
        let system = LinearEquationSystem::from_equals_equations(
            vec![equation_1, equation_2, equation_3],
            variables.clone(),
        );
        println!("{:?}", system);
        let solution = system.solve();
        assert!(approx_equal(
            solution.get(&x).unwrap().clone(),
            Equation::Variable(Variable::Rational(Rational64::new(1, 3)))
        ));
        assert!(approx_equal(
            solution.get(&y).unwrap().clone(),
            Equation::Variable(Variable::Rational(Rational64::new(7, 3)))
        ));
        assert!(approx_equal(
            solution.get(&z).unwrap().clone(),
            Equation::Variable(Variable::Integer(1))
        ));

        let equation_1 = Equation::from_latex("2*y+z=6", false);
        let equation_2 = Equation::from_latex("(-x)+y+z=3", false);
        let equation_3 = Equation::from_latex("(-x)+y=2", false);
        let system = LinearEquationSystem::from_equals_equations(
            vec![equation_1, equation_2, equation_3],
            variables.clone(),
        );
        println!("{:?}", system);
        let solution = system.solve();
        //TODO correcte oplossing
        assert!(approx_equal(
            solution.get(&x).unwrap().clone(),
            Equation::Variable(Variable::Rational(Rational64::new(1, 2)))
        ));
        assert!(approx_equal(
            solution.get(&y).unwrap().clone(),
            Equation::Variable(Variable::Rational(Rational64::new(5, 2)))
        ));
        assert!(approx_equal(
            solution.get(&z).unwrap().clone(),
            Equation::Variable(Variable::Integer(1))
        ));
    }
}
