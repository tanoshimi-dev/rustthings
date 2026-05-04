import init, {
  describe_theme,
  fragment_shader_source,
  scene_state,
  vertex_shader_source
} from "./pkg/glsl_orbit_scene.js";

const canvas = document.getElementById("gl-canvas");
const themeSelect = document.getElementById("theme");
const intensityInput = document.getElementById("intensity");
const intensityValue = document.getElementById("intensity-value");
const themeNote = document.getElementById("theme-note");
const status = document.getElementById("status");
const toggle = document.getElementById("toggle");

let paused = false;
let animationTime = 0;
let previousFrame;
let gl;
let program;
let uniforms;

function parseSceneState(payload) {
  return payload.split(";").filter(Boolean).reduce((result, entry) => {
    const [key, value] = entry.split("=");

    if (value.includes(",")) {
      result[key] = value.split(",").map(Number);
    } else {
      result[key] = Number(value);
    }

    return result;
  }, {});
}

function createShader(context, type, source) {
  const shader = context.createShader(type);
  context.shaderSource(shader, source);
  context.compileShader(shader);

  if (!context.getShaderParameter(shader, context.COMPILE_STATUS)) {
    const message = context.getShaderInfoLog(shader);
    context.deleteShader(shader);
    throw new Error(message || "Shader compilation failed");
  }

  return shader;
}

function createProgram(context, vertexSource, fragmentSource) {
  const vertexShader = createShader(context, context.VERTEX_SHADER, vertexSource);
  const fragmentShader = createShader(context, context.FRAGMENT_SHADER, fragmentSource);
  const shaderProgram = context.createProgram();

  context.attachShader(shaderProgram, vertexShader);
  context.attachShader(shaderProgram, fragmentShader);
  context.linkProgram(shaderProgram);

  if (!context.getProgramParameter(shaderProgram, context.LINK_STATUS)) {
    const message = context.getProgramInfoLog(shaderProgram);
    context.deleteProgram(shaderProgram);
    throw new Error(message || "Program linking failed");
  }

  context.deleteShader(vertexShader);
  context.deleteShader(fragmentShader);
  return shaderProgram;
}

function resizeCanvas() {
  const pixelRatio = window.devicePixelRatio || 1;
  const width = Math.max(1, Math.floor(canvas.clientWidth * pixelRatio));
  const height = Math.max(1, Math.floor(canvas.clientHeight * pixelRatio));

  if (canvas.width !== width || canvas.height !== height) {
    canvas.width = width;
    canvas.height = height;
  }

  gl.viewport(0, 0, canvas.width, canvas.height);
}

function renderFrame() {
  const theme = themeSelect.value;
  const intensity = Number(intensityInput.value);
  const state = parseSceneState(scene_state(animationTime, theme, intensity));

  intensityValue.textContent = intensity.toFixed(2);
  themeNote.textContent = describe_theme(theme);

  resizeCanvas();

  gl.useProgram(program);
  gl.uniform2f(uniforms.resolution, canvas.width, canvas.height);
  gl.uniform1f(uniforms.time, animationTime);
  gl.uniform3f(uniforms.cam, ...state.cam);
  gl.uniform3f(uniforms.accent, ...state.accent);
  gl.uniform3f(uniforms.shadow, ...state.shadow);
  gl.uniform3f(uniforms.fog, ...state.fog);
  gl.uniform1f(uniforms.glow, state.glow);
  gl.uniform1f(uniforms.pulse, state.pulse);
  gl.uniform1f(uniforms.spin, state.spin);
  gl.uniform1f(uniforms.tilt, state.tilt);
  gl.uniform1f(uniforms.grid, state.grid);
  gl.drawArrays(gl.TRIANGLE_STRIP, 0, 4);

  status.textContent = paused ? "Scene paused." : "GLSL scene rendering live.";
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

    gl = canvas.getContext("webgl2");
    if (!gl) {
      throw new Error("WebGL2 is required for this demo");
    }

    program = createProgram(gl, vertex_shader_source(), fragment_shader_source());
    uniforms = {
      resolution: gl.getUniformLocation(program, "u_resolution"),
      time: gl.getUniformLocation(program, "u_time"),
      cam: gl.getUniformLocation(program, "u_cam"),
      accent: gl.getUniformLocation(program, "u_accent"),
      shadow: gl.getUniformLocation(program, "u_shadow"),
      fog: gl.getUniformLocation(program, "u_fog"),
      glow: gl.getUniformLocation(program, "u_glow"),
      pulse: gl.getUniformLocation(program, "u_pulse"),
      spin: gl.getUniformLocation(program, "u_spin"),
      tilt: gl.getUniformLocation(program, "u_tilt"),
      grid: gl.getUniformLocation(program, "u_grid")
    };

    const quad = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, quad);
    gl.bufferData(
      gl.ARRAY_BUFFER,
      new Float32Array([-1, -1, 1, -1, -1, 1, 1, 1]),
      gl.STATIC_DRAW
    );

    const position = gl.getAttribLocation(program, "a_position");
    gl.enableVertexAttribArray(position);
    gl.vertexAttribPointer(position, 2, gl.FLOAT, false, 0, 0);

    themeNote.textContent = describe_theme(themeSelect.value);
    intensityValue.textContent = Number(intensityInput.value).toFixed(2);

    themeSelect.addEventListener("input", renderFrame);
    intensityInput.addEventListener("input", renderFrame);
    toggle.addEventListener("click", () => {
      paused = !paused;
      toggle.textContent = paused ? "Resume" : "Pause";
      renderFrame();
    });
    window.addEventListener("resize", renderFrame);

    renderFrame();
    window.requestAnimationFrame(animate);
  } catch (error) {
    status.textContent = `Failed to initialize demo: ${error}`;
  }
}

main();
