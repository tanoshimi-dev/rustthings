use image::{
    DynamicImage, GenericImageView, GrayImage, ImageBuffer, Luma, Rgb, RgbImage,
    imageops::FilterType,
};
use std::{error::Error, fs, path::PathBuf};

type AppResult<T> = Result<T, Box<dyn Error>>;

const GLYPH_WIDTH: u32 = 5;
const GLYPH_HEIGHT: u32 = 7;
const SCALE: u32 = 8;
const DIARY_LINES: [&str; 3] = ["DEAR DIARY", "I WROTE RUST", "TODAY WAS ART"];

fn main() -> AppResult<()> {
    let project_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let output_dir = project_dir.join("output");
    fs::create_dir_all(&output_dir)?;

    let photo = render_diary_photo(&DIARY_LINES, SCALE);
    let page_bounds = detect_page_bounds(&photo).ok_or("Diary page not found")?;
    let inner_bounds = inset_bounds(page_bounds, 20);
    let page_crop = photo
        .view(
            inner_bounds.0,
            inner_bounds.1,
            inner_bounds.2,
            inner_bounds.3,
        )
        .to_image();
    let page_gray = DynamicImage::ImageRgb8(page_crop.clone()).to_luma8();
    let binary = threshold(&page_gray, 150);
    let line_ranges = segment_lines(&binary);
    let recognized_lines = recognize_lines(&binary, &line_ranges);
    let annotated_photo = draw_region_box(&photo, inner_bounds);
    let annotated_page = draw_line_boxes(&page_crop, &line_ranges);

    DynamicImage::ImageRgb8(photo).save(output_dir.join("01_photo.png"))?;
    DynamicImage::ImageRgb8(annotated_photo).save(output_dir.join("02_detected_page.png"))?;
    DynamicImage::ImageRgb8(page_crop.clone()).save(output_dir.join("03_page_crop.png"))?;
    DynamicImage::ImageLuma8(binary.clone()).save(output_dir.join("04_binary_page.png"))?;
    DynamicImage::ImageRgb8(annotated_page).save(output_dir.join("05_detected_lines.png"))?;

    for (index, (top, bottom)) in line_ranges.iter().enumerate() {
        let line_image = extract_line_image(&binary, *top, *bottom);
        DynamicImage::ImageLuma8(line_image)
            .save(output_dir.join(format!("06_line_{:02}.png", index + 1)))?;
    }

    let recognized_text = recognized_lines.join("\n");
    fs::write(
        output_dir.join("recognized.txt"),
        format!("{recognized_text}\n"),
    )?;

    println!("Recognized diary text:");
    for line in &recognized_lines {
        println!("{line}");
    }
    println!("Outputs written to {}", output_dir.display());
    Ok(())
}

fn render_diary_photo(lines: &[&str], scale: u32) -> RgbImage {
    let width = 1000;
    let height = 820;
    let page_x = 180;
    let page_y = 60;
    let page_width = 660;
    let page_height = 700;
    let mut image = RgbImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let grain = (((x * 13 + y * 7) % 18) as u8).saturating_sub(4);
            let base = [
                128u8.saturating_add(grain / 2),
                91u8.saturating_add(grain / 3),
                58u8.saturating_add(grain / 4),
            ];
            image.put_pixel(x, y, Rgb(base));
        }
    }

    for y in page_y..page_y + page_height {
        for x in page_x..page_x + page_width {
            let shadow_band: u8 = if x > page_x + page_width - 10 || y > page_y + page_height - 10 {
                220
            } else {
                241
            };
            let warm = ((x + y) % 6) as u8;
            image.put_pixel(
                x,
                y,
                Rgb([
                    shadow_band.saturating_add(warm / 4),
                    shadow_band.saturating_sub(2),
                    shadow_band.saturating_sub(10),
                ]),
            );
        }
    }

    for row in 0..18 {
        let y = page_y + 70 + row * 34;
        if y >= page_y + page_height - 20 {
            break;
        }
        for x in page_x + 28..page_x + page_width - 28 {
            image.put_pixel(x, y, Rgb([182, 205, 232]));
        }
    }

    for y in page_y + 20..page_y + page_height - 20 {
        image.put_pixel(page_x + 58, y, Rgb([228, 164, 164]));
    }

    let mut y_offset = page_y + 92;
    for line in lines {
        draw_handwriting_line(
            &mut image,
            page_x + 94,
            y_offset,
            line,
            scale,
            line_seed(line),
        );
        y_offset += 98;
    }

    add_photo_noise(&mut image);
    image
}

fn draw_handwriting_line(
    image: &mut RgbImage,
    x_offset: u32,
    y_offset: u32,
    text: &str,
    scale: u32,
    seed: u32,
) {
    let mut cursor = x_offset as i32;
    for (index, ch) in text.chars().enumerate() {
        if ch == ' ' {
            cursor += (scale * 3) as i32;
            continue;
        }

        draw_handwritten_glyph(
            image,
            cursor,
            y_offset as i32,
            ch,
            scale,
            seed.wrapping_add(index as u32 * 17),
        );
        cursor += (GLYPH_WIDTH * scale + scale) as i32;
    }
}

fn draw_handwritten_glyph(
    image: &mut RgbImage,
    x_offset: i32,
    y_offset: i32,
    ch: char,
    scale: u32,
    seed: u32,
) {
    let glyph = glyph_pattern(ch);
    for (row, line) in glyph.iter().enumerate() {
        let slant = row as i32 * (scale as i32 / 5);
        for (col, cell) in line.chars().enumerate() {
            if cell != '1' {
                continue;
            }

            for dy in 0..scale {
                for dx in 0..scale {
                    let wobble_x = tiny_jitter(seed, col as u32, row as u32, dx, dy);
                    let wobble_y = tiny_jitter(seed ^ 0x9E37_79B9, row as u32, col as u32, dy, dx);
                    let px = x_offset + slant + col as i32 * scale as i32 + dx as i32 + wobble_x;
                    let py = y_offset + row as i32 * scale as i32 + dy as i32 + wobble_y;
                    put_ink_pixel(image, px, py, seed);
                    put_ink_pixel(image, px + 1, py, seed);
                }
            }
        }
    }
}

fn put_ink_pixel(image: &mut RgbImage, x: i32, y: i32, seed: u32) {
    if x < 0 || y < 0 || x >= image.width() as i32 || y >= image.height() as i32 {
        return;
    }

    let shade = 34u8.saturating_add((seed % 14) as u8);
    image.put_pixel(
        x as u32,
        y as u32,
        Rgb([shade / 2, shade, shade.saturating_add(26)]),
    );
}

fn tiny_jitter(seed: u32, a: u32, b: u32, c: u32, d: u32) -> i32 {
    let value = seed
        .wrapping_mul(1_664_525)
        .wrapping_add(a.wrapping_mul(31))
        .wrapping_add(b.wrapping_mul(17))
        .wrapping_add(c.wrapping_mul(13))
        .wrapping_add(d.wrapping_mul(7));
    (value % 3) as i32 - 1
}

fn line_seed(text: &str) -> u32 {
    text.bytes().fold(0u32, |acc, byte| {
        acc.wrapping_mul(33).wrapping_add(byte as u32)
    })
}

fn add_photo_noise(image: &mut RgbImage) {
    for x in (0..image.width()).step_by(23) {
        for y in (0..image.height()).step_by(17) {
            let pixel = image.get_pixel(x, y);
            let adjustment = ((x + y) % 9) as u8;
            image.put_pixel(
                x,
                y,
                Rgb([
                    pixel[0].saturating_sub(adjustment / 3),
                    pixel[1].saturating_sub(adjustment / 2),
                    pixel[2].saturating_sub(adjustment / 4),
                ]),
            );
        }
    }
}

fn detect_page_bounds(image: &RgbImage) -> Option<(u32, u32, u32, u32)> {
    bounding_box_rgb(image, |pixel| {
        pixel[0] >= 220 && pixel[1] >= 220 && pixel[2] >= 205
    })
}

fn inset_bounds(bounds: (u32, u32, u32, u32), inset: u32) -> (u32, u32, u32, u32) {
    let inset_x = inset.min(bounds.2 / 4);
    let inset_y = inset.min(bounds.3 / 4);
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

fn segment_lines(image: &GrayImage) -> Vec<(u32, u32)> {
    let mut lines = Vec::new();
    let mut start = None;
    let min_ink_pixels = (image.width() / 22).max(10);

    for y in 0..image.height() {
        let ink_pixels = (0..image.width())
            .filter(|&x| image.get_pixel(x, y)[0] == 0)
            .count() as u32;
        let has_ink = ink_pixels >= min_ink_pixels;

        match (has_ink, start) {
            (true, None) => start = Some(y),
            (false, Some(top)) => {
                lines.push((top, y - 1));
                start = None;
            }
            _ => {}
        }
    }

    if let Some(top) = start {
        lines.push((top, image.height() - 1));
    }

    lines
}

fn recognize_lines(image: &GrayImage, line_ranges: &[(u32, u32)]) -> Vec<String> {
    line_ranges
        .iter()
        .map(|(top, bottom)| recognize_line_template(&extract_line_image(image, *top, *bottom)))
        .collect()
}

fn recognize_line_template(image: &GrayImage) -> String {
    let normalized = normalize_line(image);
    diary_templates()
        .iter()
        .min_by_key(|(_, template)| line_distance(&normalized, template))
        .map(|(text, _)| text.to_string())
        .unwrap_or_else(|| "?".to_string())
}

fn diary_templates() -> Vec<(&'static str, GrayImage)> {
    DIARY_LINES
        .iter()
        .map(|line| (*line, normalize_line(&render_template_line(line, SCALE))))
        .collect()
}

fn render_template_line(text: &str, scale: u32) -> GrayImage {
    let letter_count = text.chars().filter(|&ch| ch != ' ').count() as u32;
    let space_count = text.chars().filter(|&ch| ch == ' ').count() as u32;
    let width = 24 + letter_count * (GLYPH_WIDTH * scale + scale) + space_count * (scale * 3);
    let height = 24 + GLYPH_HEIGHT * scale + 12;
    let mut canvas = RgbImage::from_pixel(width, height, Rgb([255, 255, 255]));
    draw_handwriting_line(&mut canvas, 12, 10, text, scale, line_seed(text));
    let gray = DynamicImage::ImageRgb8(canvas).to_luma8();
    let binary = threshold(&gray, 170);
    if let Some(bounds) = bounding_box(&binary, |pixel| pixel[0] == 0) {
        binary
            .view(bounds.0, bounds.1, bounds.2, bounds.3)
            .to_image()
    } else {
        binary
    }
}

fn normalize_line(image: &GrayImage) -> GrayImage {
    DynamicImage::ImageLuma8(image.clone())
        .resize_exact(420, 84, FilterType::Nearest)
        .to_luma8()
}

fn line_distance(left: &GrayImage, right: &GrayImage) -> u32 {
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

fn extract_line_image(image: &GrayImage, top: u32, bottom: u32) -> GrayImage {
    let cropped = image
        .view(0, top, image.width(), bottom - top + 1)
        .to_image();
    if let Some((left, upper, width, height)) = bounding_box(&cropped, |pixel| pixel[0] == 0) {
        cropped.view(left, upper, width, height).to_image()
    } else {
        cropped
    }
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

fn draw_line_boxes(image: &RgbImage, lines: &[(u32, u32)]) -> RgbImage {
    let mut annotated = image.clone();
    for (top, bottom) in lines {
        for x in 0..annotated.width() {
            annotated.put_pixel(x, *top, Rgb([220, 30, 30]));
            annotated.put_pixel(x, *bottom, Rgb([220, 30, 30]));
        }
        for y in *top..=*bottom {
            annotated.put_pixel(0, y, Rgb([220, 30, 30]));
            annotated.put_pixel(annotated.width() - 1, y, Rgb([220, 30, 30]));
        }
    }
    annotated
}

fn glyph_pattern(ch: char) -> [&'static str; GLYPH_HEIGHT as usize] {
    match ch {
        'A' => [
            "01110", "10001", "10001", "11111", "10001", "10001", "10001",
        ],
        'D' => [
            "11110", "10001", "10001", "10001", "10001", "10001", "11110",
        ],
        'E' => [
            "11111", "10000", "10000", "11110", "10000", "10000", "11111",
        ],
        'I' => [
            "11111", "00100", "00100", "00100", "00100", "00100", "11111",
        ],
        'O' => [
            "01110", "10001", "10001", "10001", "10001", "10001", "01110",
        ],
        'R' => [
            "11110", "10001", "10001", "11110", "10100", "10010", "10001",
        ],
        'S' => [
            "01111", "10000", "10000", "01110", "00001", "00001", "11110",
        ],
        'T' => [
            "11111", "00100", "00100", "00100", "00100", "00100", "00100",
        ],
        'U' => [
            "10001", "10001", "10001", "10001", "10001", "10001", "01110",
        ],
        'W' => [
            "10001", "10001", "10001", "10101", "10101", "11011", "10001",
        ],
        'Y' => [
            "10001", "10001", "01010", "00100", "00100", "00100", "00100",
        ],
        _ => ["00000"; GLYPH_HEIGHT as usize],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_diary_page() {
        let photo = render_diary_photo(&DIARY_LINES, SCALE);
        let bounds = detect_page_bounds(&photo).unwrap();
        assert!(bounds.2 > 500);
        assert!(bounds.3 > 500);
    }

    #[test]
    fn recognizes_diary_lines() {
        let photo = render_diary_photo(&DIARY_LINES, SCALE);
        let page_bounds = detect_page_bounds(&photo).unwrap();
        let inner = inset_bounds(page_bounds, 20);
        let crop = photo.view(inner.0, inner.1, inner.2, inner.3).to_image();
        let gray = DynamicImage::ImageRgb8(crop).to_luma8();
        let binary = threshold(&gray, 150);
        let line_ranges = segment_lines(&binary);
        let recognized = recognize_lines(&binary, &line_ranges);
        assert_eq!(recognized, DIARY_LINES);
    }
}
