# Handwriting diary extraction

This sample demonstrates an educational handwriting-style OCR pipeline for a diary-page photo using the [`image`](https://crates.io/crates/image) crate.

## What it demonstrates

1. Rendering a diary page inside a photo-like scene.
2. Detecting and cropping the notebook page.
3. Thresholding the page to isolate handwriting.
4. Segmenting handwritten lines.
5. Recognizing each extracted line by matching it against built-in handwritten templates.

## Run it

```powershell
Set-Location E:\dev\vs_code\products\learning\rustthings\image\prj\14-handwriting-diary-extraction
cargo run
```

## Output

The project writes the diary photo, detected page overlay, cropped page, binary handwriting image, extracted line images, and `recognized.txt` to `output\`.
