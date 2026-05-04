use image::{DynamicImage, GrayImage, ImageBuffer, Rgb};
use std::{error::Error, fs, path::PathBuf};

type AppResult<T> = Result<T, Box<dyn Error>>;

fn main() -> AppResult<()> {
    let project_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let output_dir = project_dir.join("output");
    fs::create_dir_all(&output_dir)?;

    let scan = DynamicImage::ImageRgb8(generate_scan(900, 700));
    let portrait = ensure_portrait(&scan);
    let grayscale = portrait.grayscale();
    let bounding_box =
        content_bounds(&grayscale.to_luma8(), 245).ok_or("No document content found")?;
    let cropped = portrait.crop_imm(
        bounding_box.0,
        bounding_box.1,
        bounding_box.2,
        bounding_box.3,
    );

    scan.save(output_dir.join("01_scan.png"))?;
    portrait.save(output_dir.join("02_portrait.png"))?;
    grayscale.save(output_dir.join("03_grayscale.png"))?;
    cropped.save(output_dir.join("04_cropped_content.png"))?;

    println!(
        "Created document workflow outputs in {}",
        output_dir.display()
    );
    Ok(())
}

fn ensure_portrait(image: &DynamicImage) -> DynamicImage {
    if image.width() > image.height() {
        image.rotate90()
    } else {
        image.clone()
    }
}

fn content_bounds(image: &GrayImage, white_threshold: u8) -> Option<(u32, u32, u32, u32)> {
    let mut min_x = image.width();
    let mut min_y = image.height();
    let mut max_x = 0;
    let mut max_y = 0;
    let mut found = false;

    for y in 0..image.height() {
        for x in 0..image.width() {
            if image.get_pixel(x, y)[0] < white_threshold {
                min_x = min_x.min(x);
                min_y = min_y.min(y);
                max_x = max_x.max(x);
                max_y = max_y.max(y);
                found = true;
            }
        }
    }

    if found {
        Some((min_x, min_y, max_x - min_x + 1, max_y - min_y + 1))
    } else {
        None
    }
}

fn generate_scan(width: u32, height: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    ImageBuffer::from_fn(width, height, |x, y| {
        let inside_page = x > 120 && x < width - 120 && y > 80 && y < height - 80;
        let text_line = inside_page
            && y > 140
            && y < height - 140
            && (y / 32) % 2 == 0
            && x > 180
            && x < width - 180;
        let header = inside_page && y > 110 && y < 150 && x > 180 && x < width - 260;

        if header || text_line {
            Rgb([40, 40, 40])
        } else if inside_page {
            Rgb([250, 250, 250])
        } else {
            let shadow = if x < 15 || y < 15 || x >= width - 15 || y >= height - 15 {
                220
            } else {
                235
            };
            Rgb([shadow, shadow, shadow])
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::Luma;

    #[test]
    fn content_bounds_finds_dark_region() {
        let image = GrayImage::from_fn(10, 10, |x, y| {
            if x >= 2 && x <= 6 && y >= 3 && y <= 8 {
                Luma([0])
            } else {
                Luma([255])
            }
        });

        assert_eq!(content_bounds(&image, 240), Some((2, 3, 5, 6)));
    }
}
