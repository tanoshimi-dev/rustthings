use image::{
    DynamicImage, GenericImageView, GrayImage, ImageBuffer, Luma, Rgb, RgbImage,
    imageops::FilterType,
};
use std::{error::Error, fs, path::PathBuf};

type AppResult<T> = Result<T, Box<dyn Error>>;

const GLYPH_WIDTH: u32 = 5;
const GLYPH_HEIGHT: u32 = 7;
const SCALE: u32 = 12;
const SCENE_TEXT: &str = "TRUST";

fn main() -> AppResult<()> {
    let project_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let output_dir = project_dir.join("output");
    fs::create_dir_all(&output_dir)?;

    let scene = render_shop_scene(SCENE_TEXT, SCALE);
    let sign_bounds = detect_sign_bounds(&scene).ok_or("Sign bounds not found")?;
    let sign_crop = scene
        .view(sign_bounds.0, sign_bounds.1, sign_bounds.2, sign_bounds.3)
        .to_image();
    let sign_gray = DynamicImage::ImageRgb8(sign_crop.clone()).to_luma8();
    let binary = threshold(&sign_gray, 150);
    let text_bounds = bounding_box(&binary, |pixel| pixel[0] == 0).ok_or("Text not found")?;
    let text_binary = binary
        .view(text_bounds.0, text_bounds.1, text_bounds.2, text_bounds.3)
        .to_image();
    let segments = segment_characters(&text_binary);
    let recognized = recognize_characters(&text_binary, &segments);
    let annotated_scene = draw_region_box(&scene, sign_bounds);
    let annotated_text = draw_boxes(&text_binary, &segments);

    DynamicImage::ImageRgb8(scene.clone()).save(output_dir.join("01_scene.png"))?;
    DynamicImage::ImageRgb8(annotated_scene).save(output_dir.join("02_detected_sign.png"))?;
    DynamicImage::ImageRgb8(sign_crop).save(output_dir.join("03_sign_crop.png"))?;
    DynamicImage::ImageLuma8(binary).save(output_dir.join("04_binary_sign.png"))?;
    DynamicImage::ImageLuma8(text_binary.clone()).save(output_dir.join("05_text_region.png"))?;
    DynamicImage::ImageRgb8(annotated_text).save(output_dir.join("06_detected_characters.png"))?;

    for (index, (start, end)) in segments.iter().enumerate() {
        let char_image = extract_character_image(&text_binary, *start, *end);
        DynamicImage::ImageLuma8(char_image)
            .save(output_dir.join(format!("07_character_{:02}.png", index + 1)))?;
    }

    fs::write(output_dir.join("recognized.txt"), format!("{recognized}\n"))?;

    println!("Recognized scene text: {recognized}");
    println!("Outputs written to {}", output_dir.display());
    Ok(())
}

fn render_shop_scene(text: &str, scale: u32) -> RgbImage {
    let width = 960;
    let height = 640;
    let sign_x = 250;
    let sign_y = 180;
    let sign_width = 460;
    let sign_height = 150;
    let mut image = RgbImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel = if y < 230 {
                [
                    90u8.saturating_add(((x * 60) / width) as u8),
                    140u8.saturating_add(((y * 35) / 230) as u8),
                    210u8,
                ]
            } else {
                let shade = 55u8.saturating_add(((y - 230) * 75 / (height - 230)) as u8);
                [shade, shade.saturating_add(8), shade.saturating_add(14)]
            };
            image.put_pixel(x, y, Rgb(pixel));
        }
    }

    for y in 280..height {
        for x in (0..width).step_by(120) {
            let building_width = 80;
            if x + building_width >= width {
                continue;
            }
            for bx in x..x + building_width {
                let tone = 42u8.saturating_add(((bx + y) % 18) as u8);
                image.put_pixel(bx, y, Rgb([tone, tone, tone.saturating_add(6)]));
            }
        }
    }

    for y in sign_y..sign_y + sign_height {
        for x in sign_x..sign_x + sign_width {
            let border = x < sign_x + 8
                || y < sign_y + 8
                || x >= sign_x + sign_width - 8
                || y >= sign_y + sign_height - 8;
            let color = if border {
                [110, 80, 45]
            } else {
                [232, 214, 170]
            };
            image.put_pixel(x, y, Rgb(color));
        }
    }

    draw_text(&mut image, sign_x + 34, sign_y + 32, text, scale);
    add_scene_noise(&mut image);
    image
}

fn draw_text(image: &mut RgbImage, x_offset: u32, y_offset: u32, text: &str, scale: u32) {
    let mut cursor = x_offset;
    for ch in text.chars() {
        draw_glyph(image, cursor, y_offset, ch, scale);
        cursor += GLYPH_WIDTH * scale + scale;
    }
}

fn draw_glyph(image: &mut RgbImage, x_offset: u32, y_offset: u32, ch: char, scale: u32) {
    let glyph = glyph_pattern(ch);
    for (row, line) in glyph.iter().enumerate() {
        for (col, cell) in line.chars().enumerate() {
            let color = if cell == '1' {
                Rgb([35, 28, 25])
            } else {
                Rgb([232, 214, 170])
            };
            for dy in 0..scale {
                for dx in 0..scale {
                    image.put_pixel(
                        x_offset + col as u32 * scale + dx,
                        y_offset + row as u32 * scale + dy,
                        color,
                    );
                }
            }
        }
    }
}

fn add_scene_noise(image: &mut RgbImage) {
    for x in (0..image.width()).step_by(27) {
        for y in (0..image.height()).step_by(19) {
            let pixel = image.get_pixel(x, y);
            let adjustment = ((x + y) % 7) as u8;
            image.put_pixel(
                x,
                y,
                Rgb([
                    pixel[0].saturating_sub(adjustment),
                    pixel[1].saturating_sub(adjustment / 2),
                    pixel[2].saturating_sub(adjustment / 3),
                ]),
            );
        }
    }
}

fn detect_sign_bounds(image: &RgbImage) -> Option<(u32, u32, u32, u32)> {
    let bounds = bounding_box_rgb(image, |pixel| {
        pixel[0] >= 205 && pixel[1] >= 190 && pixel[2] <= 190
    })?;
    Some(inset_bounds(bounds, 12))
}

fn inset_bounds(bounds: (u32, u32, u32, u32), inset: u32) -> (u32, u32, u32, u32) {
    let inset_x = inset.min(bounds.2 / 3);
    let inset_y = inset.min(bounds.3 / 3);
    (
        bounds.0 + inset_x,
        bounds.1 + inset_y,
        bounds.2.saturating_sub(inset_x * 2),
        bounds.3.saturating_sub(inset_y * 2),
    )
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

fn bounding_box<F>(image: &GrayImage, predicate: F) -> Option<(u32, u32, u32, u32)>
where
    F: Fn(Luma<u8>) -> bool,
{
    let mut min_x = image.width();
    let mut min_y = image.height();
    let mut max_x = 0;
    let mut max_y = 0;
    let mut found = false;

    for y in 0..image.height() {
        for x in 0..image.width() {
            if predicate(*image.get_pixel(x, y)) {
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

fn bounding_box_rgb<F>(image: &RgbImage, predicate: F) -> Option<(u32, u32, u32, u32)>
where
    F: Fn(Rgb<u8>) -> bool,
{
    let mut min_x = image.width();
    let mut min_y = image.height();
    let mut max_x = 0;
    let mut max_y = 0;
    let mut found = false;

    for y in 0..image.height() {
        for x in 0..image.width() {
            if predicate(*image.get_pixel(x, y)) {
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

fn draw_region_box(image: &RgbImage, bounds: (u32, u32, u32, u32)) -> RgbImage {
    let mut annotated = image.clone();
    let (x, y, width, height) = bounds;

    for px in x..x + width {
        annotated.put_pixel(px, y, Rgb([240, 30, 30]));
        annotated.put_pixel(px, y + height - 1, Rgb([240, 30, 30]));
    }
    for py in y..y + height {
        annotated.put_pixel(x, py, Rgb([240, 30, 30]));
        annotated.put_pixel(x + width - 1, py, Rgb([240, 30, 30]));
    }

    annotated
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
    fn detects_sign_region() {
        let scene = render_shop_scene(SCENE_TEXT, SCALE);
        let bounds = detect_sign_bounds(&scene).unwrap();
        assert!(bounds.2 > 300);
        assert!(bounds.3 > 80);
    }

    #[test]
    fn recognizes_scene_text() {
        let scene = render_shop_scene(SCENE_TEXT, SCALE);
        let bounds = detect_sign_bounds(&scene).unwrap();
        let sign_crop = scene
            .view(bounds.0, bounds.1, bounds.2, bounds.3)
            .to_image();
        let sign_gray = DynamicImage::ImageRgb8(sign_crop).to_luma8();
        let binary = threshold(&sign_gray, 150);
        let text_bounds = bounding_box(&binary, |pixel| pixel[0] == 0).unwrap();
        let text_binary = binary
            .view(text_bounds.0, text_bounds.1, text_bounds.2, text_bounds.3)
            .to_image();
        let segments = segment_characters(&text_binary);
        let recognized = recognize_characters(&text_binary, &segments);
        assert_eq!(recognized, SCENE_TEXT);
    }
}
