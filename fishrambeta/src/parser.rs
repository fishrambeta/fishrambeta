use crate::math::Equation;
use crate::math::Variable::Letter;

pub fn to_equation(equation_string: String) -> Equation {
    let sub_equations_string = equation_string.split('+');
    let sub_equations_strings = sub_equations_string.collect::<Vec<&str>>();
    if sub_equations_strings.len() == 1 {
        return Equation::Variable(Letter(equation_string));
    }

    let mut sub_equations: Vec<Equation> = Vec::new();
    for sub_equation_string in sub_equations_strings {
        sub_equations.push(to_equation(sub_equation_string.to_string()));
    }
    return Equation::Addition(sub_equations);
}

