use image::{DynamicImage, GrayImage, ImageBuffer, Luma, imageops::FilterType};
use std::{
    env,
    error::Error,
    ffi::OsString,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

type AppResult<T> = Result<T, Box<dyn Error>>;

fn main() -> AppResult<()> {
    let project_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let input_path = project_dir.join("degital-text.png");
    let output_dir = project_dir.join("output");
    let tessdata_dir = project_dir.join("tessdata");
    fs::create_dir_all(&output_dir)?;

    let source = image::open(&input_path)?;
    let grayscale = source.grayscale().to_luma8();
    let content_bounds =
        content_bounds(&grayscale, 245).unwrap_or((0, 0, grayscale.width(), grayscale.height()));
    let crop_bounds = expand_bounds(content_bounds, grayscale.width(), grayscale.height(), 24);
    let cropped = source
        .crop_imm(crop_bounds.0, crop_bounds.1, crop_bounds.2, crop_bounds.3)
        .grayscale()
        .to_luma8();
    let processed = preprocess_for_ocr(&cropped);
    let binary_preview = binary_preview(&processed, 190);

    let cropped_path = output_dir.join("01_cropped.png");
    let processed_path = output_dir.join("02_processed.png");
    let binary_path = output_dir.join("03_binary_preview.png");
    DynamicImage::ImageLuma8(cropped.clone()).save(&cropped_path)?;
    DynamicImage::ImageLuma8(processed.clone()).save(&processed_path)?;
    DynamicImage::ImageLuma8(binary_preview).save(&binary_path)?;

    let raw_text = run_tesseract(&input_path, &tessdata_dir, 6)?;
    let processed_psm6 = run_tesseract(&processed_path, &tessdata_dir, 6)?;
    let processed_psm4 = run_tesseract(&processed_path, &tessdata_dir, 4)?;

    fs::write(output_dir.join("ocr-original-psm6.txt"), &raw_text)?;
    fs::write(output_dir.join("ocr-processed-psm6.txt"), &processed_psm6)?;
    fs::write(output_dir.join("ocr-processed-psm4.txt"), &processed_psm4)?;
    fs::write(output_dir.join("recognized.txt"), &processed_psm6)?;

    println!(
        "Primary OCR result saved to: {}",
        output_dir.join("recognized.txt").display()
    );
    println!("Also saved comparison outputs from original and alternate page segmentation modes.");

    Ok(())
}

fn preprocess_for_ocr(image: &GrayImage) -> GrayImage {
    let resized = DynamicImage::ImageLuma8(image.clone())
        .resize(
            image.width() * 2,
            image.height() * 2,
            FilterType::CatmullRom,
        )
        .to_luma8();
    let contrasted = DynamicImage::ImageLuma8(resized)
        .adjust_contrast(28.0)
        .brighten(12)
        .to_luma8();

    ImageBuffer::from_fn(contrasted.width(), contrasted.height(), |x, y| {
        let value = contrasted.get_pixel(x, y)[0];
        let normalized = if value > 235 {
            255
        } else if value < 120 {
            0
        } else {
            value
        };
        Luma([normalized])
    })
}

fn binary_preview(image: &GrayImage, cutoff: u8) -> GrayImage {
    ImageBuffer::from_fn(image.width(), image.height(), |x, y| {
        if image.get_pixel(x, y)[0] < cutoff {
            Luma([0])
        } else {
            Luma([255])
        }
    })
}

fn run_tesseract(image_path: &Path, tessdata_dir: &Path, psm: u8) -> AppResult<String> {
    let tesseract_path = find_tesseract()?;
    let output = Command::new(tesseract_path)
        .arg(image_path)
        .arg("stdout")
        .arg("--tessdata-dir")
        .arg(tessdata_dir)
        .arg("-l")
        .arg("jpn")
        .arg("--psm")
        .arg(psm.to_string())
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Tesseract failed: {stderr}").into());
    }

    Ok(String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n"))
}

fn find_tesseract() -> AppResult<PathBuf> {
    if let Some(path) = env::var_os("TESSERACT_PATH") {
        let candidate = PathBuf::from(path);
        if candidate.exists() {
            return Ok(candidate);
        }
    }

    let default = PathBuf::from(r"C:\Program Files\Tesseract-OCR\tesseract.exe");
    if default.exists() {
        return Ok(default);
    }

    if let Ok(path_value) = env::var("PATH") {
        for entry in env::split_paths(&OsString::from(path_value)) {
            let candidate = entry.join("tesseract.exe");
            if candidate.exists() {
                return Ok(candidate);
            }
        }
    }

    Err("Could not find tesseract.exe. Install Tesseract or set TESSERACT_PATH.".into())
}

fn content_bounds(image: &GrayImage, threshold: u8) -> Option<(u32, u32, u32, u32)> {
    let mut min_x = image.width();
    let mut min_y = image.height();
    let mut max_x = 0;
    let mut max_y = 0;
    let mut found = false;

    for y in 0..image.height() {
        for x in 0..image.width() {
            if image.get_pixel(x, y)[0] < threshold {
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

fn expand_bounds(
    bounds: (u32, u32, u32, u32),
    image_width: u32,
    image_height: u32,
    margin: u32,
) -> (u32, u32, u32, u32) {
    let left = bounds.0.saturating_sub(margin);
    let top = bounds.1.saturating_sub(margin);
    let right = (bounds.0 + bounds.2 + margin).min(image_width);
    let bottom = (bounds.1 + bounds.3 + margin).min(image_height);
    (left, top, right - left, bottom - top)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn content_bounds_finds_non_white_region() {
        let image = GrayImage::from_fn(20, 10, |x, y| {
            if x >= 4 && x <= 12 && y >= 2 && y <= 7 {
                Luma([50])
            } else {
                Luma([255])
            }
        });

        assert_eq!(content_bounds(&image, 240), Some((4, 2, 9, 6)));
    }

    #[test]
    fn expand_bounds_stays_inside_image() {
        let expanded = expand_bounds((5, 5, 10, 10), 20, 20, 8);
        assert_eq!(expanded, (0, 0, 20, 20));
    }
}
