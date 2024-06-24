use super::{Equation, Variable};
use crate::math::polynomial::Polynomial;

impl Equation {
    pub(super) fn integrate_rational(self, integrate_to: &Variable) -> Equation {
        if let Equation::Division(d) = self {
            let a = Polynomial::from_equation(d.0, integrate_to.clone());
            let b = Polynomial::from_equation(d.1, integrate_to.clone()).simplify();

            // We must get p/q, where gcd(p,q)=1 and q is monic
            let (quotient, remainder) = a.clone().div(b.clone());

            let polynomial_part = quotient.clone().to_equation().integrate(integrate_to);
            let (q, leading_coefficient) = b.to_monic();
            let p = remainder / &leading_coefficient;
            println!("Polynomial part: {}", polynomial_part.simplify());
            assert!(
                //We must have a gcd of 1 to progress, I'm pretty sure we already guarantee
                //that by dividing, but I'll leave this in for a while.
                p.clone()
                    .gcd(q.clone())
                    .to_equation()
                    .simplify_until_complete()
                    == Equation::Variable(Variable::Integer(1))
            );

            // Now that we have gcd(r,q)=1 and q monic, we can continue by doing doing square-free
            // factorization on q.
            let factorization = q.square_free_factorization();
            todo!()
        }
        todo!("Cannot create rational function from arbitrary stuff yet");
    }
}
