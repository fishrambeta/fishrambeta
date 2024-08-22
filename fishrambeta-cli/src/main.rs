use clap::Parser;
use clap::ValueEnum;
use fishrambeta::math::steps::StepLogger;
use fishrambeta::math::{Equation, Variable};
use std::collections::BTreeMap;
use std::fmt;

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
    //Assume consecutive letters multiply variables with those names
    #[arg(long, default_value_t = false)]
    implicit_multiplication: bool,
    //Variables to propagate errors of, seperated by commas
    #[arg(long, default_value = "")]
    error_variables: String,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Operation {
    Simplify,
    Solve,
    Calculate,
    Differentiate,
    Integrate,
    Error,
}

#[derive(Debug)]
enum Result {
    Equation(Equation),
    Value(f64),
}

impl fmt::Display for Result {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Result::Equation(equation) => write!(f, "{}", equation),
            Result::Value(value) => write!(f, "{}", value),
        }
    }
}

fn main() {
    let args = Args::parse();
    let equation = Equation::from_latex(&args.equation, args.implicit_multiplication);
    println!("Input equation: {}", equation);

    use std::time::Instant;
    let now = Instant::now();
    let value_dict = fishrambeta::physicsvalues::physics_values();
    let mut step_logger = Some(StepLogger::new());
    let result = process_operation(
        equation.clone(),
        args.operation,
        &value_dict,
        &args.error_variables,
        &mut step_logger,
    );
    let elapsed = now.elapsed();
    println!("Steps: {:?}", step_logger);
    println!("Elapsed: {:.2?}", elapsed);
    println!("{}", result);
}

fn process_operation(
    equation: Equation,
    operation: Operation,
    value_dict: &BTreeMap<Variable, f64>,
    error_variables: &str,
    step_logger: &mut Option<StepLogger>,
) -> Result {
    match operation {
        Operation::Simplify => {
            let mut equation = equation.clone();
            equation = equation.simplify_until_complete_with_print(step_logger);
            println!("{}", equation.to_numpy());
            Result::Equation(equation)
        }
        Operation::Calculate => Result::Value(equation.calculate(value_dict)),
        Operation::Differentiate => {
            let mut equation = equation
                .clone()
                .differentiate(&Variable::Letter("x".to_string()));
            println!("Unsimplified: {}", equation);
            equation = equation.simplify_until_complete_with_print(step_logger);
            Result::Equation(equation)
        }
        Operation::Integrate => {
            println!("Start integrate");
            let mut equation = equation
                .clone()
                .integrate(&Variable::Letter("x".to_string()), step_logger);
            println!("Unsimplified: {}", equation);
            equation = equation.simplify_until_complete_with_print(step_logger);
            Result::Equation(equation)
        }
        Operation::Error => {
            let variables = error_variables
                .split(',')
                .map(|variable| Variable::Letter(variable.to_string()))
                .collect::<Vec<_>>();
            Result::Equation(
                equation
                    .error_analysis(variables, step_logger)
                    .simplify_until_complete_with_print(step_logger),
            )
        }
        _ => {
            panic!("Operation not yet supported")
        }
    }
}
