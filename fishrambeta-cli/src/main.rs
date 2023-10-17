use clap::Parser;
use clap::ValueEnum;
use fishrambeta::math::{Equation, Variable};
use fishrambeta::parser;
use num_rational::Rational64;
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
    propagate_variables: String,
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
    let equation = parser::IR::latex_to_equation(
        args.equation.chars().collect::<Vec<_>>(),
        args.implicit_multiplication,
    );

    println!("Input equation: {}", equation);
    use std::time::Instant;
    let now = Instant::now();
    let value_dict = fishrambeta::physicsvalues::physics_values();
    let result = process_operation(
        equation.clone(),
        args.operation,
        &value_dict,
        &args.propagate_variables,
    );
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("{}", result);
}

fn process_operation(
    equation: Equation,
    operation: Operation,
    value_dict: &BTreeMap<Variable, f64>,
    propagate_variables: &String,
) -> Result {
    match operation {
        Operation::Simplify => {
            let mut equation = equation.clone();
            equation = equation.simplify_until_complete();
            return Result::Equation(equation);
        }
        Operation::Calculate => return Result::Value(equation.calculate(&value_dict)),
        Operation::Differentiate => {
            let mut equation = equation
                .clone()
                .differentiate(&Variable::Letter("x".to_string()));
            println!("Unsimplified: {}", equation);
            equation = equation.simplify_until_complete();
            return Result::Equation(equation);
        }
        Operation::Integrate => {
            let mut equation = equation
                .clone()
                .integrate(&Variable::Letter("x".to_string()));
            println!("Unsimplified: {}", equation);
            equation = equation.simplify_until_complete();
            return Result::Equation(equation);
        }
        Operation::Error => {
            let variables = propagate_variables.split(",").collect::<Vec<_>>();
            let mut terms: Vec<Equation> = Vec::new();
            for variable in variables {
                let mut derivative =
                    equation.differentiate(&Variable::Letter(variable.to_string()));
                derivative = derivative.simplify_until_complete();
                let term = Equation::Power(Box::new((
                    Equation::Multiplication(vec![
                        derivative,
                        Equation::Variable(Variable::Letter(format!("s_{}", variable.to_string()))),
                    ]),
                    Equation::Variable(Variable::Integer(2)),
                )));
                terms.push(term);
            }
            let mut result = Equation::Power(Box::new((
                Equation::Addition(terms),
                Equation::Variable(Variable::Rational(Rational64::new(1, 2))),
            )));
            result = result.simplify_until_complete();
            Result::Equation(result)
        }
        _ => {
            panic!("Operation not yet supported")
        }
    }
}
