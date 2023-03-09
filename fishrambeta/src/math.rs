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
    for equation in addition.iter(){
        terms.insert(equation.clone(), *terms.get(equation).unwrap_or(&0)+1);
    }
    let mut simplified_addition: Vec<Equation> = Vec::new();
    for (equation, count) in terms.iter(){
        if *count == 1{
            simplified_addition.push(equation.clone())
        }else{
            simplified_addition.push(
                Equation::Multiplication(vec!(Equation::Variable(Variable::Integer(*count)), equation.clone())));
        }
    }
    return Equation::Addition(simplified_addition);
}
