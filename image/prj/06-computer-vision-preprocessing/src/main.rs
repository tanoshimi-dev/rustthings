use image::{DynamicImage, GrayImage, ImageBuffer, Luma, Rgb};
use std::{error::Error, fs, path::PathBuf};

type AppResult<T> = Result<T, Box<dyn Error>>;

fn main() -> AppResult<()> {
    let project_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let output_dir = project_dir.join("output");
    fs::create_dir_all(&output_dir)?;

    let scene = DynamicImage::ImageRgb8(generate_scene(640, 360));
    let grayscale = scene.grayscale();
    let denoised = grayscale.blur(1.8);
    let normalized = denoised.resize_exact(224, 224, image::imageops::FilterType::Triangle);
    let binary = threshold(&normalized.to_luma8(), 118);

    scene.save(output_dir.join("01_scene.png"))?;
    grayscale.save(output_dir.join("02_grayscale.png"))?;
    denoised.save(output_dir.join("03_denoised.png"))?;
    normalized.save(output_dir.join("04_normalized.png"))?;
    DynamicImage::ImageLuma8(binary).save(output_dir.join("05_binary_mask.png"))?;

    println!(
        "Created preprocessing pipeline outputs in {}",
        output_dir.display()
    );
    Ok(())
}

fn threshold(image: &GrayImage, cutoff: u8) -> GrayImage {
    ImageBuffer::from_fn(image.width(), image.height(), |x, y| {
        let value = image.get_pixel(x, y)[0];
        if value >= cutoff {
            Luma([255])
        } else {
            Luma([0])
        }
    })
}

fn generate_scene(width: u32, height: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let center_x = width as f32 / 2.0;
    let center_y = height as f32 / 2.0;

    ImageBuffer::from_fn(width, height, |x, y| {
        let dx = x as f32 - center_x;
        let dy = y as f32 - center_y;
        let distance = (dx * dx + dy * dy).sqrt();
        let base = (255.0 - distance.min(255.0)) as u8;
        let stripe = if (x / 16 + y / 12) % 2 == 0 { 20 } else { 0 };
        Rgb([base.saturating_add(stripe), 80, 120])
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn threshold_preserves_image_size() {
        let image = GrayImage::from_fn(20, 10, |_, _| Luma([120]));
        let mask = threshold(&image, 100);
        assert_eq!(mask.dimensions(), (20, 10));
        assert_eq!(mask.get_pixel(0, 0)[0], 255);
    }
}
