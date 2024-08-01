use crate::math::{Constant, Equation, Variable};
use std::fmt;

impl Equation {
    pub fn from_latex(latex: &str) -> Equation {
        //Cleanup steps
        let mut cleaned_latex = latex
            .replace("\\left(", "(")
            .replace("\\right)", ")")
            .replace("\\cdot", "*").replace(" ", "");

        Equation::from_latex_internal(&cleaned_latex)
    }

    fn from_latex_internal(latex: &str) -> Equation {
        if let Some(stripped) = latex.strip_prefix("-") {
            return Equation::Negative(Box::new(Equation::from_latex_internal(stripped)));
        }

        if let Some((a, b)) = split_latex_at_operator(latex, &'+') {
            return Equation::Addition(vec![
                Equation::from_latex_internal(a),
                Equation::from_latex_internal(b),
            ]);
        }

        if let Some((a, b)) = split_latex_at_operator(latex, &'-') {
            return Equation::Addition(vec![
                Equation::from_latex_internal(a),
                Equation::Negative(Box::new(Equation::from_latex_internal(b))),
            ]);
        }

        if let Some((a, b)) = split_latex_at_operator(latex, &'*') {
            return Equation::Multiplication(vec![
                Equation::from_latex_internal(a),
                Equation::from_latex_internal(b),
            ]);
        }

        if let Some((a, b)) = split_latex_at_operator(latex, &'/') {
            return Equation::Division(Box::new((
                Equation::from_latex_internal(a),
                Equation::from_latex_internal(b),
            )));
        }

        if let Ok(num) = latex.parse::<i64>() {
            return Equation::Variable(Variable::Integer(num));
        }

        if let Some((left, right)) = latex.split_once(".") {
        if let (Ok(left_num), Ok(right_num)) = (left.parse::<i64>(), right.parse::<i64>()){
            assert!(right_num >= 0);
            // Plus 1 because ilog is rounded down
            let log = right.len();
            let denom = 10_i64.pow(log as u32);
            let numer = left_num*denom+right_num;
            return Equation::Variable(Variable::Rational((numer, denom).into()));
        }}

        if let Some(parameters) = parse_latex_with_command(latex, "\\frac") {
            assert_eq!(parameters.len(), 2);
            return Equation::Division(Box::new((
                Equation::from_latex_internal(parameters[0]),
                Equation::from_latex_internal(parameters[1]),
            )));
        }

        if is_in_redundant_brackets(latex) {
            return Equation::from_latex_internal(&latex[1..latex.len() - 1]);
        }

        if let Some((a, b)) = split_latex_at_operator(latex, &'^') {
            return Equation::Power(Box::new((
                Equation::from_latex_internal(a),
                Equation::from_latex_internal(b),
            )));
        }

        if let Some(parameters) = parse_latex_with_command(latex, "\\sin") {
            assert_eq!(parameters.len(), 1);
            return Equation::Sin(Box::new(Equation::from_latex_internal(parameters[0])));
        }

        if let Some(parameters) = parse_latex_with_command(latex, "\\cos") {
            assert_eq!(parameters.len(), 1);
            return Equation::Cos(Box::new(Equation::from_latex_internal(parameters[0])));
        }

        if let Some(parameters) = parse_latex_with_command(latex, "\\tan") {
            assert_eq!(parameters.len(), 1);
            return Equation::Division(Box::new((
                Equation::Sin(Box::new(Equation::from_latex_internal(parameters[0]))),
                Equation::Cos(Box::new(Equation::from_latex_internal(parameters[0]))),
            )));
        }

        if let Some(parameters) = parse_latex_with_command(latex, "\\ln") {
            assert_eq!(parameters.len(), 1);
            return Equation::Ln(Box::new(Equation::from_latex_internal(parameters[0])));
        }

        match latex {
            "\\pi" => Equation::Variable(Variable::Constant(Constant::PI)),
            "e" => Equation::Variable(Variable::Constant(Constant::E)),
            letter => Equation::Variable(Variable::Letter(letter.to_string())),
        }
    }

    pub fn to_latex(&self) -> String {
        return match self {
            Equation::Variable(v) => match v {
                Variable::Integer(i) => return i.to_string(),
                Variable::Rational(r) => {
                    return format!("\\frac{{{}}}{{{}}}", r.numer(), r.denom())
                }
                Variable::Constant(c) => match c {
                    Constant::PI => "\\pi".to_string(),
                    Constant::E => "e".to_string(),
                },
                Variable::Letter(l) => return l.to_string(),
                Variable::Vector(_) => todo!(),
            },
            Equation::Negative(n) => {
                if n.needs_to_be_bracketet() {
                    format!("-({})", n.to_latex())
                } else {
                    format!("-{}", n.to_latex())
                }
            }
            Equation::Addition(a) => a
                .iter()
                .map(|t| {
                    if t.needs_to_be_bracketet() {
                        format!("({})", t.to_latex())
                    } else {
                        t.to_latex()
                    }
                })
                .collect::<Vec<_>>()
                .join("+"),
            Equation::Multiplication(m) => m
                .iter()
                .map(|t| {
                    if t.needs_to_be_bracketet() {
                        format!("({})", t.to_latex())
                    } else {
                        t.to_latex()
                    }
                })
                .collect::<Vec<_>>()
                .join("*"),
            Equation::Division(d) => format!("\\frac{{{}}}{{{}}}", d.0, d.1),
            Equation::Power(p) => {
                let base = if p.0.needs_to_be_bracketet() {
                    format!("({})", p.0.to_latex())
                } else {
                    p.0.to_latex()
                };
                format!("{}^{{{}}}", base, p.1.to_latex())
            }
            Equation::Ln(l) => format!("\\ln({})", l),
            Equation::Equals(e) => format!("{}={}", e.0, e.1),
            Equation::Sin(s) => format!("\\sin({})", s),
            Equation::Cos(c) => format!("\\cos({})", c),
            Equation::Abs(a) => format!("|{}|", a),
            Equation::Derivative(_) => todo!(),
        };
    }

    fn needs_to_be_bracketet(&self) -> bool {
        match self {
            Equation::Variable(_) => false,
            Equation::Negative(_) => true,
            Equation::Addition(a) => a.len() != 1,
            Equation::Multiplication(m) => m.len() != 1,
            Equation::Division(_) => false,
            Equation::Power(_) => false,
            Equation::Ln(_) => false,
            Equation::Equals(_) => false,
            Equation::Sin(_) => false,
            Equation::Cos(_) => false,
            Equation::Abs(_) => false,
            Equation::Derivative(_) => true,
        }
    }

    pub fn to_numpy(&self) -> String {
        return match self {
            Equation::Variable(v) => match v {
                Variable::Integer(i) => return i.to_string(),
                Variable::Rational(r) => return format!("({})/({})", r.numer(), r.denom()),
                Variable::Constant(c) => match c {
                    Constant::PI => "np.pi".to_string(),
                    Constant::E => "np.e".to_string(),
                },
                Variable::Letter(l) => return l.to_string(),
                Variable::Vector(_) => todo!(),
            },
            Equation::Negative(n) => format!("-({})", n.to_latex()),
            Equation::Addition(a) => a
                .iter()
                .map(|t| format!("({})", t.to_latex()))
                .collect::<Vec<_>>()
                .join("+"),
            Equation::Multiplication(m) => m
                .iter()
                .map(|t| format!("({})", t.to_latex()))
                .collect::<Vec<_>>()
                .join("*"),
            Equation::Division(d) => format!("({})/({})", d.0, d.1),
            Equation::Power(p) => format!("np.power(({}),{{{}}})", p.0, p.1),
            Equation::Ln(l) => format!("np.log({})", l),
            Equation::Equals(e) => format!("{}={}", e.0, e.1),
            Equation::Sin(s) => format!("np.sin({})", s),
            Equation::Cos(c) => format!("np.cos({})", c),
            Equation::Abs(a) => format!("np.abs({})", a),
            Equation::Derivative(_) => todo!(),
        };
    }
}

fn is_in_redundant_brackets(latex: &str) -> bool {
    if !is_opening_bracket(latex.chars().next().expect("Latex string cannot be empty"))
        || !is_closing_bracket(latex.chars().last().expect("Latex string cannot be empty"))
    {
        return false;
    }
    let mut current_depth = 0;
    let length = latex.len();
    for (i, c) in latex.chars().enumerate() {
        if is_opening_bracket(c) {
            current_depth += 1
        }
        if is_closing_bracket(c) {
            current_depth -= 1
        }

        if current_depth == 0 {
            return i + 1 == length;
        }
    }
    todo!()
}

fn parse_latex_with_command<'a>(latex: &'a str, command: &'a str) -> Option<Vec<&'a str>> {
    if !latex.starts_with(command) {
        return None;
    }
    let stripped_latex = &latex[command.len()..];

    let mut current_depth = 0;
    let mut parameter_indices: Vec<usize> = vec![];
    for (i, c) in stripped_latex.chars().enumerate() {
        if current_depth == 0 {
            parameter_indices.push(i);
        }

        if is_opening_bracket(c) {
            current_depth += 1
        }
        if is_closing_bracket(c) {
            current_depth -= 1
        }
    }

    let mut parameters: Vec<&str> = vec![];
    for i in 0..parameter_indices.len() - 1 {
        parameters.push(&stripped_latex[parameter_indices[i] + 1..parameter_indices[i + 1] - 1]);
    }
    parameters.push(
        &stripped_latex
            [parameter_indices[parameter_indices.len() - 1] + 1..stripped_latex.len() - 1],
    );
    Some(parameters)
}

impl fmt::Display for Equation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_latex())
    }
}

fn split_latex_at_operator<'a>(latex: &'a str, operator: &'a char) -> Option<(&'a str, &'a str)> {
    let mut current_depth = 0;
    let mut right_start = latex.len();
    for (i, c) in latex.chars().rev().enumerate() {
        if is_opening_bracket(c) {
            current_depth += 1
        }
        if is_closing_bracket(c) {
            current_depth -= 1
        }

        if c == *operator && current_depth == 0 {
            right_start = right_start - i - 1;
            break;
        }
    }
    if right_start != latex.len() {
        Some((&latex[..right_start], &latex[right_start + 1..]))
    } else {
        None
    }
}

fn is_opening_bracket(c: char) -> bool {
    ['(', '{'].contains(&c)
}
fn is_closing_bracket(c: char) -> bool {
    [')', '}'].contains(&c)
}
