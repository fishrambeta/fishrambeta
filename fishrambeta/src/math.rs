use std::collections::HashMap;

///Represents a generic math object
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum Equation{
    Variable(Variable),
    Addition(Vec<Equation>),
    Subtraction(Vec<Equation>),
    Multiplication(Vec<Equation>),
    Division(Box<(Equation, Equation)>),
    Power(Box<(Equation, Equation)>)
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
            Equation::Multiplication(multiplication) => { simplify_multiplication(multiplication) }
            Equation::Division(division) => { return Equation::Division(division) }
            Equation::Power(power) => { return Equation::Power(power) }
        }
    }
}
pub trait Symbol{
    fn simplify(self) -> Self;
}

fn simpilify_addition(addition: Vec<Equation>) -> Equation{
    let mut terms: HashMap<Equation, i32> = HashMap::new();
    for equation in addition.iter(){
        let simplified = equation.clone().simplify();
        terms.insert(simplified.clone(), *terms.get(&simplified).unwrap_or(&0)+1);
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

fn simplify_multiplication(multiplication: Vec<Equation>) -> Equation{
    let mut terms: HashMap<Equation, i32> = HashMap::new();
    for equation in multiplication.iter(){
        let simplified = equation.clone().simplify();
        terms.insert(simplified.clone(), *terms.get(&simplified).unwrap_or(&0)+1);
    } 

    let mut simplified_multiplication: Vec<Equation> = Vec::new();
    for (equation, count) in terms.iter(){
        if *count == 1{
            simplified_multiplication.push(equation.clone())
        }else{
            simplified_multiplication.push(
                Equation::Power(Box::new((equation.clone(), Equation::Variable(Variable::Integer(*count))))));
        }
    }
    return Equation::Multiplication(simplified_multiplication);
}
