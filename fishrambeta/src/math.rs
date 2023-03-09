use std::collections::HashMap;

///Represents a generic math object
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum Equation{
    Variable(Variable),
    Addition(Vec<Equation>),
    Subtraction(Vec<Equation>),
    Multiplication(Vec<Equation>),
    Division(Box<(Equation, Equation)>)
}
///Represents a single number
#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum Variable{
    Integer(i32),
    Rational((i32,i32)),
    Constant(Constant),
}
///Mathematical constants
#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum Constant{
    PI,
    E,
}

impl Symbol for Equation{
    fn simplify(self) -> Self {
        match self{
            Equation::Variable(variable) => { return Equation::Variable(variable) }
            Equation::Addition(addition) => { return simpilify_addition(addition) }
            Equation::Subtraction(subtraction) => { return Equation::Subtraction(subtraction) }
            Equation::Multiplication(multiplication) => { return Equation::Multiplication(multiplication) }
            Equation::Division(division) => { return Equation::Division(division) }
        }
    }
}
pub trait Symbol{
    fn simplify(self) -> Self;
}

fn simpilify_addition(addition: Vec<Equation>) -> Equation{
    let mut terms: HashMap<Equation, i32> = HashMap::new();
    for (i, equation) in addition.iter().enumerate(){
        terms.insert(equation.clone(), *terms.get(equation).unwrap_or(&1));
        for (j, equation2) in addition.iter().enumerate(){
            if equation == equation2 && i == j{
                terms.insert(equation.clone(), terms[equation]+1);
            }
        } 
    }
    let mut simplified_addition: Vec<Equation> = Vec::new();
    for (equation, count) in terms.iter()  {
        simplified_addition.push(
            Equation::Multiplication(vec!(Equation::Variable(Variable::Integer(count-1)), equation.clone())));
    }
    return Equation::Addition(simplified_addition);
}
