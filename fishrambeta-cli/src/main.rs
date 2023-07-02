use clap::Parser;
use clap::ValueEnum;
use fishrambeta::math::{Equation, Variable};
use fishrambeta::{logger, parser};
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
    let logger = logger::new(args.log_out, Some(args.verbose));
    let equation = parser::to_equation(args.equation, &logger);

    let mut value_dict: HashMap<Variable, f64> = HashMap::new();
    value_dict.insert(Variable::Letter("x".to_string()), 4.0);

    println!("{:?}", equation);
    let _result = process_operation(equation.clone(), args.operation, value_dict);
    //println!("{:?}", result);
    println!(
        "{}",
        equation
            .differentiate(&Variable::Letter("x".to_string()))
            .simplify()
            .simplify()
            .simplify()
            .simplify()
            .to_latex()
    );
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
