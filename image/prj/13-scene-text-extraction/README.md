# Scene text extraction

This sample demonstrates extracting text embedded in a picture-like scene using Rust and the [`image`](https://crates.io/crates/image) crate.

## What it demonstrates

1. Rendering a shop-sign scene with background clutter.
2. Detecting the sign region inside the larger picture.
3. Converting the detected sign area into a binary OCR image.
4. Cropping the actual text block from the sign.
5. Segmenting characters and recognizing the sign text with built-in bitmap templates.

## Run it

```powershell
Set-Location E:\dev\vs_code\products\learning\rustthings\image\prj\13-scene-text-extraction
cargo run
```

## Output

The project writes the scene, detected sign region, cropped sign, OCR-ready text region, extracted character snippets, and `recognized.txt` to `output\`.
