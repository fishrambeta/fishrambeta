use crate::math::{Equation, Variable};
use std::collections::BTreeMap;

impl Equation {
    pub fn simplify(self) -> Self {
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
            Equation::Variable(variable) => return Equation::Variable(variable),
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
            Equation::Addition(addition) => return simplify_addition(addition),
            Equation::Multiplication(multiplication) => simplify_multiplication(multiplication),
            Equation::Division(division) => return simplify_division(division),
            Equation::Power(power) => return simplify_power(power),
            Equation::Ln(ln) => return Equation::Ln(Box::new(ln.simplify())),
            Equation::Sin(sin) => return Equation::Sin(Box::new(sin.simplify())),
            Equation::Cos(cos) => return Equation::Cos(Box::new(cos.simplify())),
            Equation::Equals(equation) => {
                return Equation::Equals(Box::new((equation.0.simplify(), equation.1.simplify())))
            }
        }
    }
}

fn simplify_addition(addition: Vec<Equation>) -> Equation {
    if addition.len() == 1 {
        return addition[0].clone();
    }
    let mut terms: BTreeMap<Equation, i64> = BTreeMap::new();

    for equation in addition.iter() {
        let (term, count) = match equation.clone().simplify() {
            Equation::Variable(Variable::Integer(0)) => continue,
            Equation::Multiplication(multiplication) => {
                let count: i64 = multiplication
                    .iter()
                    .filter(|x| matches!(x, Equation::Variable(Variable::Integer(_))))
                    .map(|x| match *x {
                        Equation::Variable(Variable::Integer(i)) => i,
                        _ => unreachable!(),
                    })
                    .product();
                let term: Vec<Equation> = multiplication
                    .iter()
                    .filter(|x| !matches!(x, Equation::Variable(Variable::Integer(_))))
                    .cloned()
                    .collect();
                if count == 0 || term.len() == 0 {
                    (multiplication, Equation::Variable(Variable::Integer(1)));
                    break;
                }
                (Equation::Multiplication(term.clone()).simplify(), count)
            }
            Equation::Negative(negative) => (*negative, -1),
            other => (other, 1),
        };

        let previous_count = *terms.get(&term).unwrap_or(&0);
        terms.insert(term, previous_count + count);
    }

    let mut simplified_addition: Vec<Equation> = Vec::new();
    for (equation, count) in terms.iter() {
        let next_term = Equation::Multiplication(vec![
            equation.clone(),
            Equation::Variable(Variable::Integer(*count)),
        ])
        .simplify();
        simplified_addition.push(next_term);
    }

    return Equation::Addition(simplified_addition);
}

fn flatten_multiplication(multiplication: Vec<Equation>) -> Vec<Equation>{
    let mut new_mult = vec![];
    for term in multiplication {
        match term {
            Equation::Multiplication(m) => {new_mult.append(&mut flatten_multiplication(m));},
            other => {new_mult.push(other.clone())},
        };
    }
    return new_mult;
}

fn simplify_multiplication(multiplication: Vec<Equation>) -> Equation {
    let multiplication = flatten_multiplication(multiplication.clone());
    let mut terms: BTreeMap<Equation, i64> = BTreeMap::new();
    let mut total_integer_factor: i64 = 1;
    for equation in &multiplication {
        let (term, count) = match equation.clone().simplify() {
            Equation::Variable(Variable::Integer(0)) => {
                return Equation::Variable(Variable::Integer(0));
            }
            Equation::Variable(Variable::Integer(1)) => {
                continue;
            }
            Equation::Variable(Variable::Integer(n)) => {
                total_integer_factor = total_integer_factor * n;
                continue;
            }
            term => (term, 1),
        };
        let previous_count = *terms.get(&term).unwrap_or(&0);
        terms.insert(term, previous_count + count);
    }

    let mut simplified_multiplication: Vec<Equation> = match total_integer_factor {
        1 => vec![],
        n => vec![Equation::Variable(Variable::Integer(n))],
    };
    for (term, count) in terms {
        simplified_multiplication.push(
            Equation::Power(Box::new((
                term,
                Equation::Variable(Variable::Integer(count)),
            )))
            .simplify(),
        );
    }

    if simplified_multiplication.len() == 1 {
        return simplified_multiplication.remove(0);
    }

    return Equation::Multiplication(simplified_multiplication);
}

fn simplify_power(power: Box<(Equation, Equation)>) -> Equation {
    let base = power.0.simplify();
    let exponent = power.1.simplify();

    if exponent == Equation::Variable(Variable::Integer(1)) {
        return base;
    }

    match base {
        Equation::Multiplication(terms) => {
            let mut simplified_power: Vec<Equation> = vec![];
            for term in terms.iter() {
                simplified_power.push(Equation::Power(Box::new((term.clone(), exponent.clone()))));
            }
            return Equation::Multiplication(simplified_power);
        }
        Equation::Power(ref power) => {
            if let Equation::Variable(ref exponent_as_variable) = exponent {
                if let Equation::Variable(ref exponent2_as_variable) = power.1 {
                    if (matches!(exponent_as_variable, Variable::Integer(_))
                        && matches!(exponent2_as_variable, Variable::Integer(_)))
                        || (matches!(exponent_as_variable, Variable::Rational(_))
                            && matches!(exponent2_as_variable, Variable::Rational(_)))
                    {
                        return Equation::Power(Box::new((
                            power.0.clone(),
                            Equation::Multiplication(vec![exponent, power.1.clone()]),
                        )))
                        .simplify();
                    }
                }
            }
        }
        _ => {}
    }

    return Equation::Power(Box::new((base, exponent)));
}

fn simplify_division(division: Box<(Equation, Equation)>) -> Equation {
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
