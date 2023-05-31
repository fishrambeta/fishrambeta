use crate::math::Variable;
use std::{collections::HashMap, hash::Hash};

pub fn physics_values() -> HashMap<Variable, f64> {
    let mut values = HashMap::new();
    values.insert(Variable::Letter("g".to_string()), 9.81);
    values.insert(Variable::Letter("\\hbar".to_string()), 1.054571817e-34);
    return values;
}
