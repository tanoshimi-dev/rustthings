# WebAssembly tips for Rust beginners

This note is for getting comfortable with **what WebAssembly is**, **why Rust is a good fit**, and **how to try it from a simple web page**.

## What is WebAssembly?

WebAssembly, usually called **wasm**, is a compact binary format that runs in the browser.

You can think of it like this:

- JavaScript is the main language the browser understands directly
- wasm is a fast, portable module format the browser can also run
- JavaScript usually loads the wasm file and calls exported functions from it

Wasm is **not a replacement for HTML, CSS, and JavaScript**. It works best as a helper for logic that benefits from:

- speed
- predictable behavior
- code reuse from Rust or other compiled languages

## Why use wasm with Rust?

Rust fits wasm well because it gives you:

- strong type safety
- good performance
- easy reuse of non-UI business logic
- a nice split between browser UI code and Rust logic

In practice, a common pattern is:

1. Keep the UI in HTML/CSS/JavaScript.
2. Move heavy or reusable logic into Rust.
3. Compile the Rust library to wasm.
4. Call the exported Rust functions from JavaScript.

## Good beginner use cases

Start with logic that takes input and returns output:

- password strength scoring
- shopping cart price calculation
- search snippet generation
- chart point generation

These are easier than starting with direct DOM manipulation.

## When wasm is a good choice

Wasm is a good fit when:

- you want the same logic to run consistently in the browser
- the code is calculation-heavy
- you already have Rust domain logic
- you want to reduce backend round-trips for local calculations

## When wasm is not the best choice

You may not need wasm when:

- the task is simple UI glue
- plain JavaScript is shorter and clearer
- you mostly need direct DOM updates with little computation

For many apps, the best setup is:

- JavaScript for UI wiring
- Rust wasm for the important logic

## What files are usually involved?

A small Rust-to-browser setup usually has:

| File | Purpose |
| --- | --- |
| `src/lib.rs` | Rust functions exported to JavaScript |
| `Cargo.toml` | Rust crate config |
| `.wasm` | compiled WebAssembly binary |
| generated `.js` glue | loads wasm and exposes functions |
| `index.html` | browser page |

## Can you try wasm from a simple web page?

**Yes.** That is one of the easiest ways to learn it.

The simplest beginner flow is:

1. write a tiny Rust library
2. export functions with `wasm-bindgen`
3. build for the browser
4. load it from a small HTML page with `<script type="module">`

## Recommended first setup

Install the wasm target:

```powershell
rustup target add wasm32-unknown-unknown
```

Install `wasm-pack`:

```powershell
cargo install wasm-pack
```

`wasm-pack` is helpful for beginners because it generates the JavaScript loader for you.

## Minimal Rust example

`src/lib.rs`

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

`Cargo.toml`

```toml
[package]
name = "hello_wasm"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
wasm-bindgen = "0.2"
```

Build it for a web page:

```powershell
wasm-pack build --target web
```

That creates a `pkg\` folder with the wasm file and JavaScript glue code.

## Minimal HTML page

`index.html`

```html
<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <title>Hello wasm</title>
  </head>
  <body>
    <h1>Hello wasm</h1>
    <p id="result">loading...</p>

    <script type="module">
      import init, { add } from "./pkg/hello_wasm.js";

      await init();

      const result = add(20, 22);
      document.getElementById("result").textContent = `20 + 22 = ${result}`;
    </script>
  </body>
</html>
```

## How to open the page

Use a small local web server instead of double-clicking the file.

Example with Python:

```powershell
python -m http.server 8080
```

Then open:

```text
http://localhost:8080
```

## How this connects to the projects in this workspace

The projects under `wasm\prj\` already expose Rust functions with `wasm-bindgen`, so they are close to browser use already.

A good first browser experiment in this workspace is:

- `wasm\prj\03-password-strength-meter`

That project is a good fit for a simple page because:

- it takes a string input
- it returns plain values
- it matches a real browser form use case

## Example: try the password meter from a page

From `wasm\prj\03-password-strength-meter`:

```powershell
wasm-pack build --target web
```

Then create a small page that imports:

- `score_password`
- `strength_label`
- `password_feedback`

Example:

```html
<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <title>Password meter</title>
  </head>
  <body>
    <input id="password" type="password" placeholder="Type a password" />
    <p id="score"></p>
    <p id="label"></p>
    <p id="feedback"></p>

    <script type="module">
      import init, {
        score_password,
        strength_label,
        password_feedback
      } from "./pkg/password_strength_meter.js";

      await init();

      const input = document.getElementById("password");
      const score = document.getElementById("score");
      const label = document.getElementById("label");
      const feedback = document.getElementById("feedback");

      const render = () => {
        const value = input.value;
        score.textContent = `score: ${score_password(value)}`;
        label.textContent = `label: ${strength_label(value)}`;
        feedback.textContent = `feedback: ${password_feedback(value)}`;
      };

      input.addEventListener("input", render);
      render();
    </script>
  </body>
</html>
```

## Beginner advice

If you are new to wasm, follow this order:

1. Start with one exported function.
2. Return strings or numbers first.
3. Test the Rust logic natively with `cargo test`.
4. Then load it from a simple HTML page.
5. Add browser APIs later with `web-sys` only when needed.

## Summary

The important idea is:

- **HTML/CSS/JavaScript handle the page**
- **Rust wasm handles the reusable logic**

That makes wasm much easier to learn than trying to move the whole frontend into Rust at once.
