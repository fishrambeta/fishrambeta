/*
impl PartialEq for Equation {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Equation::Variable(v1) => {
                if let Equation::Variable(v2) = other {
                    return v1 == v2;
                } else {
                    return false;
                }
            }
            Equation::Addition(a1) => {
                if let Equation::Addition(a2) = other {
                    return a1.iter().zip(a2).all(|(a, b)| a == b);
                } else {
                    return false;
                }
            }
            Equation::Multiplication(m1) => {
                if let Equation::Multiplication(m2) = other {
                    return m1.iter().zip(m2).all(|(a, b)| a == b);
                } else {
                    return false;
                }
            }
            Equation::Power(p1) => {
                if let Equation::Power(p2) = other {
                    return p1.0 == p2.0 && p1.1 == p2.1;
                } else {
                    return false;
                }
            }
            Equation::Division(d1) => {
                if let Equation::Division(d2) = other {
                    return d1.0 == d2.0 && d1.1 == d2.1;
                } else {
                    return false;
                }
            }
            Equation::Negative(n1) => {
                if let Equation::Negative(n2) = other {
                    return n1 == n2;
                } else {
                    return false;
                }
            }
            Equation::Sin(s1) => {
                if let Equation::Sin(s2) = other {
                    return s1 == s2;
                } else {
                    return false;
                }
            }
            Equation::Cos(c1) => {
                if let Equation::Cos(c2) = other {
                    return c1 == c2;
                } else {
                    return false;
                }
            }
            Equation::Ln(l1) => {
                if let Equation::Ln(l2) = other {
                    return l1 == l2;
                } else {
                    return false;
                }
            }
            Equation::Equals(e1) => {
                if let Equation::Equals(e2) = other {
                    return e1 == e2;
                } else {
                    return false;
                }
            }
        }
    }
}

impl PartialEq for Variable {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Variable::Letter(l1) => {
                if let Variable::Letter(l2) = other {
                    return l1 == l2;
                } else {
                    return false;
                }
            }
            Variable::Vector(v1) => {
                if let Variable::Vector(v2) = other {
                    return v1 == v2;
                } else {
                    return false;
                }
            }
            Variable::Integer(i1) => {
                if let Some(i2) = other.get_number_or_none() {
                    return Rational64::new(*i1, 1) == i2;
                } else {
                    return false;
                }
            }
            Variable::Rational(r1) => {
                if let Some(r2) = other.get_number_or_none() {
                    return Rational64::new((*r1).0, (*r1).1) == r2;
                } else {
                    return false;
                }
            }
            Variable::Constant(c1) => {
                if let Variable::Constant(c2) = other {
                    return c1 == c2;
                } else {
                    return false;
                }
            }
        }
    }
}*/
