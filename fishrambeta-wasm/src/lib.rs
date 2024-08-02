use fishrambeta::{
    self,
    math::{Equation, Variable},
    physicsvalues,
};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn simplify(equation: &str) -> String {
    let parsed = Equation::from_latex(equation, true);
    let simplified = parsed.simplify_until_complete();

    simplified.to_latex()
}

#[wasm_bindgen]
pub fn differentiate(equation: &str) -> String {
    let parsed = Equation::from_latex(equation, true);
    let differentiated = parsed
        .differentiate(&Variable::Letter("x".to_string()))
        .simplify_until_complete();

    differentiated.to_latex()
}

#[wasm_bindgen]
pub fn integrate(equation: &str) -> String {
    let parsed = Equation::from_latex(equation, true);
    let integrated = parsed
        .integrate(&Variable::Letter("x".to_string()))
        .simplify_until_complete();

    integrated.to_latex()
}

#[wasm_bindgen]
pub fn calculate(equation: &str, user_values_keys: &str, user_values_values: &[f64]) -> f64 {
    console_error_panic_hook::set_once();
    let mut values = physicsvalues::physics_values();
    let user_values_hashmap = user_values_to_hashmap(
        user_values_keys.split("\\n\\n").collect::<Vec<_>>(),
        user_values_values,
    );
    values.extend(user_values_hashmap);
    let parsed: Equation = Equation::from_latex(equation, true);

    parsed.calculate(&values)
}

fn user_values_to_hashmap(keys: Vec<&str>, values: &[f64]) -> HashMap<Variable, f64> {
    let mut values_hashmap = HashMap::new();
    for (key, value) in keys.iter().zip(values.iter()) {
        values_hashmap.insert(Variable::Letter(key.to_string()), *value);
    }
    values_hashmap
}
