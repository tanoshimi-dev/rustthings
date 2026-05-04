# Rust image handling examples

A learning repository of small Rust projects for **image processing**, **image generation**, and **OCR-style text extraction**.

The examples are organized as standalone Cargo projects under `prj\`, with matching explanation docs under `docs\`.

## Repository structure

| Path | Purpose |
| --- | --- |
| `prj\` | Runnable Rust example projects |
| `docs\` | Notes and per-usecase explanations |

## What you can learn here

- loading and saving images
- resizing, cropping, and filtering
- batch image processing
- backend-style image workflows
- computer-vision preprocessing
- pixel-level image generation
- sprite sheet and graphics tooling
- document and photo cleanup
- OCR from synthetic and real images
- handwriting and scene-text extraction concepts

## Sample projects

| Use case | Project | Explanation |
| --- | --- | --- |
| Load and save images | `prj\01-load-and-save` | `docs\load-and-save\README.md` |
| Resize and crop | `prj\02-resize-and-crop` | `docs\resize-and-crop\README.md` |
| Color and filter operations | `prj\03-color-and-filter-operations` | `docs\color-and-filter-operations\README.md` |
| Batch processing | `prj\04-batch-processing` | `docs\batch-processing\README.md` |
| Backend and web applications | `prj\05-backend-and-web-applications` | `docs\backend-and-web-applications\README.md` |
| Computer vision preprocessing | `prj\06-computer-vision-preprocessing` | `docs\computer-vision-preprocessing\README.md` |
| Pixel-level generation | `prj\07-pixel-level-generation` | `docs\pixel-level-generation\README.md` |
| Game and graphics tooling | `prj\08-game-and-graphics-tooling` | `docs\game-and-graphics-tooling\README.md` |
| Document and photo workflows | `prj\09-document-and-photo-workflows` | `docs\document-and-photo-workflows\README.md` |
| Scientific imaging | `prj\10-scientific-imaging` | `docs\scientific-imaging\README.md` |
| OCR character extraction | `prj\11-ocr-character-extraction` | `docs\ocr-character-extraction\README.md` |
| Receipt OCR | `prj\12-receipt-ocr` | `docs\receipt-ocr\README.md` |
| Scene text extraction | `prj\13-scene-text-extraction` | `docs\scene-text-extraction\README.md` |
| Handwriting diary extraction | `prj\14-handwriting-diary-extraction` | `docs\handwriting-diary-extraction\README.md` |
| Resume image OCR | `prj\15-resume` | `docs\resume-image-ocr\README.md` |

## Getting started

1. Install Rust.
2. Open one project directory under `prj\`.
3. Run:

```powershell
cargo run
```

Example:

```powershell
Set-Location .\prj\01-load-and-save
cargo run
```

## OCR note

Most projects only need Rust and Cargo.

The real document OCR example in `prj\15-resume` also uses **Tesseract OCR** plus project-local Japanese language data in `tessdata\`.

## Recommended learning order

1. `01-load-and-save`
2. `02-resize-and-crop`
3. `03-color-and-filter-operations`
4. `04-batch-processing`
5. `06-computer-vision-preprocessing`
6. `11-ocr-character-extraction`
7. `12-receipt-ocr`
8. `13-scene-text-extraction`
9. `14-handwriting-diary-extraction`
10. `15-resume`

## Related docs

- `docs\image-handling-usecases.md`
- `docs\sample-projects.md`
