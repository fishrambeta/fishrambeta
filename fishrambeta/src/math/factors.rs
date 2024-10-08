use num_integer::Integer;

use crate::math::{Equation, Variable};

impl Equation {
    pub fn has_factor(self: &Equation, factor: &Equation) -> bool {
        if self == factor {
            return true;
        }
        if let Some(n) = factor.get_integer_or_none() {
            if n != 0 && self.gcd() % n == 0 {
                return true;
            }
        }

        match self {
            Equation::Power(power) => power.0 == *factor,
            Equation::Multiplication(multiplication) => {
                multiplication.iter().any(|x| x.clone().has_factor(factor))
            }
            Equation::Addition(addition) => addition.iter().all(|x| x.clone().has_factor(factor)),
            Equation::Negative(negative) => negative.has_factor(factor),
            _ => false,
        }
    }

    pub fn gcd(self: &Equation) -> i64 {
        match self {
            Equation::Variable(Variable::Integer(n)) => {
                if *n != 0 {
                    *n
                } else {
                    1
                }
            }
            Equation::Addition(addition) => {
                let mut gcd = 1;
                let mut first_done = false;
                for x in addition.iter().map(super::Equation::gcd) {
                    if first_done {
                        gcd = gcd.gcd(&x);
                    } else {
                        gcd = x;
                        first_done = true;
                    }
                }
                gcd
            }
            Equation::Multiplication(multiplication) => {
                return multiplication.iter().map(super::Equation::gcd).product()
            }
            Equation::Division(division) => division.0.gcd().gcd(&division.1.gcd()),
            _ => 1,
        }
    }

    fn get_all_factors(self: &Equation) -> Vec<Equation> {
        //TODO add other factors than just multiplication
        let mut factors = vec![self.clone()];
        match self {
            Equation::Multiplication(multiplication) => {
                for factor in multiplication {
                    factors.append(&mut factor.get_all_factors());
                }
            }
            Equation::Power(power) => factors.push(power.0.clone()),
            _ => {}
        }
        factors
    }

    pub fn get_factors(self: &Equation) -> Vec<Equation> {
        let mut factors: Vec<Equation> = self
            .get_all_factors()
            .into_iter()
            .filter(|x| self.has_factor(x))
            .collect();
        let gcd = self.gcd();
        if gcd != 1 {
            factors.push(Equation::Variable(Variable::Integer(gcd)));
        }
        factors
    }

    pub fn shared_factors(self: &Equation, other: &Equation) -> Vec<Equation> {
        let factors = self.get_all_factors();
        let mut shared_factors = vec![];
        for factor in factors {
            if other.has_factor(&factor) {
                shared_factors.push(factor);
            }
        }
        let shared_gcd = self.gcd().gcd(&other.gcd());
        if shared_gcd != 1 {
            shared_factors.push(Equation::Variable(Variable::Integer(shared_gcd)));
        }
        shared_factors
    }

    pub fn remove_factor(self: Equation, factor: &Equation) -> Equation {
        assert!(
            self.has_factor(factor),
            "Trying to remove factor that's not a factor"
        );

        if self == *factor {
            return Equation::Variable(Variable::Integer(1));
        }

        match self {
            Equation::Negative(negative) => {
                return Equation::Negative(Box::new(negative.remove_factor(factor)))
            }
            Equation::Multiplication(multiplication) => {
                let new: Vec<_> = multiplication
                    .iter()
                    .map(|x| {
                        if x.has_factor(factor) {
                            x.clone().remove_factor(factor)
                        } else {
                            x.clone()
                        }
                    })
                    .collect();
                return if new.is_empty() {
                    Equation::Variable(Variable::Integer(1))
                } else {
                    Equation::Multiplication(new)
                };
            }
            Equation::Power(power) => {
                return Equation::Power(Box::new((
                    power.0,
                    Equation::Addition(vec![power.1, Equation::Variable(Variable::Integer(-1))]),
                )))
            }
            Equation::Addition(addition) => {
                return Equation::Addition(
                    addition
                        .iter()
                        .map(|x| x.clone().remove_factor(factor))
                        .collect(),
                );
            }
            _ => {}
        }

        if let (Some(n), Some(n_factor)) =
            (self.get_integer_or_none(), factor.get_integer_or_none())
        {
            return Equation::Variable(Variable::Integer(n / n_factor));
        }

        self
    }
}
