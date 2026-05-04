# Load and save images

## Purpose

This sample shows the most basic image workflow in Rust:

1. create or load an image
2. decode it with the `image` crate
3. save copies in different formats

## Project

- `E:\dev\vs_code\products\learning\rustthings\image\prj\01-load-and-save`

## What the program does

- generates a sample PNG when no input file is provided
- opens the source image
- saves a PNG copy
- saves a BMP copy

## Key Rust APIs

- `image::open`
- `DynamicImage::save`
- `DynamicImage::save_with_format`

## Run

```powershell
Set-Location E:\dev\vs_code\products\learning\rustthings\image\prj\01-load-and-save
cargo run
```

Or with your own file:

```powershell
cargo run -- "C:\path\to\image.png"
```

## Output

Files are written to the project's `output\` directory.
