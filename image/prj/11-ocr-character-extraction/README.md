# OCR character extraction

This sample demonstrates a small OCR-style pipeline in pure Rust with the [`image`](https://crates.io/crates/image) crate.

## What it demonstrates

1. Rendering a text image.
2. Converting it into a binary black-and-white image.
3. Segmenting character regions by scanning for inked columns.
4. Normalizing each extracted character.
5. Recognizing characters by matching them against built-in templates.

## Run it

```powershell
Set-Location E:\dev\vs_code\products\learning\rustthings\image\prj\11-ocr-character-extraction
cargo run
```

## Output

The project writes OCR pipeline artifacts to `output\`, including the source text image, binary image, detected character boxes, each extracted character, and `recognized.txt`.
