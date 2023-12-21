use super::{Equation, Variable};
use num_integer::Integer;
use num_rational::Rational64;

pub(super) fn simplify_division(division: Box<(Equation, Equation)>) -> Equation {
    let mut numerator = division.0.simplify();
    let mut denominator = division.1.simplify();

    match numerator {
        Equation::Division(division) => {
            return Equation::Division(Box::new((
                division.0,
                Equation::Multiplication(vec![division.1, denominator]),
            )))
            .simplify();
        }
        Equation::Variable(Variable::Rational(rational)) => {
            return Equation::Division(Box::new((
                Equation::Variable(Variable::Integer(*rational.numer())),
                Equation::Multiplication(vec![
                    denominator,
                    Equation::Variable(Variable::Integer(*rational.denom())),
                ])
                .simplify(),
            )));
        }
        Equation::Multiplication(ref mut multiplication) => {
            if let Some(index) = multiplication
                .iter()
                .position(|x| matches!(x, Equation::Variable(Variable::Rational(_))))
            {
                let rational = if let Equation::Variable(Variable::Rational(r)) =
                    multiplication.remove(index)
                {
                    r
                } else {
                    unreachable!()
                };
                multiplication.push(Equation::Variable(Variable::Integer(*rational.numer())));
                return Equation::Division(Box::new((
                    Equation::Multiplication(multiplication.clone()),
                    Equation::Multiplication(vec![
                        denominator,
                        Equation::Variable(Variable::Integer(*rational.denom())),
                    ])
                    .simplify(),
                )));
            }
        }
        _ => {}
    }
    match denominator {
        Equation::Division(division) => {
            return Equation::Division(Box::new((
                Equation::Multiplication(vec![numerator, division.1]),
                division.0,
            )))
            .simplify();
        }
        Equation::Variable(Variable::Rational(rational)) => {
            return Equation::Division(Box::new((
                Equation::Multiplication(vec![
                    numerator,
                    Equation::Variable(Variable::Integer(*rational.denom())),
                ]),
                Equation::Variable(Variable::Integer(*rational.numer())),
            )));
        }
        Equation::Variable(Variable::Integer(n)) => {
            return Equation::Multiplication(vec![
                Equation::Variable(Variable::Rational(Rational64::new(1, n))),
                numerator,
            ])
        }
        Equation::Multiplication(ref mut multiplication) => {
            if let Some(index) = multiplication
                .iter()
                .position(|x| matches!(x, Equation::Variable(Variable::Rational(_))))
            {
                let rational = if let Equation::Variable(Variable::Rational(r)) =
                    multiplication.remove(index)
                {
                    r
                } else {
                    unreachable!()
                };
                multiplication.push(Equation::Variable(Variable::Integer(*rational.numer())));
                return Equation::Division(Box::new((
                    Equation::Multiplication(vec![
                        numerator,
                        Equation::Variable(Variable::Integer(*rational.denom())),
                    ])
                    .simplify(),
                    Equation::Multiplication(multiplication.clone()),
                )));
            }
        }

        _ => {}
    }

    for factor in denominator.shared_factors(&numerator) {
        if (&numerator).has_factor(&factor) && (&denominator).has_factor(&factor) {
            numerator = numerator.remove_factor(&factor);
            denominator = denominator.remove_factor(&factor);
        }
    }

    numerator = numerator.simplify();
    denominator = denominator.simplify();

    return if numerator == Equation::Variable(Variable::Integer(0)) {
        Equation::Variable(Variable::Integer(0))
    } else if denominator == Equation::Variable(Variable::Integer(1)) {
        numerator
    } else {
        Equation::Division(Box::new((numerator, denominator)))
    };
}
