use super::{polynomial::Polynomial, steps::StepLogger, Equation, Variable};

impl Equation {
    pub fn taylor_expansion(
        self,
        variable: Variable,
        around: Equation,
        degree: usize,
        step_logger: &mut Option<StepLogger>,
    ) -> Polynomial {
        if let Some(step_logger) = step_logger {
            step_logger.open_step(self.clone(), Some("Calculate taylor series"))
        }
        let mut coefficients: Vec<Equation> = Vec::new();
        let mut current_derivative = self;
        while coefficients.len() <= degree {
            if let Some(step_logger) = step_logger {
                step_logger.open_step(
                    current_derivative.clone(),
                    Some(&format!(
                        "Calculate n={} coefficient of taylor series",
                        coefficients.len()
                    )),
                )
            }

            coefficients.push(Equation::Division(Box::new((
                current_derivative
                    .clone()
                    .evaluate(&variable, &around)
                    .simplify(&mut None),
                Equation::Variable(Variable::Integer(factorial(coefficients.len() as i64))),
            ))));
            current_derivative = current_derivative
                .differentiate(&variable, step_logger)
                .simplify_until_complete(step_logger);
            if let Some(step_logger) = step_logger {
                step_logger.close_step(coefficients[coefficients.len() - 1].clone())
            }
        }
        let result = Polynomial::from_coefficients(coefficients, variable);
        if let Some(step_logger) = step_logger {
            step_logger.close_step(result.clone().into_equation())
        }
        result
    }
}

fn factorial(n: i64) -> i64 {
    if n == 0 {
        return 1;
    }
    n * factorial(n - 1)
}
