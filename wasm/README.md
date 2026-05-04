# Rust WebAssembly examples

A learning repository of small Rust projects for **browser-side business logic**, **client-side validation**, and **UI data preparation** that fit well with WebAssembly.

The examples are organized as standalone Cargo projects under `prj\`, with matching explanation docs under `docs\`.

## Repository structure

| Path | Purpose |
| --- | --- |
| `prj\` | Standalone Rust projects that can be tested locally and compiled to WebAssembly |
| `docs\` | Notes and per-usecase explanations |

## What you can learn here

- client-side text search helpers
- e-commerce pricing and checkout rules
- password-strength scoring in the browser
- chart and sparkline data preparation
- loading a wasm module from a simple web page
- procedural art scenes and animated browser visuals
- shader-driven 3D browser scenes with GLSL

## Sample projects

| Use case | Project | Explanation |
| --- | --- | --- |
| Search preview generation | `prj\01-search-preview` | `docs\search-preview\README.md` |
| Shopping cart pricing | `prj\02-shopping-cart-pricing` | `docs\shopping-cart-pricing\README.md` |
| Password strength meter | `prj\03-password-strength-meter` | `docs\password-strength-meter\README.md` |
| Sparkline data preparation | `prj\04-sparkline-data-prep` | `docs\sparkline-data-prep\README.md` |
| Simple web page integration | `prj\05-password-strength-web-page` | `docs\simple-web-page\README.md` |
| Artistic browser scene | `prj\06-neon-sunset-scene` | `docs\neon-sunset-scene\README.md` |
| GLSL 3D orbit scene | `prj\07-glsl-orbit-scene` | `docs\glsl-orbit-scene\README.md` |

## Getting started

1. Install Rust.
2. Optionally install the wasm target:

```powershell
rustup target add wasm32-unknown-unknown
```

3. Open one project directory under `prj\`.
4. Run the local demo:

```powershell
cargo run
```

5. Build a `.wasm` artifact:

```powershell
cargo build --target wasm32-unknown-unknown
```

Example:

```powershell
Set-Location .\prj\02-shopping-cart-pricing
cargo run
cargo build --target wasm32-unknown-unknown
```

## Recommended learning order

1. `01-search-preview`
2. `03-password-strength-meter`
3. `02-shopping-cart-pricing`
4. `04-sparkline-data-prep`
5. `05-password-strength-web-page`
6. `06-neon-sunset-scene`
7. `07-glsl-orbit-scene`

## Related docs

- `docs\wasm-usecases.md`
- `docs\sample-projects.md`
- `docs\tips\README.md`
