use image::{DynamicImage, ImageBuffer, Rgb};
use std::{error::Error, fs, path::PathBuf};

type AppResult<T> = Result<T, Box<dyn Error>>;

fn main() -> AppResult<()> {
    let project_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let output_dir = project_dir.join("output");
    fs::create_dir_all(&output_dir)?;

    DynamicImage::ImageRgb8(gradient(512, 256)).save(output_dir.join("01_gradient.png"))?;
    DynamicImage::ImageRgb8(checkerboard(512, 256, 32))
        .save(output_dir.join("02_checkerboard.png"))?;
    DynamicImage::ImageRgb8(heatmap(512, 256)).save(output_dir.join("03_heatmap.png"))?;

    println!("Generated pixel-level images in {}", output_dir.display());
    Ok(())
}

fn gradient(width: u32, height: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let safe_width = width.max(1);
    let safe_height = height.max(1);

    ImageBuffer::from_fn(width, height, |x, y| {
        let red = ((x * 255) / safe_width) as u8;
        let green = ((y * 255) / safe_height) as u8;
        let blue = 255u8.saturating_sub(red / 2);
        Rgb([red, green, blue])
    })
}

fn checkerboard(width: u32, height: u32, cell: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    ImageBuffer::from_fn(width, height, |x, y| {
        let dark = ((x / cell) + (y / cell)) % 2 == 0;
        if dark {
            Rgb([40, 40, 60])
        } else {
            Rgb([220, 220, 240])
        }
    })
}

fn heatmap(width: u32, height: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let center_x = width as f32 / 2.0;
    let center_y = height as f32 / 2.0;
    let max_distance = (center_x * center_x + center_y * center_y).sqrt().max(1.0);

    ImageBuffer::from_fn(width, height, |x, y| {
        let dx = x as f32 - center_x;
        let dy = y as f32 - center_y;
        let normalized = 1.0 - ((dx * dx + dy * dy).sqrt() / max_distance);
        palette(normalized.clamp(0.0, 1.0))
    })
}

fn palette(value: f32) -> Rgb<u8> {
    let red = (255.0 * value) as u8;
    let green = (255.0 * (1.0 - (value - 0.5).abs() * 2.0).max(0.0)) as u8;
    let blue = (255.0 * (1.0 - value)) as u8;
    Rgb([red, green, blue])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gradient_uses_requested_size() {
        assert_eq!(gradient(32, 16).dimensions(), (32, 16));
    }
}
