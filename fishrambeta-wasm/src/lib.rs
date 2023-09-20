use fishrambeta::{self, math::Variable, physicsvalues};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn simplify(equation: &str) -> String {
    let parsed = fishrambeta::parser::IR::latex_to_equation(
        equation.to_string().chars().collect::<Vec<_>>(),
        true,
    );
    let simplified = parsed
        .simplify()
        .simplify()
        .simplify()
        .simplify()
        .simplify()
        .simplify()
        .simplify()
        .simplify();
    let parsed_back = fishrambeta::parser::IR::equation_to_latex(simplified, true);
    return parsed_back;
}

#[wasm_bindgen]
pub fn differentiate(equation: &str) -> String {
    let parsed = fishrambeta::parser::IR::latex_to_equation(
        equation.to_string().chars().collect::<Vec<_>>(),
        true,
    );
    let differentiated = parsed
        .simplify()
        .differentiate(&Variable::Letter("x".to_string()))
        .simplify()
        .simplify()
        .simplify()
        .simplify()
        .simplify()
        .simplify()
        .simplify();
    let parsed_back = fishrambeta::parser::IR::equation_to_latex(differentiated, true);
    return parsed_back;
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
    let equationstring = equation.to_string().chars().collect::<Vec<_>>();
    let parsed: fishrambeta::math::Equation =
        fishrambeta::parser::IR::latex_to_equation(equationstring, true);
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
