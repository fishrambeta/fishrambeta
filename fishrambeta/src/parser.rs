use crate::math::{Constant, Equation, Variable};
use slog::{crit, debug, info, Logger};

#[derive(Debug, Clone)]
struct LatexEqnIR {
    name: String,
    parameters: Vec<LatexEqnIR>,
    subscript: Option<Box<LatexEqnIR>>,
    superscript: Option<Box<LatexEqnIR>>,
    depth: u32,
}
impl LatexEqnIR {
    pub fn latex_to_ir(latex: Vec<char>, logger: &Logger, depth: u32) -> Self {
        let mut latex = latex;
        let operator_index = Self::contains_operators_in_top_level(&latex);
        return if let Some(operator_index) = operator_index {
            let (left, right) = latex.split_at(operator_index);
            let (left, mut right) = (left.to_vec(), right.to_vec());
            let operator = right.remove(0);
            print_data(logger, operator.to_string(), depth);
            let (left_ir, right_ir) = (
                Self::latex_to_ir(left, logger, depth + 1),
                Self::latex_to_ir(right, logger, depth + 1),
            );
            LatexEqnIR {
                name: String::from(operator),
                subscript: None,
                superscript: None,
                depth,
                parameters: vec!(left_ir, right_ir)
            }
        } else if (
            (latex.contains(&'{') && latex.contains(&'}'))
                || (latex.contains(&'(') && latex.contains(&')')))
            || (latex.contains(&'[') && latex.contains(&']')) {
            let end_name = unsafe { latex.iter().position(|char| char == &'{' || char == &'(').unwrap_unchecked() };
            let (name, parameters) = latex.split_at(end_name);
            let (mut name, mut parameter_chars) = (name.to_vec(), parameters.to_vec());
            print_data(logger, name.iter().collect(), depth);
            let mut parameters = vec!();
            loop {
                if parameter_chars.len() == 0 { break }
                let mut depth = 1;
                parameter_chars.remove(0);
                let mut inner_data = vec!();
                loop {
                    if depth == 0 { break }
                    let char = parameter_chars.remove(0);
                    if char == '{' || char == '(' || char == '[' { depth += 1 }
                    if char == '}' || char == ')' || char == ']' { depth -= 1 }
                    inner_data.push(char)
                }
                inner_data.remove(inner_data.len() - 1);
                parameters.push(inner_data);
            }
            let parameters = parameters.into_iter().map(|param| Self::latex_to_ir(param, logger, depth + 1)).collect();
            let mut superscript = None;
            let mut subscript = None;
            if name.contains(&'^'){
                let start = unsafe{name.iter().position(|char| char == &'^').unwrap_unchecked()};
                let end = match name.contains(&'_'){
                    true => {
                        unsafe{name.iter().position(|char| char == &'_').unwrap_unchecked()}
                    }
                    false => {name.len()}
                };
                let mut superscript_chars = name.drain(start .. end).collect::<Vec<_>>();
                superscript_chars.remove(0);
                superscript = Some(Box::new(Self::latex_to_ir(superscript_chars, logger, depth+1)));
            }
            if name.contains(&'_'){
                let start = unsafe{name.iter().position(|char| char == &'_').unwrap_unchecked()};
                let mut subscript_chars = name.drain(start..name.len()).collect::<Vec<_>>();
                subscript_chars.remove(0);
                subscript = Some(Box::new(Self::latex_to_ir(subscript_chars, logger, depth+1)))
            }
            Self {
                parameters,
                name: name.into_iter().collect(),
                depth,
                superscript,
                subscript,
            }
        } else if latex.contains(&'^') {
            let start = unsafe { latex.iter().position(|char| char == &'^').unwrap_unchecked() };
            let end = match latex.iter().position(|char| char == &'_') {
                Some(end) => {
                    if end > start {
                        end
                    } else {
                        latex.len() - 1
                    }
                }
                None => {
                    latex.len() - 1
                }
            };
            let mut superscript = latex.drain(start..=end).collect::<Vec<char>>();
            superscript.remove(0);
            let superscript_ir = Self::latex_to_ir(superscript, logger, depth + 1);
            let mut equation = Self::latex_to_ir(latex, logger, depth + 1);
            equation.superscript = Some(Box::new(superscript_ir));
            equation
        } else if latex.contains(&'_') {
            let start = unsafe { latex.iter().position(|char| char == &'_').unwrap_unchecked() };
            let end = match latex.iter().position(|char| char == &'^') {
                Some(end) => {
                    if end > start {
                        end
                    } else {
                        latex.len() - 1
                    }
                }
                None => {
                    latex.len() - 1
                }
            };
            let mut subscript = latex.drain(start..=end).collect::<Vec<char>>();
            subscript.remove(0);
            let mut equation = Self::latex_to_ir(latex, logger, depth + 1);
            equation.subscript = Some(Box::new(Self::latex_to_ir(subscript.into_iter().collect::<Vec<_>>(), logger, depth + 1)));
            equation
        } else {
            print_data(logger, latex.iter().collect(), depth);
            Self {
                name: latex.into_iter().collect(),
                depth,
                parameters: vec!(),
                superscript: None,
                subscript: None,
            }
        }
    }
    fn contains_operators_in_top_level(latex: &Vec<char>) -> Option<usize> {
        let mut depth = 1;
        for (i, char) in latex.iter().enumerate() {
            if depth == 1 && (char == &'+' || char == &'-' || char == &'*' || char == &'/' || char == &'=') {
                return Some(i);
            }
            if char == &'{' || char == &'(' || char == &'[' {
                depth += 1
            } else if char == &'}' || char == &')' || char == &']'{
                depth -= 1
            }
        }
        return None;
    }
    pub fn ir_to_eqn(mut self, logger: &Logger) -> Equation {
        return match &*self.name{
             "=" => {
                Equation::Equals(Box::new((self.parameters.remove(0).ir_to_eqn(logger), self.parameters.remove(0).ir_to_eqn(logger))))
            }
            "\\vec" => {
                Equation::Variable(Variable::Vector(self.parameters.remove(0).name))
            }
            other => {
                info!(logger, "{}, params: {}" ,other, self.parameters.len());

                return Equation::Variable(Variable::Letter(String::from(other)))
            }
        };
    }
}

pub fn to_equation(latex: String, logger: &Logger) -> Equation {
    let ir = latex_to_ir(latex, logger);
    return ir_to_eqn(ir, logger);
}
fn latex_to_ir(latex: String, logger: &Logger) -> LatexEqnIR {
    return LatexEqnIR::latex_to_ir(
        preprocess(latex)
            .chars()
            .filter(|char| char != &' ')
            .collect(),
        logger,
        1,
    );
}
fn ir_to_eqn(ir: LatexEqnIR, logger: &Logger) -> Equation {
    return ir.ir_to_eqn(logger);
}
fn preprocess(latex: String) -> String {
    //return latex;
    return latex
        .replace("\\biggl\\", "")
        .replace("\\bigg\\", "")
        .replace("\\biggl", "")
        .replace("\\bigg", "")
        .replace("\\cdot", "*");
}
fn print_data(logger : &Logger, data: String, depth: u32){
    let mut formatted = String::new();
    for _ in 0..=depth{formatted.push_str(" | ")}
    formatted.push_str(&*data);
    debug!(logger, "{}" , formatted);
}