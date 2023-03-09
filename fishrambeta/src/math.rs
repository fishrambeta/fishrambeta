///Represents a generic math object
pub enum Equation{
    Variable(Variable),
    Addition(Vec<Equation>),
    Subtraction(Vec<Equation>),
    Multiplication(Vec<Equation>),
    Division(Box<(Equation, Equation)>)
}
///Represents a single number
pub enum Variable{
    Integer(u32),
    Float(f32),
    Rational((u32,u32)),
}
///Mathematical constants
pub enum Constant{
    PI,
    E,
}
impl Symbol for Equation{
    fn simplify(self) -> Self {
        match self{
            Equation::Variable(variable) => { return Equation::Variable(variable) }
            Equation::Addition(_) => {}
            Equation::Subtraction(_) => {}
            Equation::Multiplication(_) => {}
            Equation::Division(_) => {}
        }
        return self
    }
}
trait Symbol{
    fn simplify(self) -> Self;
}
