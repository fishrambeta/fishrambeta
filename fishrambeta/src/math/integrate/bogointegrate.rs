use crate::math::Constant;

use super::{Equation, Variable};
use rayon::prelude::*;

fn all_equations(integrate_to: &Variable, depth: u32) -> Vec<Equation> {
    let mut equations: Vec<_> = all_variable_equations(integrate_to, depth);

    if depth == 0 {
        return equations;
    }

    for x in all_equations(integrate_to, depth - 1) {
        for y in all_equations(integrate_to, depth - 1) {
            equations.push(Equation::Addition(vec![x.clone(), y.clone()]));
            equations.push(Equation::Multiplication(vec![x.clone(), y.clone()]));
            equations.push(Equation::Division(Box::new((x.clone(), y.clone()))));
            equations.push(Equation::Power(Box::new((x.clone(), y.clone()))));
        }
        equations.push(Equation::Ln(Box::new(x.clone())));
        equations.push(Equation::Sin(Box::new(x.clone())));
        equations.push(Equation::Cos(Box::new(x.clone())));
    }

    equations
}

fn all_variable_equations(integrate_to: &Variable, depth: u32) -> Vec<Equation> {
    let mut variables = vec![
        Equation::Variable(integrate_to.clone()),
        Equation::Variable(Variable::Constant(Constant::E)),
        Equation::Variable(Variable::Constant(Constant::PI)),
    ];
    for i in 0..((2 as i64).pow(depth + 1)) {
        variables.push(Equation::Variable(Variable::Integer(i)));
    }
    variables
}

impl Equation {
    pub(super) fn bogointegrate(&self, integrate_to: &Variable) -> Equation {
        let simplified = self.clone().simplify_until_complete();
        let mut depth = 0;
        loop {
            let equations_to_try = all_equations(integrate_to, depth);
            let len = equations_to_try.len();
            println!(
                "Searching for primitive at depth {}, number of equations: {}",
                depth, len
            );
            if let Some(primitive) = equations_to_try.into_par_iter().find_any(|x| {
                println!(
                    "Checking {}, depth: {}. Number of equations to try: {}",
                    x, depth, len
                );
                x.clone().is_primitive(&simplified, integrate_to)
            }) {
                return primitive;
            }
            depth += 1;
        }
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
