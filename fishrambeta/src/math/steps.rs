use crate::math::Equation;

#[derive(Debug)]
pub struct StepLogger {
    steps: Vec<Step>,
}

#[derive(Debug)]
pub struct Step {
    equation_before: Equation,
    equation_after: Equation,
    message: Option<String>,
}

impl StepLogger {
    pub fn new() -> Self {
        StepLogger { steps: Vec::new() }
    }
    pub fn add_step(&mut self, step: Step) {
        if step.equation_before != step.equation_after {
            self.steps.push(step);
        }
    }
}

impl Step {
    pub fn new(equation_before: Equation, equation_after: Equation, message: Option<&str>) -> Self {
        Step {
            equation_before,
            equation_after,
            message: match message {
                Some(message) => Some(message.to_string()),
                None => None,
            },
        }
    }
}
