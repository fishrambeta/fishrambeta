import init, {
  simplify,
  latex_to_numpy,
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

window.on_operation_changed = function () {
  process_operation();
};

function get_values(implicit_multiplication) {
  var keys = [];
  var values = [];
  console.log(value_fields);
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
      var value = calculate(value_latex, "", [], implicit_multiplication);
      keys.push(key_latex);
      values.push(value);
    }
  }
  return { keys: keys, values: values };
}

function process_operation() {
  var input = input_mathfield.latex();
  var operation = document.getElementById("operation").value;
  var implicit_multiplication = document.getElementById(
    "implicit-multiplication-checkbox",
  ).checked;
  result_mathfield.latex("");
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
    }
    result_mathfield.latex(result);
    result_latex_copypaste.value = result;
    var numpy = latex_to_numpy(String(result));
    result_numpy_copypaste.value = numpy;
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

var value_fields = [];

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

var result_span = document.getElementById("latex-result");
let result_mathfield = MQ.StaticMath(result_span);

var result_latex_copypaste = document.getElementById("latex-result-copypaste");
var result_numpy_copypaste = document.getElementById("numpy-result-copypaste");

document.addEventListener("DOMContentLoaded", function () {
  const operation_element = document.getElementById("operation");
  const input_values_element = document.getElementById("input-values");
  const differentiate_options = document.getElementById(
    "differentiate-options",
  );
  const integrate_options = document.getElementById("integrate-options");

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
