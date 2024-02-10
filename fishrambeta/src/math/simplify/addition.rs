use super::{Equation, Variable};
use num_rational::Rational64;
use std::collections::BTreeMap;

fn flatten_addition(addition: Vec<Equation>) -> Vec<Equation> {
    let mut new_addition = vec![];
    for term in addition {
        match term {
            Equation::Addition(a) => {
                new_addition.append(&mut flatten_addition(a));
            }
            other => new_addition.push(other),
        };
    }
    new_addition
}

pub(super) fn simplify_addition(mut addition: Vec<Equation>) -> Equation {
    if addition.len() == 1 {
        return addition.remove(0);
    }
    let addition = flatten_addition(addition);
    let mut total_rational_term: Rational64 = 0.into();
    let mut terms: BTreeMap<Equation, Rational64> = BTreeMap::new();
    let mut sin_squares: BTreeMap<Equation, Rational64> = BTreeMap::new();
    let mut cos_squares: BTreeMap<Equation, Rational64> = BTreeMap::new();

    for equation in addition.into_iter() {
        let (term, count) = match equation.simplify() {
            Equation::Variable(Variable::Integer(0)) => continue,
            Equation::Variable(Variable::Integer(i)) => {
                total_rational_term += i;
                continue;
            }
            Equation::Variable(Variable::Rational(r)) => {
                total_rational_term += r;
                continue;
            }
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
                if count == 0.into() {
                    // The multiplication is zero, so we can skip it
                    continue;
                }
                if multiplication.len() - number_of_numbers == 0 {
                    // The multiplication is a
                    // constant factor, so we add that factor to the addition
                    
                    // I'm sure this is some kind of mistake but I don't understand the code well enough to say what it is
                    (
                        Equation::Variable(Variable::Rational(count)),
                        Equation::Variable(Variable::Integer(1)),
                    );
                }
                let term: Vec<Equation> = multiplication
                    .into_iter()
                    .filter(|x| {
                        x.get_number_or_none().is_some()
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
    if total_rational_term != 0.into() {
        simplified_addition.push(Equation::Variable(Variable::Rational(total_rational_term)).simplify());
    }
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
                .push(Equation::Variable(Variable::Rational(number_of_ones)).simplify());
        }
        if sin_count != 0.into() {
            simplified_addition.push(
                Equation::Multiplication(vec![
                    Equation::Variable(Variable::Rational(sin_count)).simplify(),
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
                    Equation::Variable(Variable::Rational(cos_count)),
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
                Equation::Variable(Variable::Rational(cos_count)).simplify(),
                Equation::Power(Box::new((
                    Equation::Cos(Box::new(cos)),
                    Equation::Variable(Variable::Integer(2)),
                ))),
            ])
            .simplify(),
        );
    }

    if simplified_addition.is_empty() {
        return Equation::Variable(Variable::Integer(0));
    }

    Equation::Addition(simplified_addition)
}
