mod latexifier;
mod math;
mod parser;

use crate::math::{Constant, Equation, Symbol, Variable};
use clap::Parser;
use clap::ValueEnum;

#[derive(Parser, Debug)]
pub struct Args {
    //The equation to solve formatted in LaTeX
    #[arg(short, long)]
    equation: String,
    //The thing to do with the equation
    #[arg(short, long, value_enum)]
    operation: Operation,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Operation {
    Simplify,
    Solve,
    Calculate
}

fn main() {
    let args = Args::parse();
    let equation = parser::to_equation(args.equation);
    let result = process_operation(equation, args.operation);
    println!("{}", result.to_string());
}

fn process_operation(equation: Equation, operation: Operation) -> Equation{
    match operation {
        Operation::Simplify => {return equation.simplify().simplify().simplify()}
        _ => {panic!("Operation not yet supported")}
    }
}
