use image::{DynamicImage, ImageBuffer, Rgb, imageops::FilterType};
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

type AppResult<T> = Result<T, Box<dyn Error>>;

fn main() -> AppResult<()> {
    let project_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let input_dir = project_dir.join("input");
    let output_dir = project_dir.join("output");
    fs::create_dir_all(&input_dir)?;
    fs::create_dir_all(&output_dir)?;

    create_sample_inputs(&input_dir)?;

    let input_files = png_files(&input_dir)?;
    for input_file in &input_files {
        let image = image::open(input_file)?;
        let file_stem = input_file
            .file_stem()
            .and_then(|value| value.to_str())
            .ok_or("Invalid input filename")?;
        let standardized = image.resize(256, 256, FilterType::CatmullRom).grayscale();
        standardized.save(output_dir.join(format!("{file_stem}_standardized.png")))?;
    }

    println!("Processed {} images.", input_files.len());
    println!("Input folder: {}", input_dir.display());
    println!("Output folder: {}", output_dir.display());

    Ok(())
}

fn create_sample_inputs(input_dir: &Path) -> AppResult<()> {
    let samples = [
        ("sample_city.png", [220, 90, 80]),
        ("sample_forest.png", [80, 170, 90]),
        ("sample_ocean.png", [70, 120, 220]),
    ];

    for (file_name, tint) in samples {
        let image = DynamicImage::ImageRgb8(generate_sample(600, 400, tint));
        image.save(input_dir.join(file_name))?;
    }

    Ok(())
}

fn png_files(dir: &Path) -> AppResult<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path
            .extension()
            .and_then(|value| value.to_str())
            .is_some_and(|ext| ext.eq_ignore_ascii_case("png"))
        {
            files.push(path);
        }
    }

    files.sort();
    Ok(files)
}

fn generate_sample(width: u32, height: u32, tint: [u8; 3]) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let safe_width = width.max(1);
    let safe_height = height.max(1);

    ImageBuffer::from_fn(width, height, |x, y| {
        let vignette = (((x * 255) / safe_width) / 4 + ((y * 255) / safe_height) / 4) as u8;
        Rgb([
            tint[0].saturating_sub(vignette / 2),
            tint[1].saturating_sub(vignette / 3),
            tint[2].saturating_sub(vignette / 4),
        ])
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn png_files_only_returns_png_entries() {
        let project_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let temp_dir = project_dir.join("target").join("test-batch-inputs");
        fs::create_dir_all(&temp_dir).unwrap();
        fs::write(temp_dir.join("one.png"), b"png").unwrap();
        fs::write(temp_dir.join("two.jpg"), b"jpg").unwrap();

        let files = png_files(&temp_dir).unwrap();
        assert_eq!(files.len(), 1);
        assert!(files[0].ends_with("one.png"));

        fs::remove_dir_all(temp_dir).unwrap();
    }
}
