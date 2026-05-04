# Color and filter operations

## Purpose

This sample shows common pixel transformations that are useful in everyday image editing and preprocessing.

## Project

- `E:\dev\vs_code\products\learning\rustthings\image\prj\03-color-and-filter-operations`

## What the program does

- generates a colorful source image
- converts it to grayscale
- brightens it
- increases contrast
- blurs it
- rotates and flips it

## Key Rust APIs

- `grayscale`
- `brighten`
- `adjust_contrast`
- `blur`
- `rotate90`
- `fliph`

## Run

```powershell
Set-Location E:\dev\vs_code\products\learning\rustthings\image\prj\03-color-and-filter-operations
cargo run
```

## Output

Processed images are written to the project's `output\` directory.
