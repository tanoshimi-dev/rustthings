use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb, imageops::FilterType};
use std::{error::Error, fs, path::PathBuf};

type AppResult<T> = Result<T, Box<dyn Error>>;

fn main() -> AppResult<()> {
    let project_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let output_dir = project_dir.join("output");
    fs::create_dir_all(&output_dir)?;

    let source = DynamicImage::ImageRgb8(generate_scene(800, 500));
    let center = center_crop(&source, 240, 240);

    source.save(output_dir.join("01_original.png"))?;
    source
        .resize(320, 320, FilterType::CatmullRom)
        .save(output_dir.join("02_resized_fit.png"))?;
    source
        .thumbnail(160, 160)
        .save(output_dir.join("03_thumbnail.png"))?;
    center.save(output_dir.join("04_center_crop.png"))?;
    center
        .resize_exact(320, 320, FilterType::Nearest)
        .save(output_dir.join("05_cropped_then_scaled.png"))?;

    let (width, height) = source.dimensions();
    println!(
        "Created resize and crop samples from a {}x{} image.",
        width, height
    );
    println!("Output folder: {}", output_dir.display());

    Ok(())
}

fn center_crop(source: &DynamicImage, target_width: u32, target_height: u32) -> DynamicImage {
    let (source_width, source_height) = source.dimensions();
    let crop_width = target_width.min(source_width);
    let crop_height = target_height.min(source_height);
    let x = (source_width - crop_width) / 2;
    let y = (source_height - crop_height) / 2;

    source.crop_imm(x, y, crop_width, crop_height)
}

fn generate_scene(width: u32, height: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let safe_width = width.max(1);
    let safe_height = height.max(1);

    ImageBuffer::from_fn(width, height, |x, y| {
        let red = ((x * 255) / safe_width) as u8;
        let green = ((y * 255) / safe_height) as u8;
        let blue = 180u8.saturating_sub(((x / 4 + y / 4) % 120) as u8);
        Rgb([red, green, blue])
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn center_crop_uses_requested_size_when_available() {
        let source = DynamicImage::ImageRgb8(generate_scene(400, 300));
        let cropped = center_crop(&source, 200, 120);
        assert_eq!(cropped.dimensions(), (200, 120));
    }
}
