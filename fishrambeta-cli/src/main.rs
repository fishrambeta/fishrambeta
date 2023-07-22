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
    Differentiate,
}

#[derive(Debug)]
enum Result {
    Equation(Equation),
    Value(f64),
}

fn main() {
    let args = Args::parse();
    let equation = parser::IR::latex_to_equation(
        args.equation.chars().collect::<Vec<_>>(),
        args.implicit_multiplication,
    );

    println!("{}", equation.to_latex());
    let value_dict = fishrambeta::physicsvalues::physics_values();
    let result = process_operation(equation.clone(), args.operation, value_dict);
    println!("{:?}", result);
}

fn process_operation(
    equation: Equation,
    operation: Operation,
    value_dict: HashMap<Variable, f64>,
) -> Result {
    match operation {
        Operation::Simplify => {
            let mut equation = equation.clone();
            for _ in 0..10 {
                equation = equation.simplify();
                println!("{}", equation.to_latex());
            }
            return Result::Equation(equation);
        }
        Operation::Calculate => return Result::Value(equation.calculate(&value_dict)),
        Operation::Differentiate => {
            let mut equation = equation
                .clone()
                .differentiate(&Variable::Letter("x".to_string()));
            for _ in 0..10 {
                equation = equation.simplify();
                println!("{}", equation.to_latex());
            }
            return Result::Equation(equation);
        }
        _ => {
            panic!("Operation not yet supported")
        }
    }
}
