# Neon sunset scene

This project is a playful browser demo that asks Rust wasm to paint an animated neon landscape frame by frame.

## Native demo

```powershell
cargo run
```

## Browser demo

1. Install the wasm tools:

```powershell
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
```

2. Build the wasm package:

```powershell
wasm-pack build --target web --out-dir web/pkg
```

3. Start a local web server from `web\`:

```powershell
cd .\web
python -m http.server 8080
```

4. Open `http://localhost:8080`.
