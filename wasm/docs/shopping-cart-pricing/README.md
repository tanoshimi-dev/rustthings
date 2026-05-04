# Shopping cart pricing

## Purpose

This sample demonstrates checkout-style business logic that often needs to run instantly in the browser: discount, shipping, and tax calculation.

## Project

- `wasm\prj\02-shopping-cart-pricing`

## What the program does

- applies coupon rules
- calculates region-based shipping
- calculates tax from the discounted amount
- returns a formatted order summary string

## Key Rust APIs

- `#[wasm_bindgen]`
- integer-safe money calculations
- `Result<T, JsValue>` for surfaced errors

## Run

```powershell
Set-Location .\wasm\prj\02-shopping-cart-pricing
cargo run
```

## Build for WebAssembly

```powershell
cargo build --target wasm32-unknown-unknown
```
