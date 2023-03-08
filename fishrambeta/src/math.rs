///Represents a generic math object
pub enum Equation{
    Variable(Variable),
    Polynomial(Vec<Equation>),
}
///Represents a single number
pub enum Variable{
    Integer(u32),
    Float(f32),
    Rational((u32,u32)),
}
///Mathematical consts
pub enum Constant{
    PI,
    E,
}
impl Symbol for Equation{
    fn simplify(self) -> Self {
        match self{
            Equation::Variable(variable) => { return variable }
            Equation::Polynomial(_) => {}
        }
    }
}
trait Symbol{
    fn simplify(self) -> Self;
}