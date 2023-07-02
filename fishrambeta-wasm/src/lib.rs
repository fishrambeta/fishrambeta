use fishrambeta::{self, math::Variable, physicsvalues};
use slog::{o, Discard, Logger};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn simplify(equation: &str) -> String {
    let logger = Logger::root(Discard, o!());
    let parsed = fishrambeta::parser::to_equation(equation.to_string(), &logger);
    let simplified = parsed.simplify().simplify();
    let parsed_back = fishrambeta::parser::to_latex(simplified, &logger);
    return parsed_back;
}

#[wasm_bindgen]
pub fn calculate(equation: &str, user_values_keys: &str, user_values_values: &[f64]) -> f64 {
    let logger = Logger::root(Discard, o!());
    let mut values = physicsvalues::physics_values();
    let user_values_hashmap = user_values_to_hashmap(
        user_values_keys.split("\\n\\n").collect::<Vec<_>>(),
        user_values_values,
    );
    values.extend(user_values_hashmap);
    let parsed: fishrambeta::math::Equation =
        fishrambeta::parser::to_equation(equation.to_string(), &logger);
    let result = parsed.calculate(&values);
    return result;
}

fn user_values_to_hashmap(keys: Vec<&str>, values: &[f64]) -> HashMap<Variable, f64> {
    let mut values_hashmap = HashMap::new();
    for (key, value) in keys.iter().zip(values.iter()) {
        values_hashmap.insert(Variable::Letter(key.to_string()), *value);
    }
    return values_hashmap;
}
