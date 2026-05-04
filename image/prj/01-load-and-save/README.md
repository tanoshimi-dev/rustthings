# Load and save images

This sample focuses on the simplest image workflow in Rust: generate or load an image, then save copies in different formats with the [`image`](https://crates.io/crates/image) crate.

## What it demonstrates

1. Opening an image from disk.
2. Generating an image in memory when no input is given.
3. Saving PNG output.
4. Converting and saving a BMP copy.

## Run it

Generate a sample image and process it:

```powershell
cargo run
```

Process your own image:

```powershell
cargo run -- "C:\path\to\your\photo.png"
```

Output files are written to `output\`.
