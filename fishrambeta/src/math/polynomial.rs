use super::{Equation, Variable};
use std::iter;

#[derive(Debug, Clone)]
pub struct Polynomial {
    terms: Vec<Equation>,
    base: Variable,
}

impl Polynomial {
    pub fn to_latex(&self) -> String {
        let base = Equation::Variable(self.base.clone()).to_latex();
        let degree = self.terms.len();
        return self
            .terms
            .clone()
            .into_iter()
            .rev()
            .enumerate()
            .map(|(i, term)| format!("({})\\cdot {}^{}", term, base, degree - 1 - i))
            .intersperse(" + ".to_string())
            .collect();
    }

    pub fn simplify(self) -> Polynomial {
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
    fn zero(base: Variable, degree: i64) -> Polynomial {
        Polynomial {
            terms: (0..=degree)
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

    pub fn degree(&self) -> i64 {
        if self.terms.len() == 0 {
            return 0;
        }
        (self.terms.len() - 1) as i64
    }

    pub fn iszero(&self) -> bool {
        return self
            .terms
            .iter()
            .all(|x| x.clone().simplify() == Equation::Variable(Variable::Integer(0)));
    }

    fn single_term_polynomial(term: Equation, exponent: usize, base: Variable) -> Polynomial {
        let mut terms: Vec<Equation> = vec![];
        for _ in 0..exponent {
            terms.push(Equation::Variable(Variable::Integer(0)))
        }
        terms.push(term);
        Polynomial { terms, base }
    }

    /// Algorithm from wikipedia: Polynomial long division
    pub fn div(self, other: &Polynomial) -> (Polynomial, Polynomial) {
        let base = self.base.clone();
        if base != other.base {
            panic!("Polynomials must have the same base to divide")
        }

        let divisor = other;
        let mut remainder = self.simplify();
        let mut quotient = Polynomial::zero(base.clone(), remainder.degree() - divisor.degree());

        while remainder.degree() >= divisor.degree() && !remainder.iszero() {
            let remainder_degree = remainder.degree();
            let t = Polynomial::single_term_polynomial(
                Equation::Division(Box::new((
                    remainder.terms[remainder.degree() as usize].clone(),
                    divisor.terms[divisor.degree() as usize].clone(),
                ))),
                (remainder.degree() - divisor.degree()) as usize,
                base.clone(),
            );
            quotient = quotient + t.clone();
            remainder = remainder - t * divisor.clone();
            remainder.terms.truncate((remainder_degree) as usize);
        }

        (quotient, remainder)
    }

    pub fn gcd(self, other: Polynomial) -> Polynomial {
        let a = self.simplify();
        let b = other.simplify();

        println!(
            "Gcd step a: {}",
            a
        );
        println!(
            "Gcd step b: {}",
            b
        );
        if b.iszero() {
            println!("Is zero");
            return a;
        }

        let (q, r) = a.div(&b);
        let r = r.simplify().simplify();
        println!(
            "Gcd step r: {}",
            r
        );
        println!(
            "Gcd step q: {}\n",
            q.simplify().simplify()
        );
        return b.gcd(r);
    }

    pub fn from_equation(x: Equation, base: Variable) -> Polynomial {
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

impl std::ops::Sub for Polynomial {
    type Output = Self;

    fn sub(self, other: Polynomial) -> Self::Output {
        if self.base != other.base {
            panic!("Bases must be the same to add polynomials")
        }

        return self + -other;
    }
}

impl std::ops::Neg for Polynomial {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let new_terms: Vec<Equation> = self
            .terms
            .into_iter()
            .map(|x| Equation::Negative(Box::new(x)))
            .collect();
        Polynomial {
            terms: new_terms,
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
