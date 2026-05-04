# Receipt OCR

This sample demonstrates a receipt-style OCR workflow in Rust with the [`image`](https://crates.io/crates/image) crate.

## What it demonstrates

1. Rendering a synthetic store receipt image.
2. Detecting the receipt paper inside a larger scene.
3. Thresholding the cropped receipt for OCR.
4. Segmenting text lines from the receipt.
5. Recognizing receipt text with built-in bitmap templates.

## Run it

```powershell
Set-Location E:\dev\vs_code\products\learning\rustthings\image\prj\12-receipt-ocr
cargo run
```

## Output

The project writes receipt OCR artifacts to `output\`, including the source scene, cropped receipt, binary image, detected lines, extracted line snippets, and `recognized.txt`.
