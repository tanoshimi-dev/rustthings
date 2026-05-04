# Resume image OCR

## Purpose

This sample shows OCR on a **real image document** instead of a synthetic generated scene. It works with the file:

- `E:\dev\vs_code\products\learning\rustthings\image\prj\15-resume\degital-text.png`

## Project

- `E:\dev\vs_code\products\learning\rustthings\image\prj\15-resume`

## What the program does

- loads the actual resume image
- detects the main content area
- crops unnecessary margins
- preprocesses the image for OCR
- runs local Tesseract OCR with Japanese language data
- saves multiple OCR text outputs for comparison

## Why this is useful

This is closer to real-world document OCR work, where the source image is not synthetic and OCR quality depends on preprocessing and engine settings.

## Run

```powershell
Set-Location E:\dev\vs_code\products\learning\rustthings\image\prj\15-resume
cargo run
```

## Output

Look in `output\` for intermediate images and OCR result files.
