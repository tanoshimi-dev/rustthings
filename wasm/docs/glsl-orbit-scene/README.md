# GLSL orbit scene

## Purpose

This sample demonstrates a more advanced browser graphics workflow: Rust wasm feeding values into a WebGL2 + GLSL 3D scene.

## Project

- `wasm\prj\07-glsl-orbit-scene`

## What the project does

- exports GLSL shader source from Rust so the demo stays packaged as one wasm-focused sample
- generates deterministic per-frame scene values such as camera orbit, glow, and palette controls
- renders a fullscreen raymarched 3D scene with a ringed planet, columns, and a neon floor
- lets the browser switch themes and intensity without adding extra frontend tooling

## Key Rust APIs

- `#[wasm_bindgen]`
- `Result<T, JsValue>` for browser-facing validation
- deterministic numeric presets passed from Rust to JavaScript

## Run the native demo

```powershell
Set-Location .\wasm\prj\07-glsl-orbit-scene
cargo run
```

## Build for the browser

```powershell
wasm-pack build --target web --out-dir web/pkg
```

## Open the page

```powershell
cd .\web
python -m http.server 8080
```

Then open:

```text
http://localhost:8080
```
