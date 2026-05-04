# Document and photo workflows

## Purpose

This sample shows how scanned pages or photos are often normalized before archiving or OCR.

## Project

- `E:\dev\vs_code\products\learning\rustthings\image\prj\09-document-and-photo-workflows`

## What the program does

- generates a scan-like page image
- rotates it into portrait orientation if needed
- converts it to grayscale
- detects the non-white content bounds
- crops extra whitespace

## Key Rust APIs

- `rotate90`
- `grayscale`
- direct grayscale pixel scanning
- `crop_imm`

## Run

```powershell
Set-Location E:\dev\vs_code\products\learning\rustthings\image\prj\09-document-and-photo-workflows
cargo run
```

## Output

The `output\` directory shows the original scan, normalized portrait view, grayscale version, and cropped content.
