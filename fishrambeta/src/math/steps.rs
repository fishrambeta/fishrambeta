use crate::math::Equation;
use core::fmt;

#[derive(Debug)]
pub struct StepLogger {
    current_step_stack: Vec<Step>,
    steps: Vec<Step>,
}

#[derive(Debug)]
pub struct Step {
    equation_before: Equation,
    equation_after: Option<Equation>,
    sub_steps: Vec<Step>,
    message: Option<String>,
}

impl StepLogger {
    pub fn new() -> Self {
        StepLogger {
            current_step_stack: Vec::new(),
            steps: Vec::new(),
        }
    }

    pub fn open_step(&mut self, equation_before: Equation, message: Option<&str>) {
        let new_step = Step {
            equation_before,
            equation_after: None,
            sub_steps: Vec::new(),
            message: match message {
                Some(message) => Some(message.to_string()),
                None => None,
            },
        };
        self.current_step_stack.push(new_step);
    }

    pub fn close_step(&mut self, equation_after: Equation) {
        let mut closing_step = self
            .current_step_stack
            .pop()
            .expect("Cannot close step when no step is being written");
        if closing_step.equation_before == equation_after {
            return;
        }
        closing_step.equation_after = Some(equation_after);
        if self.current_step_stack.len() == 0 {
            self.steps.push(closing_step);
        } else {
            self.current_step_stack
                .last_mut()
                .unwrap()
                .sub_steps
                .push(closing_step);
        }
    }

    pub fn cancel_step(&mut self) {
        self.current_step_stack.pop();
    }

    pub fn get_steps_as_strings(&self) -> Vec<String> {
        self.to_string()
            .lines()
            .map(|line| line.to_string())
            .collect()
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

impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self._to_string(0))
    }
}

impl Step {
    fn _to_string(&self, depth: usize) -> String {
        let mut stringified = "\\textbf{ ".to_string();
        for _ in 0..depth {
            stringified.push_str("--");
        }
        stringified += "}";
        stringified += &match &self.message {
            Some(message) => format!(
                "\\textbf{{{}: }}{} \\textbf{{ => }} {}",
                message,
                self.equation_before,
                self.equation_after.as_ref().unwrap()
            ),
            None => format!(
                "{} \\textbf{{ => }} {}",
                self.equation_before,
                self.equation_after.as_ref().unwrap()
            ),
        };
        self.sub_steps
            .iter()
            .for_each(|step| stringified.push_str(&format!("\n{}", step._to_string(depth + 1))));
        stringified
    }
}
