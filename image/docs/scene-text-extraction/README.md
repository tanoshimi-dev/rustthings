# Scene text extraction

## Purpose

This sample is a more realistic OCR use case: it extracts text from a picture-like scene rather than a clean document. The text is embedded on a shop sign inside a larger image.

## Project

- `E:\dev\vs_code\products\learning\rustthings\image\prj\13-scene-text-extraction`

## What the program does

- generates a storefront-style scene with a sign
- detects the bright sign area
- crops the sign from the larger picture
- thresholds it for OCR
- crops the actual text block from the sign
- segments characters and recognizes the sign text with template matching

## Why this is useful

This is closer to real OCR from photos, such as:

- reading store signs
- extracting menu text
- recognizing labels in pictures
- detecting text in posters or advertisements

It is still educational and controlled. Real scene-text OCR usually also needs perspective correction, blur handling, and stronger text detection and recognition models.

## Key Rust APIs

- `view(...).to_image()` for region extraction
- grayscale conversion from RGB scenes
- bounding-box detection by pixel scans
- text-region cropping and character segmentation

## Run

```powershell
Set-Location E:\dev\vs_code\products\learning\rustthings\image\prj\13-scene-text-extraction
cargo run
```

## Output

The project's `output\` folder contains the full scene, detected sign overlay, cropped sign, OCR-ready text region, extracted character images, and recognized text output.
