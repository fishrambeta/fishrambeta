use super::{Equation, Variable};

pub(super) fn simplify_division(division: Box<(Equation, Equation)>) -> Equation {
    let mut numerator = division.0.simplify();
    let mut denominator = division.1.simplify();

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
