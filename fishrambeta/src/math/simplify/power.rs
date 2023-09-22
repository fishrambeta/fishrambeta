use super::{Equation, Variable};

pub(super) fn simplify_power(power: Box<(Equation, Equation)>) -> Equation {
    let base = power.0.simplify();
    let exponent = power.1.simplify();

    if let Some(n) = exponent.get_number_or_none() {
        if n == 1.into() {
            return base;
        } else if n == 0.into() {
            return Equation::Variable(Variable::Integer(1));
        }
    }

    match base {
        Equation::Multiplication(terms) => {
            let mut simplified_power: Vec<Equation> = vec![];
            for term in terms.into_iter() {
                simplified_power.push(Equation::Power(Box::new((term, exponent.clone()))));
            }
            return Equation::Multiplication(simplified_power);
        }
        Equation::Power(ref power) => {
            if let Some(n1) = exponent.get_number_or_none() {
                if let Some(n2) = power.1.get_number_or_none() {
                    return Equation::Power(Box::new((
                        power.0.clone(),
                        Equation::Variable(Variable::Rational((
                            *(n1 * n2).numer(),
                            *(n1 * n2).denom(),
                        ))),
                    )));
                }
            }
            if let Some(e1) = exponent.get_number_or_none() {
                if let Some(e2) = power.1.get_number_or_none() {
                    return Equation::Power(Box::new((
                        power.0.clone(),
                        Equation::Variable(Variable::Rational((
                            *(e1 * e2).numer(),
                            *(e1 * e2).denom(),
                        ))),
                    )));
                }
            }
        }
        _ => {}
    }

    return Equation::Power(Box::new((base, exponent)));
}
