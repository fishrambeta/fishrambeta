use wasm_bindgen::prelude::*;
use fishrambeta;
use slog::{Logger,Discard,o};

#[wasm_bindgen]
pub fn simplify(equation: &str) -> String{
    let logger = Logger::root(Discard,o!());
    let parsed = fishrambeta::parser::to_equation(equation.to_string(), &logger);
    let parsed_back = fishrambeta::parser::to_latex(parsed, &logger);
    return "test".to_string();
    //return parsed_back;
}
