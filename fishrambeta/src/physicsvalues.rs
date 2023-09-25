use crate::math::{Constant, Variable};
use std::collections::BTreeMap;

pub fn physics_values() -> BTreeMap<Variable, f64> {
    let mut values = BTreeMap::new();
    values.insert(Variable::Letter("g".to_string()), 9.81);
    values.insert(Variable::Letter("\\hbar".to_string()), 1.054571817e-34);
    values.insert(Variable::Letter("m_e".to_string()), 9.1093837015e-31);
    values.insert(Variable::Letter("e_0".to_string()), 1.602176634e-19);
    values.insert(Variable::Letter("a_0".to_string()), 5.29177210903e-11);
    values.insert(
        Variable::Letter("\\epsilon_0".to_string()),
        8.8541878128e-12,
    );
    values.insert(Variable::Letter("\\mu_0".to_string()), 1.25663706212e-6);
    values.insert(Variable::Letter("c".to_string()), 299792458.);
    values.insert(Variable::Letter("h".to_string()), 6.62607015e-34);
    values.insert(Variable::Letter("G".to_string()), 6.6743015e-11);
    values.insert(Variable::Letter("k_e".to_string()), 8.9875517923e9);
    values.insert(Variable::Letter("k_B".to_string()), 1.380649e-23);
    values.insert(Variable::Letter("\\sigma".to_string()), 5.670374419e-8);
    values.insert(Variable::Letter("R".to_string()), 8.31446261815324);
    values.insert(Variable::Constant(Constant::PI), std::f64::consts::PI);
    values.insert(Variable::Constant(Constant::E), std::f64::consts::E);
    //values.insert(Variable::Letter("".to_string()), );
    return values;
}
