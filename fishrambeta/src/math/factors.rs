use crate::math::{Equation, Variable};

impl Equation {
    pub fn has_factor(self: &Equation, factor: &Equation) -> bool {
        if self == factor {
            return true;
        }

        match self {
            //TODO Implement this for power
            //Equation::Power(power) => return power.0.has_factor(factor),
            Equation::Multiplication(multiplication) => {
                return multiplication.iter().any(|x| x.clone().has_factor(factor))
            }
            Equation::Addition(addition) => {
                return addition.iter().all(|x| x.clone().has_factor(factor))
            }
            Equation::Subtraction(subtraction) => {
                return subtraction.iter().all(|x| x.clone().has_factor(factor))
            }
            _ => return false,
        }
    }

    fn get_all_factors(self: &Equation) -> Vec<Equation> {
        //TODO add other factors than just multiplication
        let mut factors = vec![self.clone()];
        match self {
            Equation::Multiplication(multiplication) => factors.append(&mut multiplication.clone()),
            _ => {},
        }
        return factors;
    }

    pub fn shared_factors(self: &Equation, other: &Equation) -> Vec<Equation> {
        let factors = self.get_all_factors();
        let mut shared_factors = vec![];
        for factor in factors {
            if other.has_factor(&factor) {
                shared_factors.push(factor);
            }
        }
        return shared_factors;
    }

    pub fn remove_factor(self: Equation, factor: &Equation) -> Equation {
        if !self.has_factor(factor) {
            panic!("Trying to remove factor that's not a factor");
        }

        if self == *factor {
            return Equation::Variable(Variable::Integer(1));
        }

        match self {
            Equation::Multiplication(multiplication) => {
                return Equation::Multiplication(
                    multiplication
                        .iter()
                        .map(|x| {
                            if x.has_factor(factor) {
                                x.clone().remove_factor(factor)
                            } else {
                                x.clone()
                            }
                        })
                        .collect(),
                );
            }
            //TODO implement this for power
            //Equation::Power(power) => return power.0.has_factor(factor),
            Equation::Addition(addition) => {
                return Equation::Addition(
                    addition
                        .iter()
                        .map(|x| x.clone().remove_factor(factor))
                        .collect(),
                );
            }
            Equation::Subtraction(subtraction) => {
                return Equation::Subtraction(
                    subtraction
                        .iter()
                        .map(|x| x.clone().remove_factor(factor))
                        .collect(),
                );
            }
            _ => {}
        }

        return self;
    }
}
