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
        self.terms
            .clone()
            .into_iter()
            .rev()
            .enumerate()
            .filter(|(_, x)| *x != Equation::Variable(Variable::Integer(0)))
            .map(|(i, term)| format!("({})\\cdot {}^{}", term, base, degree - 1 - i))
            .collect::<Vec<_>>()
            .join(" + ")
    }

    pub fn simplify(self) -> Polynomial {
        let terms: Vec<Equation> = self
            .terms
            .into_iter()
            .map(|term| term.simplify(&mut None))
            .collect();
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
    fn zero(base: Variable, degree: usize) -> Polynomial {
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

    pub fn degree(&self) -> usize {
        if self.terms.is_empty() {
            return 0;
        }
        self.terms.len() - 1
    }

    pub fn is_zero(&self) -> bool {
        return self
            .terms
            .iter()
            .all(|x| x.clone().simplify(&mut None) == Equation::Variable(Variable::Integer(0)));
    }

    pub fn is_one(&self) -> bool {
        let mut terms = self.terms.clone();
        if terms.is_empty() {
            return false;
        }
        if terms.remove(terms.len() - 1).simplify(&mut None)
            == Equation::Variable(Variable::Integer(1))
        {
            return terms
                .into_iter()
                .all(|x| x.simplify(&mut None) == Equation::Variable(Variable::Integer(0)));
        }
        false
    }

    fn single_term_polynomial(term: Equation, exponent: usize, base: Variable) -> Polynomial {
        let mut terms: Vec<Equation> = vec![];
        for _ in 0..exponent {
            terms.push(Equation::Variable(Variable::Integer(0)));
        }
        terms.push(term);
        Polynomial { terms, base }
    }

    pub fn differentiate(self) -> Polynomial {
        Polynomial {
            terms: self
                .terms
                .into_iter()
                .skip(1)
                .enumerate()
                .map(|(exponent, term)| {
                    Equation::Multiplication(vec![
                        Equation::Variable(Variable::Integer(i64::try_from(exponent).unwrap() + 1)),
                        term,
                    ])
                })
                .collect(),
            base: self.base,
        }
    }

    /// Returns a monic version of the polynomial (one where the first coefficient is one), and the
    /// original first coefficient
    pub fn into_monic(mut self) -> (Polynomial, Equation) {
        let leading_coefficient = self.terms.remove(self.terms.len() - 1);
        (self / &leading_coefficient, leading_coefficient)
    }

    /// Algorithm from wikipedia: Polynomial long division
    pub fn div(self, other: Polynomial) -> (Polynomial, Polynomial) {
        let base = self.base.clone();
        assert!(
            base == other.base,
            "Polynomials must have the same base to divide"
        );

        let mut remainder = self.simplify();
        let divisor = other.simplify();
        let mut quotient = Polynomial::zero(base.clone(), remainder.degree() - divisor.degree());

        while remainder.degree() >= divisor.degree() && !remainder.is_zero() {
            let remainder_degree = remainder.degree();
            let t = Polynomial::single_term_polynomial(
                Equation::Division(Box::new((
                    remainder.terms[remainder.degree()].clone(),
                    divisor.terms[divisor.degree()].clone(),
                ))),
                remainder.degree() - divisor.degree(),
                base.clone(),
            );
            quotient = quotient + t.clone();
            remainder = remainder - t * divisor.clone();
            remainder.terms.truncate(remainder_degree);
        }

        (quotient, remainder)
    }

    /// Compute the polynomial GCD. Returns a monic polynomial in order to make the GCD unique
    pub fn gcd(self, other: Polynomial) -> Polynomial {
        let a = self.simplify();
        let b = other.simplify();

        if b.is_zero() {
            let (gcd, _) = a.into_monic();
            return gcd;
        }

        let (q, r) = a.div(b.clone());
        let r = r.simplify().simplify();
        b.gcd(r)
    }

    /// Compute the square free factorization of a polynomial, algorithm 8.2 from algorithms for
    /// computer algebra
    pub fn square_free_factorization(self) -> Vec<Polynomial> {
        let mut factors: Vec<_> = vec![];
        let a = self.clone().simplify();
        let b = self.differentiate().simplify();
        let mut c = a.clone().gcd(b.clone()).simplify();
        let (mut w, _) = a.div(c.clone());
        w = w.simplify();
        while !c.is_one() {
            let y = c.clone().gcd(w.clone()).simplify();
            let (z, _) = w.clone().div(y.clone());
            factors.push(z.simplify());
            w = y.clone();
            (c, _) = c.div(y);
        }
        factors.push(w);
        factors
    }

    pub fn from_coefficients(coefficients: Vec<Equation>, base: Variable) -> Polynomial {
        Polynomial {
            terms: coefficients,
            base,
        }
    }

    pub fn from_equation(x: Equation, base: Variable) -> Polynomial {
        debug_assert!(x.is_polynomial(&base));
        if x.is_constant(&base) {
            return Polynomial {
                terms: vec![x],
                base,
            };
        }

        match x {
            Equation::Variable(v) => {
                if v == base {
                    Polynomial::single_term_polynomial(
                        Equation::Variable(Variable::Integer(1)),
                        1,
                        base,
                    )
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
                    total = total + polynomial_term;
                }
                total
            }
            Equation::Multiplication(m) => {
                let mut total = Polynomial::one(base.clone());
                for polynomial_term in m
                    .into_iter()
                    .map(|x| Polynomial::from_equation(x, base.clone()))
                {
                    total = total * polynomial_term;
                }
                total
            }
            Equation::Power(p) => {
                if p.0 == Equation::Variable(base.clone()) {
                    if let Some(exponent) = p.1.get_integer_or_none() {
                        Polynomial::single_term_polynomial(
                            Equation::Variable(Variable::Integer(1)),
                            exponent.try_into().unwrap(),
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
                if d.1.is_constant(&base) {
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
                Polynomial::constant(Equation::Variable(Variable::Integer(-1)), base.clone())
                    * Polynomial::from_equation(*n, base)
            }

            a => unreachable!("{}", a),
        }
    }

    pub fn into_equation(self) -> Equation {
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
        if total_equation.is_empty() {
            Equation::Variable(Variable::Integer(0))
        } else {
            Equation::Addition(total_equation)
        }
    }
}

impl std::ops::Add for Polynomial {
    type Output = Self;

    fn add(self, other: Polynomial) -> Self::Output {
        assert!(
            self.base == other.base,
            "Bases must be the same to add polynomials"
        );

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
        assert!(
            self.base == other.base,
            "Bases must be the same to add polynomials"
        );

        self + -other
    }
}

impl std::ops::Div<&Equation> for Polynomial {
    type Output = Self;

    fn div(self, other: &Equation) -> Self::Output {
        let mut new_terms: Vec<_> = self
            .terms
            .into_iter()
            .map(|x| Equation::Division(Box::new((x, other.clone()))))
            .collect();
        new_terms.push(Equation::Variable(Variable::Integer(1)));
        Polynomial {
            terms: new_terms,
            base: self.base,
        }
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
        assert!(
            self.base == other.base,
            "Bases must be the same to multiply polynomials"
        );

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
                if x.is_empty() {
                    Equation::Variable(Variable::Integer(0))
                } else {
                    Equation::Addition(x)
                }
            })
            .collect();
        Polynomial {
            terms: new_terms_as_vec_of_equations,
            base: self.base,
        }
    }
}
