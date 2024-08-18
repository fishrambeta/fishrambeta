import init, {
  simplify,
  calculate,
  differentiate,
  integrate,
} from "./pkg/fishrambeta_wasm.js";
init().then(() => {
  window.calculate = calculate;
  window.differentiate = differentiate;
  window.simplify = simplify;
  window.integrate = integrate;
});

function on_input_changed() {
  process_operation();
}

function on_operation_changed() {
  process_operation();
}

function process_operation() {
  var input = input_mathfield.latex();
  var operation = document.getElementById("operation").value;
  result_mathfield.latex("");
  if (input == "") {
    return;
  }
  try {
    var result = "";

    switch (operation) {
      case "simplify":
        result = simplify(input);
        break;
      case "calculate":
        result = calculate(input);
        break;
      case "differentiate":
        result = differentiate(input);
        break;
      case "integrate":
        result = integrate(input);
        break;
    }

    result_mathfield.latex(result);
  } catch (error) {
    result_mathfield.latex("\\textbf{Invalid LaTeX (" + error + ")}");
  }
}

var MQ = MathQuill.getInterface(2);
var input_span = document.getElementById("latex-input");
var input_mathfield = MQ.MathField(input_span, {
  spaceBehavesLikeTab: true,
  handlers: {
    edit: on_input_changed,
  },
});

var result_span = document.getElementById("latex-result");
let result_mathfield = MQ.StaticMath(result_span);

document.addEventListener("DOMContentLoaded", function () {
  const selectElement = document.getElementById("operation");
  const targetElement = document.getElementById("input-values");

  // Function to check the selected value and show/hide the target element accordingly
  function toggleVisibility() {
    if (selectElement.value === "calculate") {
      targetElement.style.display = "block";
    } else {
      targetElement.style.display = "none";
    }
  }

  // Call the function on page load to set the initial state
  toggleVisibility();

  // Add an event listener to the select element
  selectElement.addEventListener("change", toggleVisibility);
});
