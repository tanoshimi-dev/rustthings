# Game and graphics tooling

## Purpose

This sample demonstrates a typical asset-pipeline task for games: creating sprites and combining them into a sprite sheet.

## Project

- `E:\dev\vs_code\products\learning\rustthings\image\prj\08-game-and-graphics-tooling`

## What the program does

- generates four small sprites
- saves each sprite as its own PNG
- packs them into one sprite sheet
- creates a nearest-neighbor preview so the pixel art stays sharp

## Key Rust APIs

- `ImageBuffer`
- pixel copying with `put_pixel`
- `resize_exact(..., FilterType::Nearest)`

## Run

```powershell
Set-Location E:\dev\vs_code\products\learning\rustthings\image\prj\08-game-and-graphics-tooling
cargo run
```

## Output

The `output\` directory contains the sprites, combined sheet, and enlarged preview.
