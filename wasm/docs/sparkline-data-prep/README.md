# Sparkline data preparation

## Purpose

This sample demonstrates how to turn raw numeric input into SVG-ready sparkline coordinates inside a browser application.

## Project

- `wasm\prj\04-sparkline-data-prep`

## What the program does

- parses comma-separated numeric values
- normalizes values into a drawing area
- produces a `points` string for SVG polylines
- can emit a complete inline SVG snippet

## Key Rust APIs

- `#[wasm_bindgen]`
- `Result<T, JsValue>` error propagation
- float normalization and formatting

## Run

```powershell
Set-Location .\wasm\prj\04-sparkline-data-prep
cargo run
```

## Build for WebAssembly

```powershell
cargo build --target wasm32-unknown-unknown
```
