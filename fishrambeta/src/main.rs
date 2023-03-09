mod math;
mod parser;

use clap::Parser;
use crate::math::{Constant, Equation, Symbol, Variable};

#[derive(Parser, Debug)]
pub struct Args{
    //The equation to solve formatted in LaTeX
    #[arg(short, long)]
    equation : String,
}

fn main() {
    let simplified = Equation::Addition(
        vec!(
            Equation::Variable(Variable::Constant(Constant::PI)),
            Equation::Variable(Variable::Constant(Constant::PI)),
            Equation::Variable(Variable::Constant(Constant::PI)),
            Equation::Variable(Variable::Constant(Constant::PI)),
            Equation::Variable(Variable::Constant(Constant::E)),
            Equation::Variable(Variable::Constant(Constant::PI)),
            Equation::Variable(Variable::Constant(Constant::PI)),
            Equation::Variable(Variable::Constant(Constant::PI)),
            Equation::Variable(Variable::Constant(Constant::PI)),
            Equation::Variable(Variable::Constant(Constant::E)),
            Equation::Variable(Variable::Constant(Constant::E)),
            Equation::Variable(Variable::Constant(Constant::E)),
            Equation::Variable(Variable::Constant(Constant::PI)),
            Equation::Variable(Variable::Constant(Constant::PI)),
        )
    ).simplify();
    println!("{:?}", simplified)
    //let args = Args::parse();
}
