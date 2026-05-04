use image::{DynamicImage, ImageBuffer, ImageFormat, Rgb};
use std::{
    env,
    error::Error,
    fs,
    path::{Path, PathBuf},
};

type AppResult<T> = Result<T, Box<dyn Error>>;

fn main() -> AppResult<()> {
    let project_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let output_dir = project_dir.join("output");
    fs::create_dir_all(&output_dir)?;

    let source = env::args().nth(1).map(PathBuf::from);
    let (source_path, source_image) = load_or_generate_image(source.as_deref(), &output_dir)?;
    let reopened_png = output_dir.join("02_reopened_copy.png");
    let bmp_copy = output_dir.join("03_converted_copy.bmp");

    source_image.save(output_dir.join("01_source_image.png"))?;
    let reopened = image::open(&source_path)?;
    reopened.save(&reopened_png)?;
    reopened.save_with_format(&bmp_copy, ImageFormat::Bmp)?;

    println!("Loaded: {}", source_path.display());
    println!("Dimensions: {}x{}", reopened.width(), reopened.height());
    println!("Wrote: {}", reopened_png.display());
    println!("Wrote: {}", bmp_copy.display());

    Ok(())
}

fn load_or_generate_image(
    input_path: Option<&Path>,
    output_dir: &Path,
) -> AppResult<(PathBuf, DynamicImage)> {
    match input_path {
        Some(path) => Ok((path.to_path_buf(), image::open(path)?)),
        None => {
            let generated = generate_sample_image(640, 360);
            let generated_path = output_dir.join("00_generated_input.png");

            DynamicImage::ImageRgb8(generated.clone())
                .save_with_format(&generated_path, ImageFormat::Png)?;

            Ok((generated_path, DynamicImage::ImageRgb8(generated)))
        }
    }
}

fn generate_sample_image(width: u32, height: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let safe_width = width.max(1);
    let safe_height = height.max(1);
    let safe_total = (width + height).max(1);

    ImageBuffer::from_fn(width, height, |x, y| {
        let red = ((x * 255) / safe_width) as u8;
        let green = ((y * 255) / safe_height) as u8;
        let blue = (((x + y) * 255) / safe_total) as u8;
        let checker = if ((x / 32) + (y / 32)) % 2 == 0 {
            24
        } else {
            0
        };

        Rgb([
            red.saturating_add(checker),
            green.saturating_add(checker),
            blue,
        ])
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_sample_image_uses_requested_size() {
        let image = generate_sample_image(64, 32);
        assert_eq!(image.dimensions(), (64, 32));
    }
}
