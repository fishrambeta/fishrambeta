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

window.on_operation_changed = function () {
  process_operation();
};

function get_values() {
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
      value_latex != null
    ) {
      var value = calculate(value_latex, "", []);
      keys.push(key_latex);
      values.push(value);
    }
  }
  return { keys: keys, values: values };
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
        var values = get_values();
        console.log(values);
        result = calculate(input, values.keys.join("\\n\\n"), values.values);
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

for (let i = 0; i < 10; i++) {
  add_new_value_field(i);
}
