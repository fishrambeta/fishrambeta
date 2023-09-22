use super::{Equation, Variable};
use num_rational::Rational64;
use std::collections::BTreeMap;

pub(super) fn simplify_addition(mut addition: Vec<Equation>) -> Equation {
    if addition.len() == 1 {
        return addition.remove(0);
    }
    let mut terms: BTreeMap<Equation, Rational64> = BTreeMap::new();

    for equation in addition.into_iter() {
        let (term, count) = match equation.simplify() {
            Equation::Variable(Variable::Integer(0)) => continue,
            Equation::Multiplication(multiplication) => {
                let count: Rational64 = multiplication
                    .iter()
                    .filter_map(|x| -> Option<Rational64> {
                        match x {
                            Equation::Variable(Variable::Integer(i)) => Some(Rational64::from(*i)),
                            Equation::Variable(Variable::Rational(r)) => {
                                Some(Rational64::new(r.0, r.1))
                            }
                            _ => None,
                        }
                    })
                    .product();
                let term: Vec<Equation> = multiplication
                    .iter()
                    .filter(|x| {
                        !matches!(x, Equation::Variable(Variable::Integer(_)))
                            && !matches!(x, Equation::Variable(Variable::Rational(_)))
                    })
                    .cloned()
                    .collect();
                if count == 0.into() || term.len() == 0 {
                    (multiplication, Equation::Variable(Variable::Integer(1)));
                    break;
                }
                (Equation::Multiplication(term).simplify(), count)
            }
            Equation::Negative(negative) => (*negative, Rational64::new(-1, 1)),
            other => (other, 1.into()),
        };

        let previous_count = *terms.get(&term).unwrap_or(&0.into());
        terms.insert(term, previous_count + count);
    }

    let mut simplified_addition: Vec<Equation> = Vec::new();
    for (equation, count) in terms.into_iter() {
        let next_term = Equation::Multiplication(vec![
            equation,
            Equation::Variable(Variable::Rational((*count.numer(), *count.denom()))).simplify(),
        ])
        .simplify();
        simplified_addition.push(next_term);
    }

    return Equation::Addition(simplified_addition);
}
