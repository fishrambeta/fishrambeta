use super::{Equation, Variable};
use std::iter;

impl Equation {
    pub fn integrate_rational(self, integrate_to: &Variable) -> Equation {
        if let Equation::Division(d) = self {
            let a = Polynomial::from_equation(d.0, integrate_to.clone());
            let b = Polynomial::from_equation(d.1, integrate_to.clone());
            let ((new_a, new_b), remainder) = a.div_rational(b);

            println!("Hello");
            let integrated_remainder = remainder.to_equation().simplify().integrate(&integrate_to).simplify();
            println!("Integrated remainder: {}", integrated_remainder);
        } else {
            todo!()
        }
        todo!()
    }
}

#[derive(Debug, Clone)]
struct Polynomial {
    terms: Vec<Equation>,
    base: Variable,
}

impl Polynomial {
    fn simplify(self) -> Polynomial {
        let terms: Vec<Equation> = self.terms.into_iter().map(|x| x.simplify()).collect();
        let mut terms_new: Vec<Equation> = terms
            .into_iter()
            .rev()
            .skip_while(|x| *x == Equation::Variable(Variable::Integer(0)))
            .collect();
        terms_new.reverse();
        Polynomial {
            terms: terms_new,
            base: self.base,
        }
    }

    /// Returns a polynomial with a value of 0
    fn zero(base: Variable, order: i64) -> Polynomial {
        Polynomial {
            terms: (0..=order)
                .map(|_| Equation::Variable(Variable::Integer(0)))
                .collect(),
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

    /// Returns a polynomial with a value of the passed constant
    fn constant(constant: Equation, base: Variable) -> Polynomial {
        Polynomial {
            terms: vec![constant],
            base,
        }
    }

    fn order(&self) -> i64 {
        if self.terms.len() == 0 {
            return 0;
        }
        (self.terms.len() - 1) as i64
    }

    fn single_term_polynomial(term: Equation, exponent: usize, base: Variable) -> Polynomial {
        let mut terms: Vec<Equation> = vec![];
        for _ in 0..exponent {
            terms.push(Equation::Variable(Variable::Integer(0)))
        }
        terms.push(term);
        Polynomial { terms, base }
    }

    /// Takes two polynomails a and b, and computes the division a/b and remainder a%b. Which it
    /// returns ((a%b, b), a//b). This algorithm was (re)invented by Ruben Bartelet.
    fn div_rational(self, other: Polynomial) -> ((Polynomial, Polynomial), Polynomial) {
        let base = self.base.clone();
        if base != other.base {
            panic!("Polynomials must have the same base to divide")
        }
        let mut polya = self.simplify();
        let polyb = other.simplify();

        let mut remainder = Polynomial::zero(base.clone(), polya.order() - polyb.order());
        while polya.order() >= polyb.order() {
            let n = polya.order() as usize;
            let m = polyb.order() as usize;
            let exponent = n - m;

            let first_coefficient =
                Equation::Division(Box::new((polya.terms.remove(n), polyb.terms[m].clone())));
            remainder.terms[exponent] = first_coefficient.clone().simplify();

            // Polya gives wrong stuff often
            let mut new_polya = vec![];
            for i in 0..n - m {
                new_polya.push(polya.terms[i].clone());
            }
            for i in n - m..n {
                new_polya.push(Equation::Addition(vec![
                    polya.terms[i].clone(),
                    Equation::Negative(Box::new(Equation::Multiplication(vec![
                        first_coefficient.clone(),
                        polyb.terms[i + m - n].clone(),
                    ]))),
                ]))
            }
            polya = Polynomial {
                terms: new_polya,
                base: polya.base,
            };

        }
        ((polya, polyb), remainder)
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
                let mut total = Polynomial::zero(base.clone(), 0);
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
            Equation::Division(d) => {
                if d.1.term_is_constant(&base) {
                    return Polynomial::constant(
                        Equation::Division(Box::new((
                            Equation::Variable(Variable::Integer(1)),
                            d.1,
                        ))),
                        base.clone(),
                    ) * Polynomial::from_equation(d.0, base);
                }
                unreachable!()
            }
            Equation::Negative(n) => {
                return Polynomial::constant(
                    Equation::Variable(Variable::Integer(-1)),
                    base.clone(),
                ) * Polynomial::from_equation(*n, base);
            }

            a => unreachable!("{}", a),
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
        Polynomial {
            terms: new_polynomial_terms,
            base: self.base,
        }
    }
}

impl std::ops::Mul for Polynomial {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
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
