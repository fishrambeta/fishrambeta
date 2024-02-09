use super::{Equation, Variable};
use num_rational::Rational64;
use rand::rngs::ThreadRng;
use rand::{seq::SliceRandom, Rng};
use rayon::prelude::*;

struct AllPrimitives {
    integrate_to: Variable,
    index: u64,
    rng: ThreadRng,
}

unsafe impl Send for AllPrimitives {}

impl Iterator for AllPrimitives {
    type Item = Equation;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        let equation = random_equation(&vec!["x".to_string()], &mut self.rng, 0);
        if self.index % 10000 != 0 {
            println!("{}: Guessing equation: {}", self.index, equation);
        }
        return Some(equation);
    }
}

fn primitives_iter(integrate_to: &Variable) -> AllPrimitives {
    let all_primitives = AllPrimitives {
        integrate_to: integrate_to.clone(),
        index: 0,
        rng: rand::thread_rng(),
    };
    return all_primitives.into_iter();
}

fn random_equation(
    relevant_variables: &Vec<String>,
    rng: &mut ThreadRng,
    complexity: u32,
) -> Equation {
    match rng.gen_range(1..200 + 2_i64.pow(complexity)) {
        0..=10 => {
            return Equation::Addition(vec![
                random_equation(relevant_variables, rng, complexity + 2),
                random_equation(relevant_variables, rng, complexity + 2),
            ])
        }
        11..=20 => {
            return Equation::Multiplication(vec![
                random_equation(relevant_variables, rng, complexity + 2),
                random_equation(relevant_variables, rng, complexity + 2),
            ])
        }
        21..=30 => {
            return Equation::Division(Box::new((
                random_equation(relevant_variables, rng, complexity + 2),
                random_equation(relevant_variables, rng, complexity + 2),
            )))
        }
        31..=40 => {
            return Equation::Sin(Box::new(random_equation(
                relevant_variables,
                rng,
                complexity + 2,
            )))
        }
        41..=50 => {
            return Equation::Cos(Box::new(random_equation(
                relevant_variables,
                rng,
                complexity + 1,
            )))
        }
        51..=60 => {
            return Equation::Ln(Box::new(random_equation(
                relevant_variables,
                rng,
                complexity + 1,
            )))
        }
        61..=70 => {
            return Equation::Negative(Box::new(random_equation(
                relevant_variables,
                rng,
                complexity + 1,
            )))
        }
        71..=80 => {
            return Equation::Power(Box::new((
                random_equation(relevant_variables, rng, complexity + 2),
                random_equation(relevant_variables, rng, complexity + 2),
            )))
        }
        _ => match rng.gen_range(1..3) {
            1 => {
                return Equation::Variable(Variable::Letter(
                    relevant_variables.choose(rng).unwrap().clone(),
                ))
            }
            2 => return Equation::Variable(Variable::Integer(rng.gen_range(1..10))),
            3 => {
                return Equation::Variable(Variable::Rational(Rational64::from((
                    rng.gen_range(-10..10),
                    rng.gen_range(1..10),
                ))))
            }
            _ => unreachable!(),
        },
    }
}

impl Equation {
    pub(super) fn bogointegrate(&self, integrate_to: &Variable) -> Equation {
        let simplified = self.clone().simplify_until_complete();
        return primitives_iter(integrate_to)
            .par_bridge()
            .find_any(|x: &Equation| x.clone().is_primitive(&simplified, integrate_to))
            .unwrap();
    }

    fn is_primitive(self, simplified: &Equation, integrate_to: &Variable) -> bool {
        let is_primitive =
            self.differentiate(integrate_to).simplify_until_complete() == *simplified;
        if is_primitive {
            println!("{} is a primitive of {}", self, simplified);
        }
        return is_primitive;
    }
}
