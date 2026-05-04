# Password strength meter

## Purpose

This sample demonstrates a signup-form helper that scores passwords locally and explains why a password is weak or strong.

## Project

- `wasm\prj\03-password-strength-meter`

## What the program does

- classifies character types
- scores password complexity
- penalizes very common weak patterns
- returns a label and feedback string

## Key Rust APIs

- `#[wasm_bindgen]`
- iterator-based character checks
- small rule-based scoring functions

## Run

```powershell
Set-Location .\wasm\prj\03-password-strength-meter
cargo run
```

## Build for WebAssembly

```powershell
cargo build --target wasm32-unknown-unknown
```
