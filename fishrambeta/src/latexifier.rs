use crate::math::Constant;
use crate::math::Equation;
use crate::math::Variable;
use std::fmt;

impl fmt::Display for Equation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", equation_to_latex(self.clone()))
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

fn equation_to_latex(equation: Equation) -> String {
    match equation {
        Equation::Variable(variable) => return to_latex_variable(variable),
        Equation::Addition(addition) => return to_latex_addition(addition),
        // Equation::Subtraction(subtraction) => return to_latex_subtraction(subtraction),
        // Equation::Multiplication(multiplication) => return to_latex_multiplication(multiplication),
        // Equation::Division(division) => return to_latex_division(division),
        // Equation::Power(power) => return to_latex_power(power),
        _ => return "".to_string(),
    }
}

fn to_latex_variable(variable: Variable) -> String {
    match variable {
        Variable::Integer(integer) => return integer.to_string(),
        Variable::Rational(rational) => {
            return format!("\\frac{{{}}}{{{}}}", rational.0, rational.1)
        }
        Variable::Letter(letter) => return letter,
        Variable::Constant(constant) => return to_latex_constant(constant),
    }
}

fn to_latex_addition(addition: Vec<Equation>) -> String {
    let mut latex: String = "".to_string();
    for term in addition {
        latex.push_str(&format!("{}+", term));
    }
    return latex;
}

fn to_latex_constant(constant: Constant) -> String {
    match constant {
        Constant::E => return "e".to_string(),
        Constant::PI => return "\\pi".to_string()
    }
}
