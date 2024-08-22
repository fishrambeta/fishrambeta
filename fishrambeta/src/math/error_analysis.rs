use crate::math::{Equation, Variable};
use num::Rational64;

use super::steps::StepLogger;

impl Equation {
    pub fn error_analysis(self, error_variables: Vec<Variable>, step_logger: &mut Option<StepLogger>) -> Equation {
        let mut terms: Vec<Equation> = Vec::new();
        for variable in error_variables {
            let mut derivative = self.differentiate(&variable);
            derivative = derivative.simplify_until_complete(step_logger);
            let term = Equation::Power(Box::new((
                Equation::Multiplication(vec![
                    derivative,
                    Equation::Variable(Variable::Letter(format!(
                        "s_{}",
                        Equation::Variable(variable)
                    ))),
                ]),
                Equation::Variable(Variable::Integer(2)),
            )));
            terms.push(term);
        }
        let result = Equation::Power(Box::new((
            Equation::Addition(terms),
            Equation::Variable(Variable::Rational(Rational64::new(1, 2))),
        )));
        return result;
    }
}
