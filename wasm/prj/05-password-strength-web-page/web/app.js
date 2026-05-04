import init, {
  password_feedback,
  score_password,
  strength_label
} from "./pkg/password_strength_web_page.js";

const input = document.getElementById("password");
const status = document.getElementById("status");
const score = document.getElementById("score");
const label = document.getElementById("label");
const feedback = document.getElementById("feedback");

function render() {
  const value = input.value;
  score.textContent = String(score_password(value));
  label.textContent = strength_label(value);
  feedback.textContent = password_feedback(value);
}

async function main() {
  try {
    await init();
    status.textContent = "Wasm loaded. Start typing.";
    input.value = "WasmReady#2026";
    input.addEventListener("input", render);
    render();
  } catch (error) {
    status.textContent = `Failed to load wasm: ${error}`;
  }
}

main();
