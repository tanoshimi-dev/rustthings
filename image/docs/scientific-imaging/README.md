# Scientific imaging

## Purpose

This sample models a measurement-style workflow where grayscale intensity values represent data rather than photographs.

## Project

- `E:\dev\vs_code\products\learning\rustthings\image\prj\10-scientific-imaging`

## What the program does

- generates a synthetic measurement field
- computes basic statistics
- maps grayscale values to a false-color image
- creates a hotspot mask using thresholding

## Key Rust APIs

- grayscale image buffers
- pixel iteration
- false-color mapping
- threshold masks

## Run

```powershell
Set-Location E:\dev\vs_code\products\learning\rustthings\image\prj\10-scientific-imaging
cargo run
```

## Output

The `output\` directory contains the raw measurement image, false-color visualization, and hotspot mask.
