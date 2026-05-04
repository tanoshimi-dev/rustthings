# Batch processing

## Purpose

This sample demonstrates how to process many images in one pass.

## Project

- `E:\dev\vs_code\products\learning\rustthings\image\prj\04-batch-processing`

## What the program does

- creates a few sample input PNG files
- scans the `input\` folder
- loads each image
- resizes it to a standard size
- converts it to grayscale
- writes outputs with predictable names

## Key Rust APIs

- `std::fs::read_dir`
- `image::open`
- `resize`
- `grayscale`

## Run

```powershell
Set-Location E:\dev\vs_code\products\learning\rustthings\image\prj\04-batch-processing
cargo run
```

## Output

Generated inputs appear in `input\` and processed results in `output\`.
