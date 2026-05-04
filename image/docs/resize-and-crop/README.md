# Resize and crop

## Purpose

This sample demonstrates how to change image size and extract a smaller region from a larger image.

## Project

- `E:\dev\vs_code\products\learning\rustthings\image\prj\02-resize-and-crop`

## What the program does

- generates a landscape test image
- creates a resized version that fits inside a target box
- creates a thumbnail
- crops the center region
- scales the cropped region for close inspection

## Key Rust APIs

- `resize`
- `resize_exact`
- `thumbnail`
- `crop_imm`

## Run

```powershell
Set-Location E:\dev\vs_code\products\learning\rustthings\image\prj\02-resize-and-crop
cargo run
```

## Output

Look in `output\` for the original, resized, thumbnail, and cropped images.
