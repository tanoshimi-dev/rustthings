# Computer vision preprocessing

## Purpose

This sample shows the kind of cleanup that usually happens before machine-learning or computer-vision analysis.

## Project

- `E:\dev\vs_code\products\learning\rustthings\image\prj\06-computer-vision-preprocessing`

## What the program does

- generates a noisy scene
- converts it to grayscale
- blurs it to reduce noise
- normalizes it to a fixed model input size
- creates a binary mask with thresholding

## Key Rust APIs

- `grayscale`
- `blur`
- `resize_exact`
- pixel access through `GrayImage`

## Run

```powershell
Set-Location E:\dev\vs_code\products\learning\rustthings\image\prj\06-computer-vision-preprocessing
cargo run
```

## Output

The `output\` directory contains each stage of the preprocessing pipeline.
