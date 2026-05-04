# OCR character extraction

## Purpose

This sample adds a basic OCR-style workflow to the image examples. It focuses on **character extraction from an image** and simple recognition.

## Project

- `E:\dev\vs_code\products\learning\rustthings\image\prj\11-ocr-character-extraction`

## What the program does

- generates a text image containing `RUST2025`
- binarizes the image with thresholding
- finds character regions by scanning columns
- extracts each character image
- normalizes each character to a fixed glyph size
- recognizes characters by comparing them with built-in templates

## Why this is useful

This is the core idea behind OCR pipelines:

1. preprocess the image
2. isolate text regions or characters
3. classify the extracted shapes

Real OCR systems usually use more advanced segmentation and recognition engines, but this sample shows the core mechanics clearly in a small Rust program.

## Key Rust APIs

- `GrayImage`
- thresholding with pixel transforms
- cropping with `view(...).to_image()`
- `resize_exact`

## Run

```powershell
Set-Location E:\dev\vs_code\products\learning\rustthings\image\prj\11-ocr-character-extraction
cargo run
```

## Output

The project's `output\` folder contains:

- the generated source text image
- a binary OCR-ready image
- a bounding-box visualization
- each extracted character image
- `recognized.txt` with the recognized string
