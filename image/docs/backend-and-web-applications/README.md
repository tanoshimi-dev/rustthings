# Backend and web applications

## Purpose

This sample models a common backend image flow: validate an upload, then generate derivatives such as avatars and banners.

## Project

- `E:\dev\vs_code\products\learning\rustthings\image\prj\05-backend-and-web-applications`

## What the program does

- generates a sample upload when none is supplied
- validates extension and minimum size
- creates a square avatar
- creates a banner crop

## Key Rust APIs

- `image::open`
- `dimensions`
- `crop_imm`
- `resize_exact`

## Run

```powershell
Set-Location E:\dev\vs_code\products\learning\rustthings\image\prj\05-backend-and-web-applications
cargo run
```

With your own upload:

```powershell
cargo run -- "C:\path\to\upload.png"
```

## Output

Look in `uploads\` for the generated input and `output\` for the derived images.
