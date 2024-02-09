use super::{Equation, Variable};
use num_rational::Rational64;

/// Takes two polynomails a and b, and computes the division a/b and remainder a%b. Which it
/// returns in (a/b, a%b)
pub fn polynomial_division(a: Equation, b: Equation) -> (Equation, Equation) {todo!()}

struct Polynomial {
    terms: Vec<Equation>,
    base: Variable,
}

impl Polynomial {
    fn from_equation(x: Equation) -> Polynomial {todo!()}

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
        return Equation::Addition(total_equation);
    }
}
