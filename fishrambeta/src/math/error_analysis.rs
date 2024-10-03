use crate::math::{Equation, Variable};
use num::Rational64;

use super::steps::{helpers::{close_step, open_step}, StepLogger};

impl Equation {
    pub fn error_analysis(
        self,
        error_variables: Vec<Variable>,
        step_logger: &mut Option<StepLogger>,
    ) -> Equation {
        open_step(step_logger, &self, Some("Do error analysis"));
        let mut terms: Vec<Equation> = Vec::new();
        for variable in error_variables {
            open_step(step_logger, &self, Some("Apply error propagation formula"));
            let mut derivative = self.differentiate(&variable, step_logger);
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
            close_step(step_logger, &term);
            terms.push(term);
        }
        let result = Equation::Power(Box::new((
            Equation::Addition(terms),
            Equation::Variable(Variable::Rational(Rational64::new(1, 2))),
        )));
        close_step(step_logger, &result);
        result
    }
}
