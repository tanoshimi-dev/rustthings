# Search preview generation

## Purpose

This sample demonstrates a realistic browser search helper: counting matches and building a small preview snippet around the first hit.

## Project

- `wasm\prj\01-search-preview`

## What the program does

- scans article text for an ASCII-insensitive query
- counts all matches
- extracts a short preview window around the first match
- exports the same logic through `wasm-bindgen`

## Key Rust APIs

- `#[wasm_bindgen]`
- string slicing with UTF-8 boundary checks
- `eq_ignore_ascii_case`

## Run

```powershell
Set-Location .\wasm\prj\01-search-preview
cargo run
```

## Build for WebAssembly

```powershell
cargo build --target wasm32-unknown-unknown
```
