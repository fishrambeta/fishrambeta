use crate::math::{Constant, Variable};
use std::collections::BTreeMap;

pub fn physics_values() -> BTreeMap<Variable, f64> {
    let mut values = BTreeMap::new();
    values.insert(Variable::Letter("g".to_string()), 9.81);
    values.insert(Variable::Letter("\\hbar".to_string()), 1.054_571_817e-34);
    values.insert(Variable::Letter("m_e".to_string()), 9.109_383_701_5e-31);
    values.insert(Variable::Letter("m_p".to_string()), 1.672_621_58e-27);
    values.insert(Variable::Letter("m_n".to_string()), 1.674_927_498_04e-27);

    values.insert(Variable::Letter("e_0".to_string()), 1.602_176_634e-19);
    values.insert(Variable::Letter("a_0".to_string()), 5.291_772_109_03e-11);
    values.insert(
        Variable::Letter("\\epsilon_0".to_string()),
        8.854_187_812_8e-12,
    );
    values.insert(Variable::Letter("\\mu_0".to_string()), 1.256_637_062_12e-6);
    values.insert(Variable::Letter("c".to_string()), 299_792_458.);
    values.insert(Variable::Letter("h".to_string()), 6.626_070_15e-34);
    values.insert(Variable::Letter("G".to_string()), 6.674_301_5e-11);
    values.insert(Variable::Letter("k_e".to_string()), 8.987_551_792_3e9);
    values.insert(Variable::Letter("k_B".to_string()), 1.380_649e-23);
    values.insert(Variable::Letter("\\sigma".to_string()), 5.670_374_419e-8);
    values.insert(Variable::Letter("R".to_string()), 8.314_462_618_153_24);
    values.insert(Variable::Letter("M_{\\odot}".to_string()), 1.988_416e30);
    values.insert(Variable::Letter("R_{\\odot}".to_string()), 6.95700e8);
    values.insert(Variable::Letter("M_{\\oplus}".to_string()), 5.972e24);
    values.insert(Variable::Letter("R_{\\oplus}".to_string()), 6371e3);

    values.insert(Variable::Constant(Constant::PI), std::f64::consts::PI);
    values.insert(Variable::Constant(Constant::E), std::f64::consts::E);
    //values.insert(Variable::Letter("".to_string()), );
    values
}
