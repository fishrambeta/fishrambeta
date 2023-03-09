///Represents a generic math object
#[derive(PartialEq)]
pub enum Equation{
    Variable(Variable),
    Addition(Vec<Equation>),
    Subtraction(Vec<Equation>),
    Multiplication(Vec<Equation>),
    Division(Box<(Equation, Equation)>)
}
///Represents a single number
#[derive(PartialEq)]
pub enum Variable{
    Integer(u32),
    Float(f32),
    Rational((u32,u32)),
    Constant(Constant),
}
///Mathematical constants
#[derive(PartialEq)]
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
    let mut simplified_addition: Vec<Equation> = Vec::new();
    for equation in addition.iter(){
        for equation2 in addition.iter(){
            if equation == equation2{
                println!("Duplicate")
            }
        } 
    }
    return Equation::Addition(addition);
}
