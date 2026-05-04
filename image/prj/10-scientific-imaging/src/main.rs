use image::{DynamicImage, GrayImage, ImageBuffer, Luma, Rgb};
use std::{error::Error, fs, path::PathBuf};

type AppResult<T> = Result<T, Box<dyn Error>>;

fn main() -> AppResult<()> {
    let project_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let output_dir = project_dir.join("output");
    fs::create_dir_all(&output_dir)?;

    let measurement = measurement_field(256, 256);
    let hotspots = threshold(&measurement, 185);
    let stats = image_stats(&measurement);

    DynamicImage::ImageLuma8(measurement.clone()).save(output_dir.join("01_measurement.png"))?;
    DynamicImage::ImageRgb8(false_color(&measurement))
        .save(output_dir.join("02_false_color.png"))?;
    DynamicImage::ImageLuma8(hotspots).save(output_dir.join("03_hotspots.png"))?;

    println!(
        "Measurement stats -> min: {}, max: {}, mean: {:.2}",
        stats.min, stats.max, stats.mean
    );
    println!("Outputs written to {}", output_dir.display());
    Ok(())
}

#[derive(Debug, PartialEq)]
struct Stats {
    min: u8,
    max: u8,
    mean: f32,
}

fn measurement_field(width: u32, height: u32) -> GrayImage {
    let center_x = width as f32 / 2.0;
    let center_y = height as f32 / 2.0;
    let max_distance = (center_x * center_x + center_y * center_y).sqrt().max(1.0);

    ImageBuffer::from_fn(width, height, |x, y| {
        let dx = x as f32 - center_x;
        let dy = y as f32 - center_y;
        let radial = 1.0 - ((dx * dx + dy * dy).sqrt() / max_distance);
        let wave = (((x as f32 / 18.0).sin() + (y as f32 / 24.0).cos()) * 0.15) + 0.15;
        let intensity = ((radial + wave).clamp(0.0, 1.0) * 255.0) as u8;
        Luma([intensity])
    })
}

fn threshold(image: &GrayImage, cutoff: u8) -> GrayImage {
    ImageBuffer::from_fn(image.width(), image.height(), |x, y| {
        if image.get_pixel(x, y)[0] >= cutoff {
            Luma([255])
        } else {
            Luma([0])
        }
    })
}

fn false_color(image: &GrayImage) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    ImageBuffer::from_fn(image.width(), image.height(), |x, y| {
        let value = image.get_pixel(x, y)[0] as f32 / 255.0;
        let red = (255.0 * value) as u8;
        let green = (255.0 * (1.0 - (value - 0.5).abs() * 2.0).max(0.0)) as u8;
        let blue = (255.0 * (1.0 - value)) as u8;
        Rgb([red, green, blue])
    })
}

fn image_stats(image: &GrayImage) -> Stats {
    let mut min = u8::MAX;
    let mut max = u8::MIN;
    let mut sum = 0u64;
    let total = (image.width() as u64 * image.height() as u64).max(1);

    for pixel in image.pixels() {
        let value = pixel[0];
        min = min.min(value);
        max = max.max(value);
        sum += value as u64;
    }

    Stats {
        min,
        max,
        mean: sum as f32 / total as f32,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn image_stats_handles_small_image() {
        let image = GrayImage::from_fn(2, 2, |x, y| match (x, y) {
            (0, 0) => Luma([0]),
            (1, 0) => Luma([100]),
            (0, 1) => Luma([200]),
            _ => Luma([255]),
        });

        let stats = image_stats(&image);
        assert_eq!(stats.min, 0);
        assert_eq!(stats.max, 255);
        assert!((stats.mean - 138.75).abs() < f32::EPSILON);
    }
}
