use std::env::var;
use crate::math::{Constant, Equation, Variable};
use slog::{crit, debug, info, Logger};
use std::ops::Add;

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
                parameters: vec![left_ir, right_ir],
            }
        } else if ((latex.contains(&'{') && latex.contains(&'}'))
            || (latex.contains(&'(') && latex.contains(&')')))
            || (latex.contains(&'[') && latex.contains(&']'))
        {
            let end_name = unsafe {
                latex
                    .iter()
                    .position(|char| char == &'{' || char == &'(')
                    .unwrap_unchecked()
            };
            let (name, parameters) = latex.split_at(end_name);
            let (mut name, mut parameter_chars) = (name.to_vec(), parameters.to_vec());
            print_data(logger, name.iter().collect(), depth);
            let mut parameters = vec![];
            loop {
                if parameter_chars.len() == 0 {
                    break;
                }
                let mut depth = 1;
                parameter_chars.remove(0);
                let mut inner_data = vec![];
                loop {
                    if depth == 0 {
                        break;
                    }
                    let char = parameter_chars.remove(0);
                    if char == '{' || char == '(' || char == '[' {
                        depth += 1
                    }
                    if char == '}' || char == ')' || char == ']' {
                        depth -= 1
                    }
                    inner_data.push(char)
                }
                inner_data.remove(inner_data.len() - 1);
                parameters.push(inner_data);
            }
            let parameters = parameters
                .into_iter()
                .map(|param| Self::latex_to_ir(param, logger, depth + 1))
                .collect();
            let mut superscript = None;
            let mut subscript = None;
            if name.contains(&'^') {
                let start = unsafe { name.iter().position(|char| char == &'^').unwrap_unchecked() };
                let end = match name.contains(&'_') {
                    true => unsafe { name.iter().position(|char| char == &'_').unwrap_unchecked() },
                    false => name.len(),
                };
                let mut superscript_chars = name.drain(start..end).collect::<Vec<_>>();
                superscript_chars.remove(0);
                superscript = Some(Box::new(Self::latex_to_ir(
                    superscript_chars,
                    logger,
                    depth + 1,
                )));
            }
            if name.contains(&'_') {
                let start = unsafe { name.iter().position(|char| char == &'_').unwrap_unchecked() };
                let mut subscript_chars = name.drain(start..name.len()).collect::<Vec<_>>();
                subscript_chars.remove(0);
                subscript = Some(Box::new(Self::latex_to_ir(
                    subscript_chars,
                    logger,
                    depth + 1,
                )))
            }
            Self {
                parameters,
                name: name.into_iter().collect(),
                depth,
                superscript,
                subscript,
            }
        } else if latex.contains(&'^') {
            let start = unsafe {
                latex
                    .iter()
                    .position(|char| char == &'^')
                    .unwrap_unchecked()
            };
            let end = match latex.iter().position(|char| char == &'_') {
                Some(end) => {
                    if end > start {
                        end
                    } else {
                        latex.len() - 1
                    }
                }
                None => latex.len() - 1,
            };
            let mut superscript = latex.drain(start..=end).collect::<Vec<char>>();
            superscript.remove(0);
            let superscript_ir = Self::latex_to_ir(superscript, logger, depth + 1);
            let mut equation = Self::latex_to_ir(latex, logger, depth + 1);
            equation.superscript = Some(Box::new(superscript_ir));
            equation
        } else if latex.contains(&'_') {
            let start = unsafe {
                latex
                    .iter()
                    .position(|char| char == &'_')
                    .unwrap_unchecked()
            };
            let end = match latex.iter().position(|char| char == &'^') {
                Some(end) => {
                    if end > start {
                        end
                    } else {
                        latex.len() - 1
                    }
                }
                None => latex.len() - 1,
            };
            let mut subscript = latex.drain(start..=end).collect::<Vec<char>>();
            subscript.remove(0);
            let mut equation = Self::latex_to_ir(latex, logger, depth + 1);
            equation.subscript = Some(Box::new(Self::latex_to_ir(
                subscript.into_iter().collect::<Vec<_>>(),
                logger,
                depth + 1,
            )));
            equation
        } else {
            print_data(logger, latex.iter().collect(), depth);
            Self {
                name: latex.into_iter().collect(),
                depth,
                parameters: vec![],
                superscript: None,
                subscript: None,
            }
        };
    }
    fn contains_operators_in_top_level(latex: &Vec<char>) -> Option<usize> {
        let mut depth = 1;
        for (i, char) in latex.iter().enumerate() {
            if depth == 1
                && (char == &'+' || char == &'-' || char == &'*' || char == &'/' || char == &'=')
            {
                return Some(i);
            }
            if char == &'{' || char == &'(' || char == &'[' {
                depth += 1
            } else if char == &'}' || char == &')' || char == &']' {
                depth -= 1
            }
        }
        return None;
    }
    pub fn ir_to_eqn(mut self, logger: &Logger) -> Equation {
        return match &*self.name {
            "=" => Equation::Equals(Box::new((
                self.parameters.remove(0).ir_to_eqn(logger),
                self.parameters.remove(0).ir_to_eqn(logger),
            ))),
            "\\vec" | "\\hat" => {
                if self.superscript.is_none() {
                    Equation::Variable(Variable::Vector(
                        self.parameters.remove(0).name_with_subscript(),
                    ))
                } else {
                    let power = unsafe { self.superscript.unwrap_unchecked() };
                    self.superscript = None;
                    Equation::Power(Box::new((self.ir_to_eqn(logger), power.ir_to_eqn(logger))))
                }
            }
            "\\frac" => {
                if self.parameters.len() < 2 {
                    crit!(logger, "Fraction supplied with less than two arguments");
                    panic!();
                } else {
                    let first = self.parameters.remove(0);
                    let second = self.parameters.remove(0);
                    if self.parameters.len() > 0 {
                        let mut parameters_unparsed = vec![];
                        std::mem::swap(&mut self.parameters, &mut parameters_unparsed);
                        let mut parameters = vec![];
                        for param in parameters_unparsed {
                            parameters.push(param.ir_to_eqn(logger));
                        }
                        parameters.push(Equation::Division(Box::new((
                            first.ir_to_eqn(logger),
                            second.ir_to_eqn(logger),
                        ))));
                        return Equation::Multiplication(parameters);
                    } else {
                        return Equation::Division(Box::new((
                            first.ir_to_eqn(logger),
                            second.ir_to_eqn(logger),
                        )));
                    }
                }
            }
            "*" => {
                return Equation::Multiplication(
                    self.parameters
                        .into_iter()
                        .map(|x| x.ir_to_eqn(logger))
                        .collect::<Vec<_>>(),
                )
            }
            "+" => {
                if self.parameters.len() != 2 {
                    crit!(logger, "Invalid addition, not two paramters");
                    panic!();
                } else {
                    return Equation::Addition(vec![
                        self.parameters.remove(0).ir_to_eqn(logger),
                        self.parameters.remove(0).ir_to_eqn(logger),
                    ]);
                }
            }
            "-" => {
                if self.parameters.len() != 2 {
                    crit!(logger, "Invalid addition, not two paramters");
                    panic!();
                } else {
                    return Equation::Subtraction(vec![
                        self.parameters.remove(0).ir_to_eqn(logger),
                        self.parameters.remove(0).ir_to_eqn(logger),
                    ]);
                }
            }
            other => {
                info!(logger, "{}, params: {}", other, self.parameters.len());
                debug!(logger, "{:?}", self);
                if self.parameters.len() > 0 {
                    let isinvalid = other == "";
                    let mut parameters_unparsed = vec![];
                    std::mem::swap(&mut self.parameters, &mut parameters_unparsed);
                    let mut parameters = vec![];
                    for param in parameters_unparsed {
                        parameters.push(param.ir_to_eqn(logger));
                    }
                    let function = if self.superscript.is_none() {
                        Equation::Variable(Variable::Letter(self.name_with_subscript()))
                    } else if !other.contains(&['1','2','3','4','5','6','7','8','9','0']){
                        let power = unsafe { self.superscript.unwrap_unchecked() };
                        self.superscript = None;
                        Equation::Power(Box::new((self.ir_to_eqn(logger), power.ir_to_eqn(logger))))
                    } else{
                        todo!("IsNumber");
                    };
                    if !isinvalid{
                        parameters.push(function);
                    }
                    return Equation::Multiplication(parameters);
                }
                if other.contains(&['1','2','3','4','5','6','7','8','9','0']){
                    let mut individual_variables = vec!();
                    let mut last_combined = vec!();
                    for char in other.chars(){
                        if Self::is_number(&char){
                            if last_combined.len() == 0 || Self::is_number(&last_combined[last_combined.len()-1]){
                                last_combined.push(char);
                            }
                            else{
                                individual_variables.push(last_combined.iter().collect::<String>());
                                last_combined = vec!(char)
                            }
                        }
                        else{
                            if last_combined.len() == 0 || !Self::is_number(&last_combined[last_combined.len()-1]){
                                last_combined.push(char);
                            }
                            else{
                                individual_variables.push(last_combined.iter().collect::<String>());
                                last_combined=vec!(char)
                            }
                        }
                    }
                    if last_combined.len() != 0{
                        individual_variables.push(last_combined.iter().collect::<String>())
                    }
                    if individual_variables.len() == 1{
                        return if !individual_variables[0].contains('.') {
                            let integer = match individual_variables[0].parse::<i32>() {
                                Ok(int) => { int }
                                Err(error) => {
                                    crit!(logger, "Failed to parse integer, {}", error);
                                    panic!();
                                }
                            };
                            Equation::Variable(Variable::Integer(integer))
                        } else {
                            Self::parse_float(individual_variables[0].clone(), logger)
                        }
                    }
                    let mut parsed_eqs = vec!();
                    for variable in individual_variables{
                        //Suboptimal check, may be improved later
                        if Self::is_number(&variable.chars().collect::<Vec<_>>()[0]){
                            let number = if variable.contains('.'){ Self::parse_float(variable, logger)} else {match variable.parse::<i32>() {
                                Ok(int) => { Equation::Variable(Variable::Integer(int)) }
                                Err(error) => {
                                    crit!(logger, "Failed to parse integer, {}", error);
                                    panic!();
                                }
                            }};
                            parsed_eqs.push(number);
                        }else{
                            parsed_eqs.push(Equation::Variable(Variable::Letter(variable)))
                        };
                    }
                    return Equation::Multiplication(parsed_eqs);
                }
                return Equation::Variable(Variable::Letter(String::from(
                    self.name_with_subscript(),
                )));
            }
        };
    }
    pub fn name_with_subscript(self) -> String {
        let mut name = self.name;
        if self.subscript.is_some() {
            name.push('_');
            name.push_str(unsafe { &self.subscript.unwrap_unchecked().name });
        }
        return name;
    }
    fn is_number(char : &char) -> bool {
        return ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '.'].contains(char);
    }
    pub fn eqn_to_ir(eqn: Equation, logger: &Logger, depth: u32) -> LatexEqnIR {
        return match eqn {
            Equation::Variable(var) => {
                match var {
                    Variable::Constant(constant) => {
                        match constant {
                            Constant::E => LatexEqnIR { name: String::from("e"), depth, parameters: vec!(), superscript: None, subscript: None },
                            Constant::PI => LatexEqnIR { name: String::from("\\pi"), depth, parameters: vec!(), superscript: None, subscript: None },
                        }
                    }
                    Variable::Letter(name) => {
                        return LatexEqnIR { name, depth, parameters: vec!(), superscript: None, subscript: None }
                    }
                    Variable::Vector(name) => {
                        return LatexEqnIR { name: String::from("\\vec"), parameters: vec!(LatexEqnIR { name, depth: depth + 1, subscript: None, superscript: None, parameters: vec!() }), depth, superscript: None, subscript: None }
                    }
                    Variable::Integer(int) => {
                        return LatexEqnIR { name: int.to_string(), parameters: vec!(), depth, superscript: None, subscript: None }
                    }
                    Variable::Rational((p, q)) => {
                        return LatexEqnIR {
                            name: String::from("\\frac"),
                            parameters: vec!(
                                LatexEqnIR { name: p.to_string(), depth: depth + 1, subscript: None, superscript: None, parameters: vec!() },
                                LatexEqnIR { name: q.to_string(), depth: depth + 1, subscript: None, superscript: None, parameters: vec!() },
                            ),
                            depth,
                            superscript: None,
                            subscript: None
                        }
                    }
                }
            }
            Equation::Addition(additions) => {
                let mut parameters = vec!();
                for parameter in additions {
                    parameters.push(Self::eqn_to_ir(parameter, logger, depth + 1))
                }
                LatexEqnIR {
                    name: String::from("+"),
                    depth,
                    superscript: None,
                    subscript: None,
                    parameters,
                }
            }
            Equation::Subtraction(subtractions) => {
                let mut parameters = vec!();
                for parameter in subtractions {
                    parameters.push(Self::eqn_to_ir(parameter, logger, depth + 1))
                }
                LatexEqnIR {
                    name: String::from("-"),
                    depth,
                    superscript: None,
                    subscript: None,
                    parameters,
                }
            }
            Equation::Multiplication(multiplications) => {
                let mut parameters = vec!();
                for parameter in multiplications {
                    parameters.push(Self::eqn_to_ir(parameter, logger, depth + 1))
                }
                LatexEqnIR {
                    name: String::from("*"),
                    depth,
                    superscript: None,
                    subscript: None,
                    parameters,
                }
            }
            Equation::Division(division) => {
                let division = *division;
                let parameters = vec!(Self::eqn_to_ir(division.0, logger, depth + 1), Self::eqn_to_ir(division.1, logger, depth + 1));
                LatexEqnIR { name: String::from("\\frac"), depth, superscript: None, subscript: None, parameters }
            }
            Equation::Equals(statements) => {
                let statements = *statements;
                let parameters = vec!(Self::eqn_to_ir(statements.0, logger, depth + 1), Self::eqn_to_ir(statements.1, logger, depth + 1));
                LatexEqnIR { name: String::from("="), depth, superscript: None, subscript: None, parameters }
            }
            Equation::Power(params) => {
                let params = *params;
                let parameters = vec!(Self::eqn_to_ir(params.0, logger, depth + 1), Self::eqn_to_ir(params.1, logger, depth + 1));
                LatexEqnIR { name: String::from("^"), depth, superscript: None, subscript: None, parameters }
            }
        }
    }
    pub fn ir_to_latex(ir: LatexEqnIR, logger: &Logger) -> String {
        let mut ir = ir;
        return match &*ir.name {
            "*" => {
                let mut latex = String::from(ir.parameters.remove(0).name_with_subscript());
                for parameters in ir.parameters.into_iter() {
                    latex.push('*');
                    latex.push_str(&*Self::ir_to_latex(parameters, logger))
                }
                return latex
            }
            "\\frac" => {
                return format!("\\frac{}{}", Self::ir_to_latex(ir.parameters.remove(0), logger), Self::ir_to_latex(ir.parameters.remove(0), logger))
            }

            _ => { todo!() }
        }
    }
    pub fn parse_float(number: String, logger : &Logger) -> Equation{
        let splits = number.split('.').collect::<Vec<_>>();
        if splits.len() != 2{
            crit!(logger, "Invalid number passed");
            panic!();
        }
        let (lhs, rhs) = (splits[0], splits[1]);
        let denominator: i32= 10i32.pow(rhs.len() as u32);
        let mut nominator = String::from(lhs);
        nominator.push_str(rhs);
        return Equation::Variable(Variable::Rational((nominator.parse::<i32>().unwrap(),denominator)));
    }
}
pub fn to_equation(latex: String, logger: &Logger) -> Equation {
    let ir = latex_to_ir(latex, logger);
    return ir_to_eqn(ir, logger);
}
pub fn to_latex(eqn: Equation, logger: &Logger) -> String {
    let ir = eqn_to_ir(eqn, logger);
    return LatexEqnIR::ir_to_latex(ir, logger);
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
fn eqn_to_ir(eqn: Equation, logger: &Logger) -> LatexEqnIR {
    return LatexEqnIR::eqn_to_ir(eqn, logger, 1);
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
fn print_data(logger: &Logger, data: String, depth: u32) {
    let mut formatted = String::new();
    for _ in 0..=depth {
        formatted.push_str(" | ")
    }
    formatted.push_str(&*data);
    debug!(logger, "{}", formatted);
}