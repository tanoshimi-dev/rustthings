use image::{DynamicImage, ImageBuffer, Rgb};
use std::{error::Error, fs, path::PathBuf};

type AppResult<T> = Result<T, Box<dyn Error>>;

fn main() -> AppResult<()> {
    let project_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let output_dir = project_dir.join("output");
    fs::create_dir_all(&output_dir)?;

    let source = DynamicImage::ImageRgb8(generate_scene(512, 512));

    source.save(output_dir.join("01_original.png"))?;
    source
        .grayscale()
        .save(output_dir.join("02_grayscale.png"))?;
    source
        .brighten(28)
        .save(output_dir.join("03_brighten.png"))?;
    source
        .adjust_contrast(24.0)
        .save(output_dir.join("04_contrast.png"))?;
    source.blur(2.8).save(output_dir.join("05_blur.png"))?;
    source
        .rotate90()
        .save(output_dir.join("06_rotate_90.png"))?;
    source
        .fliph()
        .save(output_dir.join("07_flip_horizontal.png"))?;

    println!(
        "Created color and filter examples in {}",
        output_dir.display()
    );
    Ok(())
}

fn generate_scene(width: u32, height: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let safe_width = width.max(1);
    let safe_height = height.max(1);
    let safe_sum = (width + height).max(1);

    ImageBuffer::from_fn(width, height, |x, y| {
        let red = (((x + y / 2) * 255) / safe_sum) as u8;
        let green = ((y * 255) / safe_height) as u8;
        let blue = 255u8.saturating_sub(((x * 255) / safe_width) as u8);
        let accent = if (x / 32 + y / 32) % 2 == 0 { 18 } else { 0 };
        Rgb([
            red.saturating_add(accent),
            green.saturating_add(accent),
            blue,
        ])
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_scene_has_expected_dimensions() {
        let scene = generate_scene(200, 100);
        assert_eq!(scene.dimensions(), (200, 100));
    }
}
