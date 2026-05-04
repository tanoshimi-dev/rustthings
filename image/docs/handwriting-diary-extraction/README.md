# Handwriting diary extraction

## Purpose

This sample shows a harder OCR-style use case: extracting handwritten text from a diary-page photo.

## Project

- `E:\dev\vs_code\products\learning\rustthings\image\prj\14-handwriting-diary-extraction`

## What the program does

- generates a diary page on a desk-like photo background
- detects the notebook page region
- crops the page from the photo
- thresholds the page to isolate handwriting
- segments line regions
- recognizes each extracted line with built-in handwritten templates

## Why this is useful

This is closer to a handwriting-recognition workflow than simple OCR. It demonstrates the preprocessing stages you would usually need before using a dedicated handwriting model.

Real handwriting extraction is much harder than this sample because real pages often include:

- perspective distortion
- shadows and page curl
- inconsistent handwriting
- crossed-out words and decorations

## Key Rust APIs

- `view(...).to_image()` for page and line extraction
- grayscale conversion and thresholding
- row-based handwritten line segmentation
- template matching on normalized extracted lines

## Run

```powershell
Set-Location E:\dev\vs_code\products\learning\rustthings\image\prj\14-handwriting-diary-extraction
cargo run
```

## Output

The project's `output\` folder contains the full diary photo, detected page, cropped page, binary handwriting image, extracted line snippets, and recognized text output.
