# Resume image OCR

This project extracts text from the real image file `degital-text.png` using:

1. Rust image preprocessing with the `image` crate
2. Local Tesseract OCR with Japanese language data

## What it does

- loads `degital-text.png`
- crops the main content area
- upscales and improves contrast for OCR
- saves intermediate images to `output\`
- runs Tesseract OCR on the original and processed images
- saves OCR text results for comparison

## Requirements

- Tesseract installed locally at `C:\Program Files\Tesseract-OCR\tesseract.exe`
  or available via `TESSERACT_PATH`
- `tessdata\jpn.traineddata` present in this project

## Run

```powershell
Set-Location E:\dev\vs_code\products\learning\rustthings\image\prj\15-resume
cargo run
```

## Output

The project writes:

- `output\01_cropped.png`
- `output\02_processed.png`
- `output\03_binary_preview.png`
- `output\ocr-original-psm6.txt`
- `output\ocr-processed-psm6.txt`
- `output\ocr-processed-psm4.txt`
- `output\recognized.txt`
