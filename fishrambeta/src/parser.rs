use crate::math::Constant;
use crate::math::Equation;
use crate::math::Variable;
use crate::math::Variable::Letter;

pub fn to_equation(equation_string: String) -> Equation {
    if equation_string.contains("*"){
        let sub_equations_string = equation_string.split('*');
        let sub_equations_strings = sub_equations_string.collect::<Vec<&str>>();

        let mut sub_equations: Vec<Equation> = Vec::new();
        for sub_equation_string in sub_equations_strings {
            sub_equations.push(to_equation(sub_equation_string.to_string()));
        }
        return Equation::Multiplication(sub_equations);
    }

    if equation_string.contains("/"){
        let sub_equations_string = equation_string.split('/');
        let sub_equations_strings = sub_equations_string.collect::<Vec<&str>>();
        return Equation::Division(Box::new((to_equation(sub_equations_strings[0].to_string()), to_equation(sub_equations_strings[1].to_string()))));
    }

    if equation_string.contains("+"){
        let sub_equations_string = equation_string.split('+');
        let sub_equations_strings = sub_equations_string.collect::<Vec<&str>>();

        let mut sub_equations: Vec<Equation> = Vec::new();
        for sub_equation_string in sub_equations_strings {
            sub_equations.push(to_equation(sub_equation_string.to_string()));
        }
        return Equation::Addition(sub_equations);
    }

    if equation_string.contains("^") {
        let sub_equations_string = equation_string.split('^');
        let sub_equations_strings = sub_equations_string.collect::<Vec<&str>>();
        return Equation::Power(Box::new((to_equation(sub_equations_strings[0].to_string()), to_equation(sub_equations_strings[1].to_string()))));
    }

    return only_letters_to_equation(equation_string);
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
