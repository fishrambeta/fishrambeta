use crate::math::{Equation, Variable};

mod addition;
mod division;
mod multiplication;
mod power;

impl Equation {
    pub fn simplify_until_complete(mut self) -> Self {
        let mut previous = self.to_latex();
        for i in 1..100 {
            self = self.simplify();
            println!("{}: {}", i, self);
            if self.to_latex() == previous {
                break;
            }
            previous = self.to_latex();
        }
        return self;
    }
    pub(super) fn simplify(self) -> Self {
        let calculated_wrapped = self.calculate_exact();
        if calculated_wrapped.is_some() {
            let calculated = calculated_wrapped.unwrap();
            if calculated.is_integer() {
                return Equation::Variable(Variable::Integer(calculated.to_integer()));
            }
            return Equation::Variable(Variable::Rational((
                *calculated.numer(),
                *calculated.denom(),
            )));
        }
        match self {
            Equation::Variable(variable) => match variable {
                Variable::Rational((n, d)) => {
                    return if d == 1 {
                        Equation::Variable(Variable::Integer(n))
                    } else {
                        Equation::Variable(Variable::Rational((n, d)))
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
                    return Equation::Variable(Variable::Rational((-rational.0, rational.1)))
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
            Equation::Equals(equation) => {
                return Equation::Equals(Box::new((equation.0.simplify(), equation.1.simplify())))
            }
        }
    }
}
