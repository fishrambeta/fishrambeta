use crate::math::Variable;
use std::{collections::HashMap, hash::Hash};

pub fn physics_values() -> HashMap<Variable, f64> {
    let mut values = HashMap::new();
    values.insert(Variable::Letter("g".to_string()), 9.81);
    return values;
}
