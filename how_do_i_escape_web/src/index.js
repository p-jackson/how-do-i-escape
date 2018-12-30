// @ts-check

import "./style.css";

const form = document.querySelector("form");
if (!(form instanceof HTMLFormElement))
  throw new Error("expected form element");

const escapedOutputs = document.getElementById("escapedOutputContainer");
if (!escapedOutputs) throw new Error("expected output container element");

const input = document.getElementById("grapheme-input");
if (!(input instanceof HTMLInputElement))
  throw new Error("expected input element");

const cssOutput = document.getElementById("css-output");
if (!cssOutput) throw new Error("expected css output element");

const htmlOutput = document.getElementById("html-output");
if (!htmlOutput) throw new Error("expected html output element");

const jsOutput = document.getElementById("js-output");
if (!jsOutput) throw new Error("expected js output element");

const hintButtons = Array.from(document.querySelectorAll(".hints-button"));

import("../crate/pkg").then(crate => {
  crate.init();

  const updateOutputs = () => {
    if (input.value) {
      cssOutput.textContent = crate.escape_css(input.value);
      htmlOutput.textContent = crate.escape_html(input.value);
      jsOutput.textContent = crate.escape_js(input.value);
    } else {
      // Insert zero-width spaces so nothing appears in box
      cssOutput.textContent = "\u200b";
      htmlOutput.textContent = "\u200b";
      jsOutput.textContent = "\u200b";
    }
  };

  input.addEventListener("input", updateOutputs);

  form.addEventListener("submit", e => {
    e.preventDefault();
    updateOutputs();
  });

  hintButtons.forEach(button => {
    button.addEventListener("click", () => {
      input.value = button.textContent;
      updateOutputs();
    });
  });

  updateOutputs();
});
