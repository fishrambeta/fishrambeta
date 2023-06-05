use fishrambeta::{self, math::Symbol, physicsvalues};
use slog::{o, Discard, Logger};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn simplify(equation: &str) -> String {
    let logger = Logger::root(Discard, o!());
    let parsed = fishrambeta::parser::to_equation(equation.to_string(), &logger, true);
    let simplified = parsed.simplify().simplify();
    let parsed_back = fishrambeta::parser::to_latex(simplified, &logger, true);
    return parsed_back;
}

#[wasm_bindgen]
pub fn calculate(equation: &str) -> f64 {
    let logger = Logger::root(Discard, o!());
    let values = physicsvalues::physics_values();
    let parsed: fishrambeta::math::Equation =
        fishrambeta::parser::to_equation(equation.to_string(), &logger, true);
    let result = parsed.calculate(&values);
    return result;
}
