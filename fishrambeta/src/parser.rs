use crate::math::Equation;
use slog::{crit, Logger};

#[derive(Debug, Clone)]
struct LatexEqnIR {
    name: String,
    parameters: Vec<LatexEqnIR>,
    #[cfg(debug_assertions)]
    depth: u32,
}
impl LatexEqnIR {
    pub fn create_from_latex(
        latex: Vec<char>,
        logger: &Logger,
        #[cfg(debug_assertions)] depth: u32,
    ) -> Self {
        let mut latex = latex;
        //Handle equals sign
        return if latex.contains(&'='){
            let index = unsafe{latex.iter().position(|char| char == &'=').unwrap_unchecked()};
            let (left, right) = latex.split_at(index);
            let (left, mut right) = (left.to_vec(), right.to_vec());
            right.remove(0);
            if left.contains(&'=') || right.contains(&'=') {
                crit!(logger, "Equation contains multiple equals signs, unsupported");
                panic!();
            }
            #[cfg(debug_assertions)]
            let (left_ir, right_ir) = (Self::create_from_latex(left, logger, depth+1), Self::create_from_latex(right, logger, depth+1));
            #[cfg(not(debug_assertions))]
            let (left_ir, right_ir) = (Self::create_from_latex(left.to_vec(), logger), Self::create_from_latex(right.to_vec(), logger));
            return Self{
                name: String::from("equals"),
                parameters: vec!(left_ir, right_ir),
                #[cfg(debug_assertions)] depth,
            };
        }
        //Handle \*****{}{}{}{}{}... constructions
        else if latex.starts_with(&['\\']) && latex.contains(&'{') && latex.contains(&'}') {
            let mut name = vec![];
            latex.remove(0);
            loop {
                if latex[0] == '{' {
                    break;
                } else {
                    name.push(latex.remove(0))
                }
            }
            let name: String = name.iter().collect();
            let mut parameters = vec![];
            //Loop over individual parameters
            loop {
                if latex.len() == 0{
                    break;
                }
                if latex[0] != '{' {
                    break;
                }
                if latex.len() == 0 {
                    break;
                }
                latex.remove(0); //Remove trailing {
                let mut depth_counter = 1;
                let mut inner_data = vec![];
                loop {
                    // Loop until }
                    if latex[0] == '{' {
                        depth_counter += 1
                    }
                    else if latex[0] == '}' {
                        depth_counter += -1
                    }
                    if depth_counter == 0 {
                        latex.remove(0);
                        break;
                    }
                    inner_data.push(latex.remove(0))
                }
                #[cfg(debug_assertions)]
                let parameter = LatexEqnIR::create_from_latex(inner_data, logger, depth + 1);
                #[cfg(not(debug_assertions))]
                let parameter = LatexEqnIR::create_from_latex(inner_data, logger);
                parameters.push(parameter);
            }
            Self {
                name,
                parameters,
                #[cfg(debug_assertions)]
                depth,
            }
        }
        //Handle +- and other operators
        else if latex.contains(&'+') {
            let index = unsafe {
                latex
                    .iter()
                    .position(|char| char == &'+')
                    .unwrap_unchecked()
            };
            let (first, second) = latex.split_at(index);
            let (first, mut second) = (first.to_vec(), second.to_vec());
            second.remove(0);
            #[cfg(debug_assertions)]
            let (first_ir, second_ir) = (
                Self::create_from_latex(first, logger, depth + 1),
                Self::create_from_latex(second, logger, depth + 1),
            );
            #[cfg(not(debug_assertions))]
            let (first_ir, second_ir) = (
                Self::create_from_latex(first, logger),
                Self::create_from_latex(second, logger),
            );
            Self {
                name: String::from("subtraction"),
                parameters: vec![first_ir, second_ir],
                #[cfg(debug_assertions)]
                depth,
            }
        }
        else if latex.contains(&'-') {
            let index = unsafe {
                latex
                    .iter()
                    .position(|char| char == &'-')
                    .unwrap_unchecked()
            };
            let (first, second) = latex.split_at(index);
            let (first, mut second) = (first.to_vec(), second.to_vec());
            second.remove(0);
            #[cfg(debug_assertions)]
            let (first_ir, second_ir) = (
                Self::create_from_latex(first, logger, depth + 1),
                Self::create_from_latex(second, logger, depth + 1),
            );
            #[cfg(not(debug_assertions))]
            let (first_ir, second_ir) = (
                Self::create_from_latex(first, logger),
                Self::create_from_latex(second, logger),
            );
            Self {
                name: String::from("addition"),
                parameters: vec![first_ir, second_ir],
                #[cfg(debug_assertions)]
                depth,
            }
        }
        //Constants
        else {
            Self {
                name: latex.iter().collect(),
                parameters: vec![],
                #[cfg(debug_assertions)]
                depth,
            }
        };
    }
    pub fn ir_to_eqn(&self, logger: &Logger) -> Equation {
        todo!()
    }
}

pub fn to_equation(latex: String, logger: &Logger) -> Equation {
    let ir = latex_to_ir(latex, logger);
    return ir_to_eqn(ir, logger);
}
fn latex_to_ir(latex: String, logger: &Logger) -> LatexEqnIR {
    return LatexEqnIR::create_from_latex(latex.chars().filter(|char| char != &' ').collect(), logger, 0);
}
fn ir_to_eqn(ir: LatexEqnIR, logger: &Logger) -> Equation {
    return ir.ir_to_eqn(logger);
}
