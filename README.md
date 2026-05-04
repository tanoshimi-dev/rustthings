# rustthings

A personal Rust learning workspace with small projects grouped by topic.

## Directories

| Path | Purpose |
| --- | --- |
| `image\` | Rust image processing and OCR examples |
| `sound\` | Rust audio and sound experiments |
| `wasm\` | Rust WebAssembly experiments |

## Start here

If you want the image-processing examples, open:

- `image\README.md`

That directory contains standalone Rust projects for:

- image loading and saving
- resizing and filtering
- pixel generation
- document and photo processing
- OCR-style text extraction

## Running examples

Move into the project you want and run:

```powershell
cargo run
```

Example:

```powershell
Set-Location .\image\prj\01-load-and-save
cargo run
```
