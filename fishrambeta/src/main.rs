mod math;
mod parser;

use clap::Parser;
use crate::math::{Constant, Equation, Variable};

#[derive(Parser, Debug)]
pub struct Args{
    //The equation to solve formatted in LaTeX
    #[arg(short, long)]
    equation : String,
}

fn main() {
    let args = Args::parse();
    Equation::Addition(vec!(Equation::Variable(Vari)))
}
