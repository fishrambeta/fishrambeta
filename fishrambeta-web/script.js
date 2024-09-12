import init, {
  simplify,
  calculate,
  differentiate,
  integrate,
  taylor_expansion,
  error_analysis,
} from "./pkg/fishrambeta_wasm.js";
init().then(() => {
  window.calculate = calculate;
  window.differentiate = differentiate;
  window.simplify = simplify;
  window.integrate = integrate;
  window.taylor_expansion = taylor_expansion;
  window.error_analysis = error_analysis;
});

function on_input_changed() {
  process_operation();
}

window.on_operation_changed = function () {
  process_operation();
};

function get_values(implicit_multiplication) {
  var keys = [];
  var values = [];
  for (var i = 0; i < value_fields.length; i++) {
    var key_latex = value_fields[i].key.latex();
    var value_latex = value_fields[i].value.latex();
    if (
      key_latex != "" &&
      value_latex != "" &&
      key_latex != null &&
      value_latex != null &&
      key_latex !== undefined
    ) {
      var value = calculate(value_latex, "", [], implicit_multiplication).latex;
      keys.push(key_latex);
      values.push(value);
    }
  }
  return { keys: keys, values: values };
}

function get_error_variables() {
  var variables = [];
  for (var i = 0; i < value_fields.length; i++) {
    var variable_latex = error_variable_fields[i].latex();
    if (
      variable_latex != "" &&
      variable_latex != null &&
      variable_latex !== undefined
    ) {
      variables.push(variable_latex);
    }
  }
  return variables;
}

function process_operation() {
  var input = input_mathfield.latex();
  var operation = document.getElementById("operation").value;
  var implicit_multiplication = document.getElementById(
    "implicit-multiplication-checkbox",
  ).checked;
  var scientific_notation = document.getElementById(
    "scientific-notation-checkbox",
  ).checked;
  result_mathfield.latex("");
  steps_parent.innerHTML = "";

  result_latex_copypaste.value = "";
  result_numpy_copypaste.value = "";
  if (input == "") {
    return;
  }
  try {
    var result = "";

    switch (operation) {
      case "simplify":
        result = simplify(input, implicit_multiplication);
        break;
      case "calculate":
        var values = get_values(implicit_multiplication);
        console.log("values:");
        console.log(values);
        result = calculate(
          input,
          values.keys.join("\\n\\n"),
          values.values,
          implicit_multiplication,
        );
        break;
      case "differentiate":
        var differentiate_to = differentiate_to_mathfield.latex();
        if (differentiate_to == "") {
          throw new Error("Cannot differentiate to empty string");
        }
        result = differentiate(
          input,
          differentiate_to,
          implicit_multiplication,
        );
        break;
      case "integrate":
        var integrate_to = integrate_to_mathfield.latex();
        if (integrate_to == "") {
          throw new Error("Cannot integrate to empty string");
        }
        result = integrate(input, integrate_to, implicit_multiplication);
        break;
      case "taylor-expansion":
        var taylor_expansion_to = taylor_expansion_to_mathfield.latex();
        var taylor_expansion_around = taylor_expansion_around_mathfield.latex();
        var taylor_expansion_degree = Number(
          taylor_expansion_degree_mathfield.latex(),
        );
        if (taylor_expansion_to == "") {
          throw new Error("Cannot taylor expand to empty string");
        }
        result = taylor_expansion(
          input,
          taylor_expansion_to,
          taylor_expansion_around,
          taylor_expansion_degree,
          implicit_multiplication,
        );
        break;
      case "error-analysis":
        var error_variables = get_error_variables();
        if (error_variables.length == 0) {
          throw new Error(
            "Cannot do error analysis without specifying variables",
          );
        }
        result = error_analysis(
          input,
          error_variables.join("\\n\\n"),
          implicit_multiplication,
        );
        break;
    }
    if (operation == "calculate" && scientific_notation) {
      var result_float = parseFloat(result.latex);
      var sign = Math.sign(result_float);
      result_float = Math.abs(result_float);
      var log = Math.log10(result_float);
      var characteristic = Math.floor(log);
      var mantissa = log - characteristic;
      var mantissa_exponentiated = Math.pow(10, mantissa);
      switch (sign) {
        case -1:
          result.latex =
            "-" +
            mantissa_exponentiated +
            " \\cdot 10^{" +
            characteristic +
            "}";
          break;
        case 1:
          result.latex =
            mantissa_exponentiated + " \\cdot 10^{" + characteristic + "}";
          break;
        case 0:
          result.latex = "0";
      }
    }
    result_mathfield.latex(result.latex);
    result_latex_copypaste.value = result.latex;
    var i = 0;
    for (var step of result.steps) {
      steps_parent.insertAdjacentHTML(
        "beforeend",
        `<p><span id="step-latex-${i}"></span></p>`,
      );
      var step_span = document.getElementById(`step-latex-${i}`);
      var step_mathfield = MQ.StaticMath(step_span);
      step_mathfield.latex(step);
      i += 1;
    }
    result_numpy_copypaste.value = result.numpy;
  } catch (error) {
    console.log(error);
    result_mathfield.latex("\\textbf{Invalid LaTeX (" + error + ")}");
  }
}

function add_new_value_field(id) {
  var container = document.getElementById("input-values-container");
  var key_id = `input-value-key-${id}`;
  var value_id = `input-value-value-${id}`;
  container.insertAdjacentHTML(
    "afterbegin",
    `<p><span class="latex-key" id="${key_id}"></span>=<span class="latex-value" id="${value_id}"></span></p>`,
  );
  var key_span = document.getElementById(key_id);
  var key_mathfield = MQ.MathField(key_span, {
    spaceBehavesLikeTab: true,
    handlers: {
      edit: function () {
        try {
          process_operation();
        } catch {}
      },
    },
  });
  var value_span = document.getElementById(value_id);
  var value_mathfield = MQ.MathField(value_span, {
    spaceBehavesLikeTab: true,
    handlers: {
      edit: function () {
        try {
          process_operation();
        } catch {}
      },
    },
  });
  value_fields[id] = { key: key_mathfield, value: value_mathfield };
}

function add_error_variable_field(id) {
  var container = document.getElementById("error-variables-container");
  var key_id = `error-variable-${id}`;
  container.insertAdjacentHTML(
    "afterbegin",
    `<p><span class="latex-key" id="${key_id}"></span></p>`,
  );
  var error_variable_span = document.getElementById(key_id);
  var error_variable_mathfield = MQ.MathField(error_variable_span, {
    spaceBehavesLikeTab: true,
    handlers: {
      edit: function () {
        try {
          process_operation();
        } catch {}
      },
    },
  });
  error_variable_fields[id] = error_variable_mathfield;
}

var value_fields = [];
var error_variable_fields = [];

var MQ = MathQuill.getInterface(2);
var input_span = document.getElementById("latex-input");
var input_mathfield = MQ.MathField(input_span, {
  spaceBehavesLikeTab: true,
  handlers: {
    edit: on_input_changed,
  },
});

var differentiate_to_span = document.getElementById("differentiate-to");
var differentiate_to_mathfield = MQ.MathField(differentiate_to_span, {
  spaceBehavesLikeTab: true,
  handlers: {
    edit: on_input_changed,
  },
});

var integrate_to_span = document.getElementById("integrate-to");
var integrate_to_mathfield = MQ.MathField(integrate_to_span, {
  spaceBehavesLikeTab: true,
  handlers: {
    edit: on_input_changed,
  },
});

var taylor_expansion_to_span = document.getElementById("taylor-expansion-to");
var taylor_expansion_to_mathfield = MQ.MathField(taylor_expansion_to_span, {
  spaceBehavesLikeTab: true,
  handlers: {
    edit: on_input_changed,
  },
});
var taylor_expansion_around_span = document.getElementById(
  "taylor-expansion-around",
);
var taylor_expansion_around_mathfield = MQ.MathField(
  taylor_expansion_around_span,
  {
    spaceBehavesLikeTab: true,
    handlers: {
      edit: on_input_changed,
    },
  },
);
var taylor_expansion_degree_span = document.getElementById(
  "taylor-expansion-degree",
);
var taylor_expansion_degree_mathfield = MQ.MathField(
  taylor_expansion_degree_span,
  {
    spaceBehavesLikeTab: true,
    handlers: {
      edit: on_input_changed,
    },
  },
);

var result_span = document.getElementById("latex-result");
let result_mathfield = MQ.StaticMath(result_span);

var steps_parent = document.getElementById("steps-parent");

var result_latex_copypaste = document.getElementById("latex-result-copypaste");
var result_numpy_copypaste = document.getElementById("numpy-result-copypaste");

document.addEventListener("DOMContentLoaded", function () {
  const operation_element = document.getElementById("operation");
  const input_values_element = document.getElementById("input-values");
  const differentiate_options = document.getElementById(
    "differentiate-options",
  );
  const integrate_options = document.getElementById("integrate-options");
  const taylor_expansion_options = document.getElementById(
    "taylor-expansion-options",
  );
  const error_analysis_options = document.getElementById(
    "error-analysis-options",
  );

  function toggle_visibilities() {
    if (operation_element.value === "calculate") {
      input_values_element.style.display = "block";
    } else {
      input_values_element.style.display = "none";
    }

    if (operation_element.value === "differentiate") {
      differentiate_options.style.display = "block";
    } else {
      differentiate_options.style.display = "none";
    }

    if (operation_element.value === "integrate") {
      integrate_options.style.display = "block";
    } else {
      integrate_options.style.display = "none";
    }

    if (operation_element.value === "taylor-expansion") {
      taylor_expansion_options.style.display = "block";
    } else {
      taylor_expansion_options.style.display = "none";
    }

    if (operation_element.value === "error-analysis") {
      error_analysis_options.style.display = "block";
    } else {
      error_analysis_options.style.display = "none";
    }
  }

  toggle_visibilities();

  // Add an event listener to the select element
  operation_element.addEventListener("change", toggle_visibilities);
});

function on_update_latex_checkbox() {
  var checkbox = document.getElementById("show-latex-checkbox");
  var element = document.getElementById("latex-result-p");

  if (checkbox.checked) {
    element.style.display = "block";
  } else {
    element.style.display = "none";
  }
}
document
  .getElementById("show-latex-checkbox")
  .addEventListener("change", on_update_latex_checkbox);

on_update_latex_checkbox();
function on_update_numpy_checkbox() {
  var checkbox = document.getElementById("show-numpy-checkbox");
  var element = document.getElementById("numpy-result-p");

  if (checkbox.checked) {
    element.style.display = "block";
  } else {
    element.style.display = "none";
  }
}
document
  .getElementById("show-numpy-checkbox")
  .addEventListener("change", on_update_numpy_checkbox);
on_update_numpy_checkbox();

for (let i = 0; i < 10; i++) {
  add_new_value_field(i);
}

for (let i = 0; i < 10; i++) {
  add_error_variable_field(i);
}
