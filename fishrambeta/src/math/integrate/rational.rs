use super::{Equation, Variable};
use std::iter;

/// Takes two polynomails a and b, and computes the division a/b and remainder a%b. Which it
/// returns in (a/b, a%b)
pub fn polynomial_division(a: Equation, b: Equation) -> (Equation, Equation) {
    todo!()
}

impl Equation {
    pub fn integrate_rational(self, integrate_to: Variable) -> Equation {
        let polynomial = Polynomial::from_equation(self, integrate_to);
        println!("{:?}", polynomial);
        println!("{}", polynomial.to_equation().simplify());
        todo!()
    }
}

#[derive(Debug, Clone)]
struct Polynomial {
    terms: Vec<Equation>,
    base: Variable,
}

impl Polynomial {
    /// Returns a polynomial with a value of 0
    fn zero(base: Variable) -> Polynomial {
        Polynomial {
            terms: vec![Equation::Variable(Variable::Integer(0))],
            base,
        }
    }

    /// Returns a polynomial with a value of 1
    fn one(base: Variable) -> Polynomial {
        Polynomial {
            terms: vec![Equation::Variable(Variable::Integer(1))],
            base,
        }
    }

    fn single_term_polynomial(term: Equation, exponent: usize, base: Variable) -> Polynomial {
        let mut terms: Vec<Equation> = vec![];
        for _ in 0..exponent {
            terms.push(Equation::Variable(Variable::Integer(0)))
        }
        terms.push(term);
        Polynomial { terms, base }
    }

    fn from_equation(x: Equation, base: Variable) -> Polynomial {
        if x.term_is_constant(&base) {
            return Polynomial {
                terms: vec![x],
                base,
            };
        }

        match x {
            Equation::Variable(v) => {
                if v == base {
                    return Polynomial::single_term_polynomial(
                        Equation::Variable(Variable::Integer(1)),
                        1,
                        base,
                    );
                } else {
                    unreachable!()
                }
            }
            Equation::Addition(a) => {
                let mut total = Polynomial::zero(base.clone());
                for polynomial_term in a
                    .into_iter()
                    .map(|x| Polynomial::from_equation(x, base.clone()))
                {
                    total = total + polynomial_term
                }
                total
            }
            Equation::Multiplication(m) => {
                let mut total = Polynomial::one(base.clone());
                for polynomial_term in m
                    .into_iter()
                    .map(|x| Polynomial::from_equation(x, base.clone()))
                {
                    total = total * polynomial_term
                }
                total
            }
            Equation::Power(p) => {
                if p.0 == Equation::Variable(base.clone()) {
                    if let Some(exponent) = p.1.get_integer_or_none() {
                        Polynomial::single_term_polynomial(
                            Equation::Variable(Variable::Integer(1)),
                            exponent as usize,
                            base,
                        )
                    } else {
                        unreachable!()
                    }
                } else {
                    unreachable!()
                }
            }

            _ => unreachable!(),
        }
    }

    fn to_equation(self) -> Equation {
        let mut total_equation: Vec<Equation> = Vec::new();
        for (exponent, equation) in self.terms.into_iter().enumerate() {
            let new_term = Equation::Multiplication(vec![
                equation,
                Equation::Power(Box::new((
                    Equation::Variable(self.base.clone()),
                    Equation::Variable(Variable::Integer(exponent.try_into().unwrap())),
                ))),
            ]);
            total_equation.push(new_term);
        }
        if total_equation.len() != 0 {
            Equation::Addition(total_equation)
        } else {
            Equation::Variable(Variable::Integer(0))
        }
    }
}

impl std::ops::Add for Polynomial {
    type Output = Self;

    fn add(self, other: Polynomial) -> Self::Output {
        //TODO this code must be wrong somehow
        println!("Called addition!!!!");
        if self.base != other.base {
            panic!("Bases must be the same to add polynomials")
        }

        let (longest, shortest) = if self.terms.len() > other.terms.len() {
            (self.terms, other.terms)
        } else {
            (other.terms, self.terms)
        };

        let mut new_polynomial_terms: Vec<Equation> = Vec::new();
        for (a, b) in longest.into_iter().zip(
            shortest
                .into_iter()
                .chain(iter::repeat(Equation::Variable(Variable::Integer(0)))),
        ) {
            new_polynomial_terms.push(Equation::Addition(vec![a, b]));
        }
        println!(
            "New polynomial temrs length: {}",
            new_polynomial_terms.len()
        );
        Polynomial {
            terms: new_polynomial_terms,
            base: self.base,
        }
    }
}

impl std::ops::Mul for Polynomial {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        println!("Called multiplication!!!!");

        if self.base != other.base {
            panic!("Bases must be the same to multiply polynomials")
        }

        let max_exponent = self.terms.len() * other.terms.len();
        let mut new_terms_as_vec_of_vecs: Vec<Vec<Equation>> =
            (0..=max_exponent).map(|_| Vec::new()).collect();

        for (i, a) in self.terms.iter().enumerate() {
            for (j, b) in other.terms.iter().enumerate() {
                let coefficient = Equation::Multiplication(vec![a.clone(), b.clone()]);
                let exponent = i + j;
                new_terms_as_vec_of_vecs[exponent].push(coefficient);
            }
        }

        let new_terms_as_vec_of_equations: Vec<Equation> = new_terms_as_vec_of_vecs
            .into_iter()
            .map(|x| {
                if x.len() != 0 {
                    Equation::Addition(x)
                } else {
                    Equation::Variable(Variable::Integer(0))
                }
            })
            .collect();
        Polynomial {
            terms: new_terms_as_vec_of_equations,
            base: self.base,
        }
    }
}
