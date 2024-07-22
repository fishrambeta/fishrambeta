use crate::math::{Equation, Variable};
use std::collections::BTreeMap;

#[rustfmt::skip]
const RANDOM_VALUES: [f64; 100] = [217.77919232197257, -35.022747163580675, -283.61757906755554, -422.8332777676821, -194.92360881854609, -477.86655577996, -29.012950819869673, -138.99250282163388, 217.37245037627065, 81.12293398936777, -484.6028233519494, -0.06141649619939926, -408.66081587017345, 454.5372144138511, 372.28974818102233, -339.08510157330574, 25.845056907138996, -6.623750578823376, 487.08176906403116, -235.16112120581255, -64.7076534048785, 379.04666015789155, -136.0099803454238, -270.1014077737798, -261.5338088646316, -299.83008733261875, 313.9502700105247, -436.3768060008657, 99.7130184799197, 253.665853120292, -485.56748448173124, -344.44107825401136, -305.60402556424737, -391.05762733119997, 259.79906875779363, -8.764033182361857, 401.89443171718904, 2.7298738221693952, -357.96757518076316, 30.066251011020086, 242.56134250311607, -188.3264692428852, -323.53321697284275, 46.36239256439262, 431.7482207979351, -32.9332217853173, -450.43980354035773, -313.44881729890176, -267.20467304181113, 43.682177656362, 250.4869482395178, -12.887344437762636, -9.100668379016554, 10.393089747143677, -31.534588565308354, 143.96249940655662, -110.11227778129404, -439.896803085285, 173.6373649619834, 176.99133234452813, -375.8187008434254, 365.2037962273764, -241.68433524572805, -464.88278829565945, -474.9472097056635, 214.6598812841528, 48.32911952122981, 199.99881004343263, -412.47194657311326, -386.77457496996414, -223.87912390602935, 115.87691202707208, -328.9029744692947, -376.33676502167845, -312.6983421683308, 302.39542101021084, -107.12349408768807, -444.4281036745549, -362.7697478836581, 180.4954544989173, -171.39029182793684, -73.87299139904479, 62.979932170930624, 289.7646491023522, 364.4356845596167, 160.76244782253548, -447.16766726529863, 161.07671711644014, 416.80878009610444, -73.91271610683026, 333.94896017534666, 228.33890219136163, 376.7379356120366, -390.77198880083984, 467.366395341391, 330.41419879022885, 161.37986540748682, -288.80853058314636, -152.20836571291431, -182.71524412760652];

#[test]
fn simplify() {
    let valuedicts = valuedicts();

    assert!(simplified_is_equal("x^2", &valuedicts));
    assert!(simplified_is_equal("\\frac{x}{x}", &valuedicts));
    assert!(simplified_is_equal("\\frac{\\frac{-x}{x}}{x}", &valuedicts));
    assert!(simplified_is_equal(
        "\\frac{1}{\\left(x+4\\right)^2}",
        &valuedicts
    ));
}

#[test]
fn differentiate() {
    let valuedicts = valuedicts();

    assert!(derivative_is_equal(
        "\\tan(x)",
        "\\frac{1}{\\cos(x)^2}",
        &valuedicts
    ));
    assert!(derivative_is_equal(
        "\\frac{\\tan(x)}{x^2}",
        "\\frac{x*\\frac{1}{\\cos(x)^2}-2*\\tan(x)}{x^3}",
        &valuedicts
    ));
}

fn valuedicts() -> [BTreeMap<Variable, f64>; 30] {
    let mut array: [BTreeMap<Variable, f64>; 30] = Default::default();
    for i in 0..30 {
        let mut valuedict = BTreeMap::new();
        for j in 0..2 {
            match j {
                0 => valuedict.insert(Variable::Letter("x".to_string()), RANDOM_VALUES[i * 3 + j]),
                1 => valuedict.insert(Variable::Letter("x".to_string()), RANDOM_VALUES[i * 3 + j]),
                2 => valuedict.insert(Variable::Letter("x".to_string()), RANDOM_VALUES[i * 3 + j]),
                _ => unreachable!(),
            };
        }
        array[i] = valuedict;
    }
    array
}

fn simplified_is_equal(equation: &str, valuedicts: &[BTreeMap<Variable, f64>]) -> bool {
    let parsed = Equation::from_latex(equation);
    let simplified = parsed.clone().simplify_until_complete();

    valuedicts
        .iter()
        .all(|values| approx_equal(parsed.calculate(values), simplified.calculate(values)))
}

fn derivative_is_equal(
    equation: &str,
    expected_result: &str,
    valuedicts: &[BTreeMap<Variable, f64>],
) -> bool {
    let parsed = Equation::from_latex(equation);
    let correct = Equation::from_latex(expected_result);
    let derivative = parsed
        .differentiate(&Variable::Letter("x".to_string()))
        .simplify_until_complete();
    valuedicts
        .iter()
        .all(|values| approx_equal(derivative.calculate(values), correct.calculate(values)))
}

fn approx_equal(a: f64, b: f64) -> bool {
    let p = a / 10000.;
    (a - b).abs() < p.abs()
}
