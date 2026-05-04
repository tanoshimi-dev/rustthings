# Neon sunset scene

## Purpose

This sample demonstrates a more artistic use of Rust and WebAssembly: generating an animated browser scene instead of only business logic or data transforms.

## Project

- `wasm\prj\06-neon-sunset-scene`

## What the project does

- exports a Rust function that returns a full SVG scene for the current animation frame
- builds layered gradients, stars, mountains, reflections, and a retro grid
- lets the browser switch themes and motion intensity without extra frontend tooling
- keeps the art deterministic so native and wasm demos stay in sync

## Key Rust APIs

- `#[wasm_bindgen]`
- `Result<T, JsValue>` for browser-facing validation
- deterministic procedural generation with math functions and string output

## Run the native demo

```powershell
Set-Location .\wasm\prj\06-neon-sunset-scene
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
