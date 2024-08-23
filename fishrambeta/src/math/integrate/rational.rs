use super::{Equation, Variable};
use crate::math::{polynomial::Polynomial, steps::StepLogger};

impl Equation {
    pub(super) fn integrate_rational(self, integrate_to: &Variable, step_logger: &mut Option<StepLogger>) -> Equation {
        if let Equation::Division(d) = self {
            let a = Polynomial::from_equation(d.0, integrate_to.clone());
            let b = Polynomial::from_equation(d.1, integrate_to.clone()).simplify();

            // We must get p/q, where gcd(p,q)=1 and q is monic
            let (quotient, remainder) = a.clone().div(b.clone());

            let polynomial_part = quotient.clone().into_equation().integrate(integrate_to, step_logger);
            let (q, leading_coefficient) = b.into_monic();
            let p = remainder / &leading_coefficient;
            println!("Polynomial part: {}", polynomial_part.simplify(step_logger));
            assert!(
                //We must have a gcd of 1 to progress, I'm pretty sure we already guarantee
                //that by dividing, but I'll leave this in for a while.
                p.clone()
                    .gcd(q.clone())
                    .into_equation()
                    .simplify_until_complete(step_logger)
                    == Equation::Variable(Variable::Integer(1))
            );

            // Now that we have gcd(r,q)=1 and q monic, we can continue by doing doing square-free
            // factorization on q.
            hermite_algorithm(&p, q);
            todo!()
        }
        todo!("Cannot create rational function from arbitrary stuff yet");
    }
}

/// Apply hermite's algorithm to reduce the polynomial integral. Requires q to be monic and
/// square-free.
fn hermite_algorithm(p: &Polynomial, q: Polynomial) {
    let mut factorization = q.square_free_factorization();
    let k = factorization.len();
    let f = factorization.remove(k - 1);
    let g = factorization;
}
