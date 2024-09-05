use super::Equation;
use super::Variable;

impl Equation {
    pub fn is_constant(&self, variable: &Variable) -> bool {
        match self {
            Equation::Addition(a) => a.iter().all(|x| x.is_constant(variable)),
            Equation::Multiplication(m) => m.iter().all(|x| x.is_constant(variable)),
            Equation::Negative(n) => n.is_constant(variable),
            Equation::Division(d) => d.0.is_constant(variable) && d.1.is_constant(variable),
            Equation::Power(p) => p.0.is_constant(variable) && p.1.is_constant(variable),
            Equation::Sin(t) => t.is_constant(variable),
            Equation::Cos(t) => t.is_constant(variable),
            Equation::Ln(t) => t.is_constant(variable),
            Equation::Equals(_) => panic!("Equation containing = cannot be integrated"),
            Equation::Variable(v) => v != variable,
            Equation::Abs(a) => a.is_constant(variable),
            Equation::Derivative(_) => {
                panic!("Derivative cannot be integrated")
            }
        }
    }

    pub fn is_polynomial(&self, variable: &Variable) -> bool {
        match self {
            Equation::Addition(a) => return a.iter().all(|x| x.is_polynomial(variable)),
            Equation::Power(p) => {
                (p.0 == Equation::Variable(variable.clone()))
                    && (p.1.get_integer_or_none().is_some())
            }
            Equation::Negative(n) => n.is_polynomial(variable),
            Equation::Variable(_) => true,
            Equation::Multiplication(m) => return m.iter().all(|x| x.is_polynomial(variable)),
            _ => self.is_constant(variable),
        }
    }

    pub fn is_rational(&self, variable: &Variable) -> bool {
        self.term_is_rational_function(variable, false)
    }

    fn term_is_rational_function(&self, variable: &Variable, is_in_division: bool) -> bool {
        if self.is_polynomial(variable) {
            return true;
        }
        if is_in_division {
            return self.is_polynomial(variable);
        }

        match self {
            Equation::Addition(a) => {
                return a
                    .iter()
                    .all(|x| x.term_is_rational_function(variable, is_in_division))
            }
            Equation::Multiplication(m) => {
                return m
                    .iter()
                    .all(|x| x.term_is_rational_function(variable, is_in_division))
            }
            Equation::Division(d) => {
                d.0.term_is_rational_function(variable, true)
                    && d.1.term_is_rational_function(variable, true)
            }
            misc => misc.is_constant(variable),
        }
    }

    pub fn is_linear(&self, variable: &Variable) -> bool {
        match self {
            Equation::Negative(n) => n.is_linear(variable),
            Equation::Addition(a) => a.iter().all(|x| x.is_linear(variable)),
            Equation::Multiplication(m) => m.iter().all(|x| x.is_linear(variable)),
            Equation::Variable(_) => true,
            _ => self.is_constant(variable),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_linear() {
        assert!(!Equation::from_latex("x^2", false).is_linear(&Variable::Letter("x".to_string())));
        assert!(
            Equation::from_latex("x+3y^2-5x", false).is_linear(&Variable::Letter("x".to_string()))
        );
        assert!(Equation::from_latex("x+\\sin(y^2)-4x", false)
            .is_linear(&Variable::Letter("x".to_string())));
        assert!(!Equation::from_latex("\\sin(5*x)+4x", false)
            .is_linear(&Variable::Letter("x".to_string())));
    }
}
