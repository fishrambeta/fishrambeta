use clap::Parser;
use clap::ValueEnum;
use fishrambeta::math::{Equation, Variable};
use fishrambeta::parser;
use std::collections::HashMap;

#[derive(Parser, Debug)]
pub struct Args {
    //The equation to solve formatted in LaTeX
    #[arg(short, long)]
    equation: String,
    //The thing to do with the equation
    #[arg(short, long, value_enum)]
    operation: Operation,
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
    #[arg(short, long)]
    log_out: Option<String>,
    #[arg(long, default_value_t = false)]
    implicit_multiplication: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Operation {
    Simplify,
    Solve,
    Calculate,
}

#[derive(Debug)]
enum Result {
    Equation(Equation),
    Value(f64),
}

fn main() {
    let args = Args::parse();
    let mut equation = parser::IR::latex_to_equation(
        args.equation.chars().collect::<Vec<_>>(),
        args.implicit_multiplication,
    );

    println!("{}", equation.to_latex());
    //let _result = process_operation(equation.clone(), args.operation, value_dict);

    for _ in 0..10 {
        equation = equation.simplify();
        println!("{}", equation.to_latex());
    }
}

fn process_operation(
    equation: Equation,
    operation: Operation,
    value_dict: HashMap<Variable, f64>,
) -> Result {
    match operation {
        Operation::Simplify => return Result::Equation(equation.simplify().simplify().simplify()),
        Operation::Calculate => return Result::Value(equation.calculate(&value_dict)),
        _ => {
            panic!("Operation not yet supported")
        }
    }
}
