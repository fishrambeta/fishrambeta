use super::{Equation, Variable};

/// Takes two polynomails a and b, and computes the division a/b and remainder a%b. Which it
/// returns in (a/b, a%b)
pub fn polynomial_division(a: Equation, b: Equation) -> (Equation, Equation) {
    todo!()
}

struct Polynomial {
    terms: Vec<Equation>,
    base: Variable,
}

impl Polynomial {
    fn from_equation(x: Equation) -> Polynomial {
        todo!()
    }

    fn to_equation(mut self) -> Equation {
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
        Equation::Addition(total_equation)
    }
}

impl std::ops::Add for Polynomial {
    type Output = Self;

    fn add(self, other: Polynomial) -> Self::Output {
        if self.base != other.base {
            panic!("Bases must be the same to add polynomials")
        }

        let mut new_polynomial_terms: Vec<Equation> = Vec::new();
        for (a, b) in self.terms.into_iter().zip(other.terms) {
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
            (0..max_exponent).map(|_| Vec::new()).collect();

        for (i, a) in self.terms.iter().enumerate() {
            for (j, b) in other.terms.iter().enumerate() {
                let coefficient = Equation::Multiplication(vec![a.clone(), b.clone()]);
                let exponent = i + j;
                new_terms_as_vec_of_vecs[exponent].push(coefficient);
            }
        }

        let new_terms_as_vec_of_equations: Vec<Equation> = new_terms_as_vec_of_vecs
            .into_iter()
            .map(|x| Equation::Addition(x))
            .collect();
        Polynomial {
            terms: new_terms_as_vec_of_equations,
            base: self.base,
        }
    }
}
