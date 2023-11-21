use crate::math::{Equation, Variable};

mod addition;
mod division;
mod multiplication;
mod power;

impl Equation {
    pub fn simplify_until_complete(self) -> Self {
        let mut equation = self.clone();
        let mut previous = equation.to_latex();
        for _ in 1..3 {
            equation = equation.simplify();
            if equation.to_latex() == previous {
                break;
            }
            previous = equation.to_latex();
        }
        return equation;
    }

    pub fn simplify_until_complete_with_print(self) -> Self {
        let mut equation = self.clone();
        let mut previous = equation.to_latex();
        for i in 1..5 {
            equation = equation.simplify();
            println!("{i}: {equation}");
            if equation.to_latex() == previous {
                break;
            }
            previous = equation.to_latex();
        }
        return equation;
    }

    pub(super) fn simplify(self) -> Self {
        let calculated_wrapped = self.calculate_exact();
        if calculated_wrapped.is_some() {
            let calculated = calculated_wrapped.unwrap();
            if calculated.is_integer() {
                return Equation::Variable(Variable::Integer(calculated.to_integer()));
            }
            return Equation::Variable(Variable::Rational(calculated));
        }
        match self {
            Equation::Variable(variable) => match variable {
                Variable::Rational(r) => {
                    return if r.is_integer() {
                        Equation::Variable(Variable::Integer(r.to_integer()))
                    } else {
                        Equation::Variable(Variable::Rational(r))
                    }
                }
                variable => return Equation::Variable(variable),
            },
            Equation::Negative(negative) => match *negative {
                Equation::Negative(negative) => return (*negative).simplify(),
                Equation::Variable(Variable::Integer(0)) => {
                    return Equation::Variable(Variable::Integer(0))
                }
                Equation::Variable(Variable::Integer(integer)) => {
                    return Equation::Variable(Variable::Integer(-integer))
                }
                Equation::Variable(Variable::Rational(rational)) => {
                    return Equation::Variable(Variable::Rational(-rational))
                }

                negative => return Equation::Negative(Box::new(negative.simplify())),
            },
            Equation::Addition(addition) => return addition::simplify_addition(addition),
            Equation::Multiplication(multiplication) => {
                multiplication::simplify_multiplication(multiplication)
            }
            Equation::Division(division) => return division::simplify_division(division),
            Equation::Power(power) => return power::simplify_power(power),
            Equation::Ln(ln) => return Equation::Ln(Box::new(ln.simplify())),
            Equation::Sin(sin) => return Equation::Sin(Box::new(sin.simplify())),
            Equation::Cos(cos) => return Equation::Cos(Box::new(cos.simplify())),
            Equation::Abs(abs) => return Equation::Abs(Box::new(abs.simplify())),
            Equation::Equals(equation) => {
                return Equation::Equals(Box::new((equation.0.simplify(), equation.1.simplify())))
            }
        }
    }
}
