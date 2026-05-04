# Receipt OCR

## Purpose

This sample shows how receipt OCR differs from single-word OCR. Instead of one controlled text image, it works with a receipt-like document inside a larger scene and extracts multiple lines of text.

## Project

- `E:\dev\vs_code\products\learning\rustthings\image\prj\12-receipt-ocr`

## What the program does

- generates a synthetic receipt scene
- finds the white receipt paper inside the image
- crops the receipt region
- thresholds it into a black-and-white OCR image
- segments text lines
- performs simple template OCR on each line

## Why this is useful

Receipt OCR is a common real-world image use case for:

- expense tracking
- bookkeeping
- purchase history extraction
- store analytics

This project is still an educational sample. Real receipts usually need stronger preprocessing, deskew correction, and a dedicated OCR engine for reliable results.

## Key Rust APIs

- `view(...).to_image()` for cropping
- pixel scans for bounding boxes
- row-based line segmentation
- `resize_exact` for character normalization

## Run

```powershell
Set-Location E:\dev\vs_code\products\learning\rustthings\image\prj\12-receipt-ocr
cargo run
```

## Output

The project's `output\` folder contains the generated receipt scene, cropped receipt, binary OCR image, line overlays, extracted lines, and recognized text output.
