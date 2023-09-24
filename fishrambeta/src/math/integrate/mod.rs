use super::{Equation, Variable};

impl Equation {
    pub fn integrate(&self, integrate_to: &Variable) -> Equation {
        match self {
            Equation::Addition(addition) => {
                return Equation::Addition(addition.iter().map(|x| x.integrate(integrate_to)).collect());
            }
            Equation::Variable(Variable::Integer(i)) => return Equation::Multiplication(vec![Equation::Variable(Variable::Integer(*i)), Equation::Variable(integrate_to.clone())]),
            Equation::Variable(Variable::Rational(r)) => return Equation::Multiplication(vec![Equation::Variable(Variable::Rational(*r)), Equation::Variable(integrate_to.clone())]),
            _ => todo!()
        }
    }
}
