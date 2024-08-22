use crate::math::Equation;
use core::fmt;

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

    pub fn get_steps_as_strings(&self) -> Vec<String> {
        self.steps.iter().map(|step| step.to_string()).collect()
    }
}
impl fmt::Display for StepLogger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let stringified = self
            .steps
            .iter()
            .map(|step| format!("{}", step))
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{}", stringified)
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

impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.message {
            Some(message) => write!(
                f,
                "\\textbf{{{}: }}{} \\textbf{{=>}} {}",
                message, self.equation_before, self.equation_after
            ),
            None => write!(
                f,
                "{} \\textbf{{=>}} {}",
                self.equation_before, self.equation_after
            ),
        }
    }
}
