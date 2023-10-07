use crate::parser::IR;

use super::{Equation, Variable};

struct AllPrimitives {
    integrate_to: Variable,
    index: u64,
}

impl Iterator for AllPrimitives {
    type Item = Equation;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        return Some(Equation::Division(Box::new((
            Equation::Sin(Box::new(Equation::Multiplication(vec![
                Equation::Variable(Variable::Letter("x".to_string())),
                Equation::Variable(Variable::Letter("y".to_string())),
            ]))),
            Equation::Variable(Variable::Letter("y".to_string())),
        ))));
    }
}

fn primitives_iter(integrate_to: &Variable) -> AllPrimitives {
    let all_primitives = AllPrimitives {
        integrate_to: integrate_to.clone(),
        index: 0,
    };
    return all_primitives.into_iter();
}

impl Equation {
    pub(super) fn bogointegrate(&self, integrate_to: &Variable) -> Equation {
        for primitive in primitives_iter(integrate_to) {
            if primitive
                .differentiate(integrate_to)
                .simplify_until_complete()
                == *self
            {
                return primitive;
            }
        }
        unreachable!()
    }
}
