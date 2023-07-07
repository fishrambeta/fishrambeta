use crate::math::{Equation, Variable};
use std::iter;

impl Equation {
    pub fn multiply_by(self: Self, by: &Self) -> Self {
        match self {
            Equation::Variable(variable) => {
                return if Equation::Variable(variable.clone()) == *by {
                    Equation::Power(Box::new((
                        Equation::Variable(variable),
                        Equation::Variable(Variable::Integer(2)),
                    )))
                } else {
                    Equation::Multiplication(vec![by.clone(), Equation::Variable(variable)])
                }
            }
            Equation::Addition(addition) => {
                return Equation::Addition(
                    addition
                        .into_iter()
                        .map(|x| x.multiply_by(by))
                        .collect::<Vec<_>>(),
                )
            }
            Equation::Power(power) => {
                return if power.0 == *by {
                    Equation::Power(Box::new((
                        power.0,
                        Equation::Addition(vec![power.1, Equation::Variable(Variable::Integer(1))])
                            .simplify(),
                    )))
                } else {
                    Equation::Multiplication(vec![by.clone(), Equation::Power(power)])
                }
            }
            Equation::Multiplication(multiplication) => {
                if multiplication.len() == 1 {
                    return multiplication[0].clone().multiply_by(by);
                }
                let mut tmp = multiplication.clone();
                tmp.push(by.clone());
                return Equation::Multiplication(tmp);
            }
            equation => return Equation::Multiplication(vec![by.clone(), equation]),
        }
    }
}
