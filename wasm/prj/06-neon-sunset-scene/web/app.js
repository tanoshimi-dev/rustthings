import init, {
  describe_theme,
  render_scene_svg
} from "./pkg/neon_sunset_scene.js";

const sceneFrame = document.getElementById("scene-frame");
const themeSelect = document.getElementById("theme");
const motionInput = document.getElementById("motion");
const motionValue = document.getElementById("motion-value");
const themeNote = document.getElementById("theme-note");
const status = document.getElementById("status");
const toggle = document.getElementById("toggle");

const WIDTH = 960;
const HEIGHT = 540;

let paused = false;
let animationTime = 0;
let previousFrame;

function renderFrame() {
  const theme = themeSelect.value;
  const motion = Number(motionInput.value);

  motionValue.textContent = motion.toFixed(2);
  themeNote.textContent = describe_theme(theme);
  sceneFrame.innerHTML = render_scene_svg(WIDTH, HEIGHT, animationTime, theme, motion);
  status.textContent = paused ? "Scene paused." : "Wasm painting live.";
}

function animate(now) {
  if (previousFrame === undefined) {
    previousFrame = now;
  }

  const delta = (now - previousFrame) / 1000;
  previousFrame = now;

  if (!paused) {
    animationTime += delta;
  }

  try {
    renderFrame();
  } catch (error) {
    status.textContent = `Rendering failed: ${error}`;
    return;
  }

  window.requestAnimationFrame(animate);
}

async function main() {
  try {
    await init();
    themeNote.textContent = describe_theme(themeSelect.value);
    motionValue.textContent = Number(motionInput.value).toFixed(2);

    themeSelect.addEventListener("input", renderFrame);
    motionInput.addEventListener("input", renderFrame);
    toggle.addEventListener("click", () => {
      paused = !paused;
      toggle.textContent = paused ? "Resume" : "Pause";
      renderFrame();
    });

    renderFrame();
    window.requestAnimationFrame(animate);
  } catch (error) {
    status.textContent = `Failed to load wasm: ${error}`;
  }
}

main();
