# Simple web page integration

## Purpose

This sample demonstrates the smallest practical browser setup in this workspace: a Rust wasm library loaded from a plain HTML page with a little JavaScript.

## Project

- `wasm\prj\05-password-strength-web-page`

## What the project does

- reuses the password strength logic from `03-password-strength-meter`
- exports browser-friendly wrapper functions from a wasm crate
- loads the generated wasm module from a plain web page
- updates the page live as the user types

## Project layout

- `src\lib.rs` - wasm exports
- `src\main.rs` - small native demo
- `web\index.html` - simple browser page
- `web\app.js` - JavaScript that loads wasm and wires the UI
- `web\styles.css` - lightweight styling

## Run the native demo

```powershell
Set-Location .\wasm\prj\05-password-strength-web-page
cargo run
```

## Build for the browser

Install the tools once:

```powershell
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
```

Build the web package:

```powershell
wasm-pack build --target web --out-dir web/pkg
```

## Open the page

From the project directory:

```powershell
cd .\web
python -m http.server 8080
```

Then open:

```text
http://localhost:8080
```
