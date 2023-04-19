mod math;
mod latexifier;
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
    let args = Args::parse();
    let equation = parser::to_equation(args.equation);
    let simplified = equation.simplify().simplify().simplify();
    println!("{}", simplified.to_string());
    println!("{:?}", simplified)
}
