use crate::math::{Constant, Equation, Variable};

impl Equation {
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
            Equation::Ln(l) => format!("\\ln({l})"),
            Equation::Equals(e) => format!("{}={}", e.0, e.1),
            Equation::Sin(s) => format!("\\sin({s})"),
            Equation::Cos(c) => format!("\\cos({c})"),
            Equation::Abs(a) => format!("|{a}|"),
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
            Equation::Ln(l) => format!("np.log({l})"),
            Equation::Equals(e) => format!("{}={}", e.0, e.1),
            Equation::Sin(s) => format!("np.sin({s})"),
            Equation::Cos(c) => format!("np.cos({c})"),
            Equation::Abs(a) => format!("np.abs({a})"),
            Equation::Derivative(_) => todo!(),
        };
    }
}
