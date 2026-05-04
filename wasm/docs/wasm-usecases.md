# Practical WebAssembly use cases in Rust

This note summarizes browser-focused tasks that work well when you compile Rust to WebAssembly.

## 1. Search preview generation

Use this when you need to:

- count matches in article text without server round-trips
- build small preview snippets for in-page search
- keep search helper logic deterministic across browsers

Typical Rust tasks:

- string scanning
- snippet extraction
- ASCII-insensitive comparison

## 2. Shopping cart pricing

Use this when you need to:

- calculate discounts locally before checkout
- apply shipping rules per region
- keep tax and promo logic consistent between UI states

Typical Rust tasks:

- integer-based price calculations
- coupon handling
- tax and shipping rules

## 3. Password strength meter

Use this when you need to:

- score passwords while the user types
- show immediate feedback in a signup form
- avoid sending every keystroke to a backend

Typical Rust tasks:

- character classification
- pattern checks
- scoring heuristics

## 4. Sparkline data preparation

Use this when you need to:

- convert raw numeric series into chart-ready points
- build inline SVG snippets in the browser
- normalize values before drawing to canvas or SVG

Typical Rust tasks:

- parsing user-provided CSV data
- min/max normalization
- point generation

## 5. Artistic scenes and generative visuals

Use this when you need to:

- generate animated SVG or canvas-friendly scene data in the browser
- keep visual rules deterministic without shipping a larger JavaScript art engine
- experiment with interactive toys, landing pages, or playful demos driven by Rust

Typical Rust tasks:

- procedural shape generation
- palette selection and animation timing
- deterministic math for scene layers and effects

## 6. GLSL and WebGL-driven 3D scenes

Use this when you need to:

- feed shader uniforms or shader source from Rust into a browser graphics demo
- prototype interactive 3D scenes without building a full game engine
- keep palette, camera motion, or scene presets deterministic across demos

Typical Rust tasks:

- scene preset generation
- uniform packaging and validation
- organizing shader source alongside browser-facing exports

## Rust crates to know

### `wasm-bindgen`

Best starting point for:

- exporting Rust functions to JavaScript
- exchanging strings, numbers, and vectors with browser code
- building `cdylib` crates for the wasm target

### `js-sys`

Useful when you need:

- JavaScript standard objects such as arrays and dates
- browser-compatible helpers without hand-written bindings

### `web-sys`

Useful when you need:

- direct DOM access
- browser APIs such as `Window`, `Document`, `CanvasRenderingContext2d`, or `Storage`

## Good learning path

1. Start with pure data transforms exported through `wasm-bindgen`.
2. Keep domain logic independent from the browser API surface.
3. Add `web-sys` only when you actually need DOM or canvas integration.
4. Test the Rust core logic natively before wiring it into JavaScript.

## Related examples in this workspace

See:

- `docs\sample-projects.md`

That index points to one standalone Rust sample project per use case.
