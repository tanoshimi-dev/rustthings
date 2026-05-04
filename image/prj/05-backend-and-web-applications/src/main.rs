use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb, imageops::FilterType};
use std::{
    env,
    error::Error,
    fs,
    path::{Path, PathBuf},
};

type AppResult<T> = Result<T, Box<dyn Error>>;

fn main() -> AppResult<()> {
    let project_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let upload_dir = project_dir.join("uploads");
    let output_dir = project_dir.join("output");
    fs::create_dir_all(&upload_dir)?;
    fs::create_dir_all(&output_dir)?;

    let input_arg = env::args().nth(1).map(PathBuf::from);
    let upload_path = ensure_upload(input_arg.as_deref(), &upload_dir)?;
    let upload = image::open(&upload_path)?;
    validate_upload(&upload_path, &upload)?;

    square_avatar(&upload, 128).save(output_dir.join("avatar_128.png"))?;
    cover_crop(&upload, 600, 200).save(output_dir.join("banner_600x200.png"))?;

    println!("Validated upload: {}", upload_path.display());
    println!(
        "Generated backend-style derivatives in {}",
        output_dir.display()
    );
    Ok(())
}

fn ensure_upload(input: Option<&Path>, upload_dir: &Path) -> AppResult<PathBuf> {
    match input {
        Some(path) => Ok(path.to_path_buf()),
        None => {
            let generated = DynamicImage::ImageRgb8(generate_upload(720, 480));
            let generated_path = upload_dir.join("generated_upload.png");
            generated.save(&generated_path)?;
            Ok(generated_path)
        }
    }
}

fn validate_upload(path: &Path, image: &DynamicImage) -> AppResult<()> {
    let extension = path
        .extension()
        .and_then(|value| value.to_str())
        .ok_or("Upload must have a file extension")?;
    if !is_allowed_extension(extension) {
        return Err(format!("Unsupported extension: {extension}").into());
    }

    let (width, height) = image.dimensions();
    if width < 128 || height < 128 {
        return Err(format!("Upload is too small: {}x{}", width, height).into());
    }

    Ok(())
}

fn is_allowed_extension(extension: &str) -> bool {
    matches!(
        extension.to_ascii_lowercase().as_str(),
        "png" | "jpg" | "jpeg" | "bmp" | "webp"
    )
}

fn square_avatar(source: &DynamicImage, size: u32) -> DynamicImage {
    let edge = source.width().min(source.height());
    let x = (source.width() - edge) / 2;
    let y = (source.height() - edge) / 2;
    source
        .crop_imm(x, y, edge, edge)
        .resize_exact(size, size, FilterType::CatmullRom)
}

fn cover_crop(source: &DynamicImage, width: u32, height: u32) -> DynamicImage {
    let target_ratio = width as f32 / height as f32;
    let source_ratio = source.width() as f32 / source.height() as f32;

    let cropped = if source_ratio > target_ratio {
        let crop_width = (source.height() as f32 * target_ratio) as u32;
        let x = (source.width() - crop_width) / 2;
        source.crop_imm(x, 0, crop_width, source.height())
    } else {
        let crop_height = (source.width() as f32 / target_ratio) as u32;
        let y = (source.height() - crop_height) / 2;
        source.crop_imm(0, y, source.width(), crop_height)
    };

    cropped.resize_exact(width, height, FilterType::CatmullRom)
}

fn generate_upload(width: u32, height: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let safe_width = width.max(1);
    let safe_height = height.max(1);

    ImageBuffer::from_fn(width, height, |x, y| {
        let red = 70u8.saturating_add(((x * 120) / safe_width) as u8);
        let green = 80u8.saturating_add(((y * 90) / safe_height) as u8);
        let blue = 160u8.saturating_sub((((x + y) * 90) / (safe_width + safe_height)) as u8);
        Rgb([red, green, blue])
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allowed_extensions_match_common_web_formats() {
        assert!(is_allowed_extension("png"));
        assert!(is_allowed_extension("JPG"));
        assert!(!is_allowed_extension("gif"));
    }
}
