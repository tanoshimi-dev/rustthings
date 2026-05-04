use image::{
    DynamicImage, GenericImageView, GrayImage, ImageBuffer, Luma, Rgb, RgbImage,
    imageops::FilterType,
};
use std::{error::Error, fs, path::PathBuf};

type AppResult<T> = Result<T, Box<dyn Error>>;

const GLYPH_WIDTH: u32 = 5;
const GLYPH_HEIGHT: u32 = 7;
const SCALE: u32 = 12;

fn main() -> AppResult<()> {
    let project_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let output_dir = project_dir.join("output");
    fs::create_dir_all(&output_dir)?;

    let source = render_text_image("RUST2025", SCALE);
    let binary = threshold(&source, 140);
    let segments = segment_characters(&binary);
    let recognized = recognize_characters(&binary, &segments);
    let annotated = draw_boxes(&binary, &segments);

    DynamicImage::ImageLuma8(source.clone()).save(output_dir.join("01_source_text.png"))?;
    DynamicImage::ImageLuma8(binary.clone()).save(output_dir.join("02_binary_text.png"))?;
    DynamicImage::ImageRgb8(annotated).save(output_dir.join("03_detected_characters.png"))?;

    for (index, (start, end)) in segments.iter().enumerate() {
        let char_image = extract_character_image(&binary, *start, *end);
        DynamicImage::ImageLuma8(char_image)
            .save(output_dir.join(format!("04_character_{:02}.png", index + 1)))?;
    }

    fs::write(output_dir.join("recognized.txt"), format!("{recognized}\n"))?;

    println!("Recognized text: {recognized}");
    println!("Detected {} character regions.", segments.len());
    println!("Outputs written to {}", output_dir.display());
    Ok(())
}

fn render_text_image(text: &str, scale: u32) -> GrayImage {
    let margin = scale;
    let spacing = scale;
    let glyph_count = text.chars().count() as u32;
    let width =
        margin * 2 + glyph_count * GLYPH_WIDTH * scale + glyph_count.saturating_sub(1) * spacing;
    let height = margin * 2 + GLYPH_HEIGHT * scale;
    let mut image = GrayImage::from_pixel(width, height, Luma([255]));

    let mut x_offset = margin;
    for ch in text.chars() {
        draw_glyph(&mut image, x_offset, margin, ch, scale);
        x_offset += GLYPH_WIDTH * scale + spacing;
    }

    add_noise(&mut image);
    image
}

fn draw_glyph(image: &mut GrayImage, x_offset: u32, y_offset: u32, ch: char, scale: u32) {
    let glyph = glyph_pattern(ch);
    for (row, line) in glyph.iter().enumerate() {
        for (col, cell) in line.chars().enumerate() {
            let intensity = if cell == '1' { 20 } else { 255 };
            for dy in 0..scale {
                for dx in 0..scale {
                    image.put_pixel(
                        x_offset + col as u32 * scale + dx,
                        y_offset + row as u32 * scale + dy,
                        Luma([intensity]),
                    );
                }
            }
        }
    }
}

fn add_noise(image: &mut GrayImage) {
    for x in (0..image.width()).step_by(31) {
        for y in (0..image.height()).step_by(27) {
            if x + 1 < image.width() && y + 1 < image.height() {
                image.put_pixel(x, y, Luma([215]));
                image.put_pixel(x + 1, y + 1, Luma([225]));
            }
        }
    }
}

fn threshold(image: &GrayImage, cutoff: u8) -> GrayImage {
    ImageBuffer::from_fn(image.width(), image.height(), |x, y| {
        if image.get_pixel(x, y)[0] < cutoff {
            Luma([0])
        } else {
            Luma([255])
        }
    })
}

fn segment_characters(image: &GrayImage) -> Vec<(u32, u32)> {
    let mut segments = Vec::new();
    let mut in_region = false;
    let mut start = 0;

    for x in 0..image.width() {
        let has_ink = (0..image.height()).any(|y| image.get_pixel(x, y)[0] == 0);
        if has_ink && !in_region {
            in_region = true;
            start = x;
        } else if !has_ink && in_region {
            in_region = false;
            segments.push((start, x - 1));
        }
    }

    if in_region {
        segments.push((start, image.width() - 1));
    }

    segments
}

fn recognize_characters(image: &GrayImage, segments: &[(u32, u32)]) -> String {
    let templates = templates();

    segments
        .iter()
        .map(|(start, end)| {
            let cropped = extract_character_image(image, *start, *end);
            let normalized = DynamicImage::ImageLuma8(cropped)
                .resize_exact(GLYPH_WIDTH, GLYPH_HEIGHT, FilterType::Nearest)
                .to_luma8();
            let normalized = threshold(&normalized, 128);

            templates
                .iter()
                .min_by_key(|(_, template)| pixel_distance(&normalized, template))
                .map(|(ch, _)| *ch)
                .unwrap_or('?')
        })
        .collect()
}

fn extract_character_image(image: &GrayImage, start: u32, end: u32) -> GrayImage {
    let mut min_y = image.height();
    let mut max_y = 0;

    for y in 0..image.height() {
        for x in start..=end {
            if image.get_pixel(x, y)[0] == 0 {
                min_y = min_y.min(y);
                max_y = max_y.max(y);
            }
        }
    }

    let top = min_y.min(image.height().saturating_sub(1));
    let bottom = max_y.max(top);
    image
        .view(start, top, end - start + 1, bottom - top + 1)
        .to_image()
}

fn pixel_distance(left: &GrayImage, right: &GrayImage) -> u32 {
    let mut score = 0;
    for y in 0..left.height() {
        for x in 0..left.width() {
            if left.get_pixel(x, y)[0] != right.get_pixel(x, y)[0] {
                score += 1;
            }
        }
    }
    score
}

fn draw_boxes(image: &GrayImage, segments: &[(u32, u32)]) -> RgbImage {
    let mut rgb = DynamicImage::ImageLuma8(image.clone()).to_rgb8();
    for (start, end) in segments {
        for x in *start..=*end {
            rgb.put_pixel(x, 0, Rgb([220, 30, 30]));
            rgb.put_pixel(x, rgb.height() - 1, Rgb([220, 30, 30]));
        }
        for y in 0..rgb.height() {
            rgb.put_pixel(*start, y, Rgb([220, 30, 30]));
            rgb.put_pixel(*end, y, Rgb([220, 30, 30]));
        }
    }
    rgb
}

fn templates() -> Vec<(char, GrayImage)> {
    ['R', 'U', 'S', 'T', '2', '0', '5']
        .into_iter()
        .map(|ch| (ch, pattern_to_image(glyph_pattern(ch))))
        .collect()
}

fn pattern_to_image(pattern: [&'static str; GLYPH_HEIGHT as usize]) -> GrayImage {
    ImageBuffer::from_fn(GLYPH_WIDTH, GLYPH_HEIGHT, |x, y| {
        if pattern[y as usize].as_bytes()[x as usize] == b'1' {
            Luma([0])
        } else {
            Luma([255])
        }
    })
}

fn glyph_pattern(ch: char) -> [&'static str; GLYPH_HEIGHT as usize] {
    match ch {
        'R' => [
            "11110", "10001", "10001", "11110", "10100", "10010", "10001",
        ],
        'U' => [
            "10001", "10001", "10001", "10001", "10001", "10001", "01110",
        ],
        'S' => [
            "01111", "10000", "10000", "01110", "00001", "00001", "11110",
        ],
        'T' => [
            "11111", "00100", "00100", "00100", "00100", "00100", "00100",
        ],
        '2' => [
            "01110", "10001", "00001", "00010", "00100", "01000", "11111",
        ],
        '0' => [
            "01110", "10001", "10011", "10101", "11001", "10001", "01110",
        ],
        '5' => [
            "11111", "10000", "10000", "11110", "00001", "00001", "11110",
        ],
        _ => ["00000"; GLYPH_HEIGHT as usize],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn segments_match_number_of_characters() {
        let image = threshold(&render_text_image("RUST2025", SCALE), 140);
        let segments = segment_characters(&image);
        assert_eq!(segments.len(), 8);
    }

    #[test]
    fn recognizes_rendered_text() {
        let image = threshold(&render_text_image("RUST2025", SCALE), 140);
        let segments = segment_characters(&image);
        let recognized = recognize_characters(&image, &segments);
        assert_eq!(recognized, "RUST2025");
    }
}
