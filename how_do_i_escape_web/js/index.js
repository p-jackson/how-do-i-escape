// @ts-check

const form = document.querySelector("form");
if (!(form instanceof HTMLFormElement))
  throw new Error("expected form element");

const input = document.getElementById("grapheme-input");
if (!(input instanceof HTMLInputElement))
  throw new Error("expected input element");

const cssOutput = document.getElementById("css-output");
if (!cssOutput) throw new Error("expected css output element");

const htmlOutput = document.getElementById("html-output");
if (!htmlOutput) throw new Error("expected html output element");

const jsOutput = document.getElementById("js-output");
if (!jsOutput) throw new Error("expected js output element");

import("../crate/pkg").then(crate => {
  crate.init();

  const updateOutputs = () => {
    cssOutput.textContent = crate.escape_css(input.value);
    htmlOutput.textContent = crate.escape_html(input.value);
    jsOutput.textContent = crate.escape_js(input.value);
  };

  input.addEventListener("input", updateOutputs);

  form.addEventListener("submit", e => {
    e.preventDefault();
    updateOutputs();
  });

  updateOutputs();
});
