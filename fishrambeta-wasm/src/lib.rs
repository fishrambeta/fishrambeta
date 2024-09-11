use fishrambeta::{
    self,
    math::{steps::StepLogger, Equation, Variable},
    physicsvalues,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Result {
    latex: String,
    steps: Vec<String>,
}

#[wasm_bindgen]
pub fn latex_to_numpy(equation: &str) -> String {
    let parsed = Equation::from_latex(equation, false);
    parsed.to_numpy()
}

#[wasm_bindgen]
pub fn simplify(equation: &str, implicit_multiplication: bool) -> JsValue {
    let parsed = Equation::from_latex(equation, implicit_multiplication);
    let mut step_logger = Some(StepLogger::new());
    let simplified = parsed.simplify_until_complete(&mut step_logger);

    serde_wasm_bindgen::to_value(&Result {
        latex: simplified.to_latex(),
        steps: step_logger.unwrap().get_steps_as_strings(),
    })
    .unwrap()
}

#[wasm_bindgen]
pub fn differentiate(
    equation: &str,
    differentiate_to: &str,
    implicit_multiplication: bool,
) -> JsValue {
    let parsed = Equation::from_latex(equation, implicit_multiplication);
    let mut step_logger = Some(StepLogger::new());
    let differentiated = parsed
        .differentiate(
            &Variable::Letter(differentiate_to.to_string()),
            &mut step_logger,
        )
        .simplify_until_complete(&mut step_logger);
    serde_wasm_bindgen::to_value(&Result {
        latex: differentiated.to_latex(),
        steps: step_logger.unwrap().get_steps_as_strings(),
    })
    .unwrap()
}

#[wasm_bindgen]
pub fn integrate(equation: &str, integrate_to: &str, implicit_multiplication: bool) -> JsValue {
    let parsed = Equation::from_latex(equation, implicit_multiplication);
    let mut step_logger = Some(StepLogger::new());
    let integrated = parsed
        .integrate(
            &Variable::Letter(integrate_to.to_string()),
            &mut step_logger,
        )
        .simplify_until_complete(&mut step_logger);

    serde_wasm_bindgen::to_value(&Result {
        latex: integrated.to_latex(),
        steps: step_logger.unwrap().get_steps_as_strings(),
    })
    .unwrap()
}

#[wasm_bindgen]
pub fn calculate(
    equation: &str,
    user_values_keys: &str,
    user_values_values: &[f64],
    implicit_multiplication: bool,
) -> JsValue {
    let mut values = physicsvalues::physics_values();
    let user_values_hashmap = user_values_to_hashmap(
        user_values_keys.split("\\n\\n").collect::<Vec<_>>(),
        user_values_values,
    );
    values.extend(user_values_hashmap);
    let parsed: Equation = Equation::from_latex(equation, implicit_multiplication);
    let result = parsed.calculate(&values);
    serde_wasm_bindgen::to_value(&Result {
        latex: result.to_string(),
        steps: vec![
            "\\textbf{I hope you know how to fill in variables in an equation...}".to_string(),
        ],
    })
    .unwrap()
}

#[wasm_bindgen]
pub fn taylor_expansion(
    equation: &str,
    variable: &str,
    around: &str,
    degree: i32,
    implicit_multiplication: bool,
) -> JsValue {
    let parsed = Equation::from_latex(equation, implicit_multiplication);
    let around = Equation::from_latex(around, implicit_multiplication);
    let mut step_logger = Some(StepLogger::new());
    let taylor_expansion = parsed
        .taylor_expansion(
            Variable::Letter(variable.to_string()),
            around,
            degree.try_into().unwrap(),
            &mut step_logger,
        )
        .into_equation()
        .simplify_until_complete(&mut step_logger);
    serde_wasm_bindgen::to_value(&Result {
        latex: taylor_expansion.to_latex(),
        steps: step_logger.unwrap().get_steps_as_strings(),
    })
    .unwrap()
}

#[wasm_bindgen]
pub fn error_analysis(
    equation: &str,
    error_variables: &str,
    implicit_multiplication: bool,
) -> JsValue {
    let parsed: Equation = Equation::from_latex(equation, implicit_multiplication);
    let mut step_logger = Some(StepLogger::new());
    let error_variables: Vec<_> = error_variables
        .split("\\n\\n")
        .map(|variable| Variable::Letter(variable.to_string()))
        .collect();
    let errors = parsed
        .error_analysis(error_variables, &mut step_logger)
        .simplify_until_complete(&mut step_logger);
    serde_wasm_bindgen::to_value(&Result {
        latex: errors.to_latex(),
        steps: step_logger.unwrap().get_steps_as_strings(),
    })
    .unwrap()
}

fn user_values_to_hashmap(keys: Vec<&str>, values: &[f64]) -> HashMap<Variable, f64> {
    let mut values_hashmap = HashMap::new();
    for (key, value) in keys.iter().zip(values.iter()) {
        values_hashmap.insert(Variable::Letter(key.to_string()), *value);
    }
    values_hashmap
}
