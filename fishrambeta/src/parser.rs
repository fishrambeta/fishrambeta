use crate::math::Constant;
use crate::math::Equation;
use crate::math::Variable;
use crate::math::Variable::Letter;

pub fn to_equation(equation_string: String) -> Equation {
    let sub_equations_string = equation_string.split('+');
    let sub_equations_strings = sub_equations_string.collect::<Vec<&str>>();
    if sub_equations_strings.len() == 1 {
        return only_letters_to_equation(sub_equations_strings[0].to_string());
    }

    let mut sub_equations: Vec<Equation> = Vec::new();
    for sub_equation_string in sub_equations_strings {
        sub_equations.push(to_equation(sub_equation_string.to_string()));
    }

    return Equation::Addition(sub_equations);
}

pub fn only_letters_to_equation(letters: String) -> Equation {
    let mut variables: Vec<Equation> = Vec::new();
    for (_i, c) in letters.chars().enumerate() {
        match c {
            'e' | 'E' => variables.push(Equation::Variable(Variable::Constant(Constant::E))),
            _ => variables.push(Equation::Variable(Letter(c.to_string()))),
        }
    }

    return Equation::Multiplication(variables);
}
