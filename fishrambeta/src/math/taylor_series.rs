use super::{
    polynomial::Polynomial,
    steps::{
        helpers::{close_step, open_step},
        StepLogger,
    },
    Equation, Variable,
};

impl Equation {
    pub fn taylor_expansion(
        self,
        variable: Variable,
        around: &Equation,
        degree: usize,
        step_logger: &mut Option<StepLogger>,
    ) -> Polynomial {
        open_step(step_logger, &self, Some("Calculate taylor series"));
        let mut coefficients: Vec<Equation> = Vec::new();
        let mut current_derivative = self;
        while coefficients.len() <= degree {
            open_step(
                step_logger,
                &current_derivative,
                Some(&format!(
                    "Calculate n={} coefficient of taylor series",
                    coefficients.len()
                )),
            );

            coefficients.push(Equation::Division(Box::new((
                current_derivative
                    .clone()
                    .evaluate(&variable, around)
                    .simplify(&mut None),
                Equation::Variable(Variable::Integer(factorial(coefficients.len().try_into().expect("Taylor series cannot be calculated due to the 1/n! term being larger than i64")))),
            ))));
            current_derivative = current_derivative
                .differentiate(&variable, step_logger)
                .simplify_until_complete(step_logger);
            close_step(step_logger, &coefficients[coefficients.len() - 1]);
        }
        let result = Polynomial::from_coefficients(
            coefficients,
            Equation::Addition(vec![
                Equation::Variable(variable),
                Equation::Negative(Box::new(around.clone())),
            ])
            .simplify(&mut None),
        );

        close_step(step_logger, &result.clone().into_equation());
        result
    }
}

fn factorial(n: i64) -> i64 {
    if n == 0 {
        return 1;
    }
    n * factorial(n - 1)
}
