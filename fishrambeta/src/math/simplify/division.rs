use super::{Equation, Variable};

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
