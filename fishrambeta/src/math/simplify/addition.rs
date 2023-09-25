use super::{Equation, Variable};
use num_rational::Rational64;
use std::collections::BTreeMap;

pub(super) fn simplify_addition(mut addition: Vec<Equation>) -> Equation {
    if addition.len() == 1 {
        return addition.remove(0);
    }
    let mut terms: BTreeMap<Equation, Rational64> = BTreeMap::new();
    let mut sin_squares: BTreeMap<Equation, Rational64> = BTreeMap::new();
    let mut cos_squares: BTreeMap<Equation, Rational64> = BTreeMap::new();

    for equation in addition.into_iter() {
        let (term, count) = match equation.simplify() {
            Equation::Variable(Variable::Integer(0)) => continue,
            Equation::Multiplication(multiplication) => {
                let mut number_of_numbers = 0;
                let count: Rational64 = multiplication
                    .iter()
                    .filter_map(|x| -> Option<Rational64> {
                        if let Some(n) = x.get_number_or_none() {
                            number_of_numbers += 1;
                            Some(n)
                        } else {
                            None
                        }
                    })
                    .product();
                if count == 0.into() || multiplication.len() - number_of_numbers == 0 {
                    (multiplication, Equation::Variable(Variable::Integer(1)));
                    break;
                }
                let term: Vec<Equation> = multiplication
                    .into_iter()
                    .filter(|x| {
                        if let None = x.get_number_or_none() {
                            true
                        } else {
                            false
                        }
                    })
                    .collect();
                (Equation::Multiplication(term).simplify(), count)
            }
            Equation::Negative(negative) => (*negative, Rational64::new(-1, 1)),
            Equation::Power(box (Equation::Sin(sin), Equation::Variable(Variable::Integer(2)))) => {
                let previous_count = *sin_squares.get(&sin).unwrap_or(&0.into());
                sin_squares.insert(*sin, previous_count + 1);
                continue;
            }
            Equation::Power(box (Equation::Cos(cos), Equation::Variable(Variable::Integer(2)))) => {
                let previous_count = *cos_squares.get(&cos).unwrap_or(&0.into());
                cos_squares.insert(*cos, previous_count + 1);
                continue;
            }

            other => (other, 1.into()),
        };

        let previous_count = *terms.get(&term).unwrap_or(&0.into());
        terms.insert(term, previous_count + count);
    }

    let mut simplified_addition: Vec<Equation> = Vec::new();
    for (equation, count) in terms.into_iter() {
        if count == 1.into() {
            simplified_addition.push(equation);
        } else {
            let next_term = Equation::Multiplication(vec![
                equation,
                Equation::Variable(Variable::Rational(count)).simplify(),
            ])
            .simplify();
            simplified_addition.push(next_term);
        }
    }

    for (sin, mut sin_count) in sin_squares.into_iter() {
        let mut cos_count = *cos_squares.get(&sin).unwrap_or(&0.into());
        let number_of_ones = sin_count.min(cos_count);
        cos_squares.remove(&sin);
        sin_count -= number_of_ones;
        cos_count -= number_of_ones;
        if number_of_ones != 0.into() {
            simplified_addition
                .push(Equation::Variable(Variable::Rational(number_of_ones.into())).simplify());
        }
        if sin_count != 0.into() {
            simplified_addition.push(
                Equation::Multiplication(vec![
                    Equation::Variable(Variable::Rational(sin_count.into())).simplify(),
                    Equation::Power(Box::new((
                        Equation::Sin(Box::new(sin.clone())),
                        Equation::Variable(Variable::Integer(2)),
                    ))),
                ])
                .simplify(),
            );
        }
        if cos_count != 0.into() {
            simplified_addition.push(
                Equation::Multiplication(vec![
                    Equation::Variable(Variable::Rational(cos_count.into())),
                    Equation::Power(Box::new((
                        Equation::Cos(Box::new(sin)),
                        Equation::Variable(Variable::Integer(2)).simplify(),
                    ))),
                ])
                .simplify(),
            );
        }
    }
    for (cos, cos_count) in cos_squares.into_iter() {
        simplified_addition.push(
            Equation::Multiplication(vec![
                Equation::Variable(Variable::Rational(cos_count.into())).simplify(),
                Equation::Power(Box::new((
                    Equation::Cos(Box::new(cos)),
                    Equation::Variable(Variable::Integer(2)),
                ))),
            ])
            .simplify(),
        );
    }

    return Equation::Addition(simplified_addition);
}