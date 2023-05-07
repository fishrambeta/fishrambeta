mod math;
mod parser;
mod logger;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args{
    //The equation to solve formatted in LaTeX
    #[arg(short, long)]
    equation : String,
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
    #[arg(short, long)]
    log_out: Option<String>,
}

fn main() {
    let args = Args::parse();
    let logger = logger::new(args.log_out, args.verbose);
    let equation = parser::to_equation(args.equation, &logger);
    //let simplified = equation.simplify().simplify().simplify();
    //println!("{}", simplified.to_string());
}
