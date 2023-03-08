use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args{
    //The equation to solve formatted in LaTeX
    #[arg(short, long)]
    equation : String,
}

fn main() {
    let args = Args::parse();
}
