use image::{
    DynamicImage, GenericImageView, GrayImage, ImageBuffer, Luma, Rgb, RgbImage,
    imageops::FilterType,
};
use std::{error::Error, fs, path::PathBuf};

type AppResult<T> = Result<T, Box<dyn Error>>;

const GLYPH_WIDTH: u32 = 5;
const GLYPH_HEIGHT: u32 = 7;
const SCALE: u32 = 10;
const RECEIPT_LINES: [&str; 4] = ["MART", "APPLE 120", "MILK 250", "TOTAL 370"];

fn main() -> AppResult<()> {
    let project_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let output_dir = project_dir.join("output");
    fs::create_dir_all(&output_dir)?;

    let source = render_receipt_scene(&RECEIPT_LINES, SCALE);
    let receipt_bounds = detect_receipt_bounds(&source, 238).ok_or("Receipt bounds not found")?;
    let receipt_crop = source
        .view(
            receipt_bounds.0,
            receipt_bounds.1,
            receipt_bounds.2,
            receipt_bounds.3,
        )
        .to_image();
    let binary = threshold(&receipt_crop, 170);
    let line_ranges = segment_lines(&binary);
    let recognized_lines = recognize_receipt_lines(&binary, &line_ranges);
    let annotated = draw_line_boxes(&receipt_crop, &line_ranges);

    DynamicImage::ImageLuma8(source.clone()).save(output_dir.join("01_source_receipt.png"))?;
    DynamicImage::ImageLuma8(receipt_crop.clone()).save(output_dir.join("02_receipt_crop.png"))?;
    DynamicImage::ImageLuma8(binary.clone()).save(output_dir.join("03_binary_receipt.png"))?;
    DynamicImage::ImageRgb8(annotated).save(output_dir.join("04_detected_lines.png"))?;

    for (index, (top, bottom)) in line_ranges.iter().enumerate() {
        let line_image = extract_line_image(&binary, *top, *bottom);
        DynamicImage::ImageLuma8(line_image)
            .save(output_dir.join(format!("05_line_{:02}.png", index + 1)))?;
    }

    let recognized_text = recognized_lines.join("\n");
    fs::write(
        output_dir.join("recognized.txt"),
        format!("{recognized_text}\n"),
    )?;

    println!("Recognized receipt text:");
    for line in &recognized_lines {
        println!("{line}");
    }
    println!("Outputs written to {}", output_dir.display());
    Ok(())
}

fn render_receipt_scene(lines: &[&str], scale: u32) -> GrayImage {
    let scene_width = 900;
    let scene_height = 700;
    let paper_x = 150;
    let paper_y = 80;
    let paper_width = 560;
    let paper_height = 500;
    let mut image = GrayImage::from_pixel(scene_width, scene_height, Luma([198]));

    for y in paper_y..paper_y + paper_height {
        for x in paper_x..paper_x + paper_width {
            let shadow = if x > paper_x + paper_width - 8 || y > paper_y + paper_height - 8 {
                232
            } else {
                245
            };
            image.put_pixel(x, y, Luma([shadow]));
        }
    }

    let mut y_offset = paper_y + 45;
    for line in lines {
        draw_text(&mut image, paper_x + 40, y_offset, line, scale);
        y_offset += GLYPH_HEIGHT * scale + scale * 2;
    }

    add_receipt_noise(&mut image);
    image
}

fn draw_text(image: &mut GrayImage, x_offset: u32, y_offset: u32, text: &str, scale: u32) {
    let mut cursor = x_offset;
    for ch in text.chars() {
        if ch == ' ' {
            cursor += scale * 3;
        } else {
            draw_glyph(image, cursor, y_offset, ch, scale);
            cursor += GLYPH_WIDTH * scale + scale;
        }
    }
}

fn draw_glyph(image: &mut GrayImage, x_offset: u32, y_offset: u32, ch: char, scale: u32) {
    let glyph = glyph_pattern(ch);
    for (row, line) in glyph.iter().enumerate() {
        for (col, cell) in line.chars().enumerate() {
            let intensity = if cell == '1' { 15 } else { 245 };
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

fn add_receipt_noise(image: &mut GrayImage) {
    for x in (0..image.width()).step_by(29) {
        for y in (0..image.height()).step_by(23) {
            let base = image.get_pixel(x, y)[0];
            let adjusted = base.saturating_sub(8);
            image.put_pixel(x, y, Luma([adjusted]));
        }
    }
}

fn detect_receipt_bounds(image: &GrayImage, paper_cutoff: u8) -> Option<(u32, u32, u32, u32)> {
    bounding_box(image, |pixel| pixel[0] >= paper_cutoff)
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

    for y in 0..image.height() {
        let has_ink = (0..image.width()).any(|x| image.get_pixel(x, y)[0] == 0);
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

fn recognize_receipt_lines(image: &GrayImage, line_ranges: &[(u32, u32)]) -> Vec<String> {
    line_ranges
        .iter()
        .map(|(top, bottom)| {
            let line_image = extract_line_image(image, *top, *bottom);
            recognize_line(&line_image)
        })
        .collect()
}

fn recognize_line(image: &GrayImage) -> String {
    let segments = segment_characters(image);
    let templates = templates();
    let mut recognized = String::new();
    let mut last_end = None;

    for (start, end) in segments {
        if let Some(previous_end) = last_end {
            if start.saturating_sub(previous_end) > SCALE * 3 {
                recognized.push(' ');
            }
        }

        let cropped = extract_character_image(image, start, end);
        let normalized = DynamicImage::ImageLuma8(cropped)
            .resize_exact(GLYPH_WIDTH, GLYPH_HEIGHT, FilterType::Nearest)
            .to_luma8();
        let normalized = threshold(&normalized, 128);

        let best = templates
            .iter()
            .min_by_key(|(_, template)| pixel_distance(&normalized, template))
            .map(|(ch, _)| *ch)
            .unwrap_or('?');

        recognized.push(best);
        last_end = Some(end);
    }

    recognized
}

fn segment_characters(image: &GrayImage) -> Vec<(u32, u32)> {
    let mut segments = Vec::new();
    let mut start = None;

    for x in 0..image.width() {
        let has_ink = (0..image.height()).any(|y| image.get_pixel(x, y)[0] == 0);
        match (has_ink, start) {
            (true, None) => start = Some(x),
            (false, Some(left)) => {
                segments.push((left, x - 1));
                start = None;
            }
            _ => {}
        }
    }

    if let Some(left) = start {
        segments.push((left, image.width() - 1));
    }

    segments
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

fn extract_character_image(image: &GrayImage, start: u32, end: u32) -> GrayImage {
    let cropped = image
        .view(start, 0, end - start + 1, image.height())
        .to_image();
    if let Some((left, top, width, height)) = bounding_box(&cropped, |pixel| pixel[0] == 0) {
        cropped.view(left, top, width, height).to_image()
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

fn draw_line_boxes(image: &GrayImage, lines: &[(u32, u32)]) -> RgbImage {
    let mut rgb = DynamicImage::ImageLuma8(image.clone()).to_rgb8();
    for (top, bottom) in lines {
        for x in 0..rgb.width() {
            rgb.put_pixel(x, *top, Rgb([220, 30, 30]));
            rgb.put_pixel(x, *bottom, Rgb([220, 30, 30]));
        }
        for y in *top..=*bottom {
            rgb.put_pixel(0, y, Rgb([220, 30, 30]));
            rgb.put_pixel(rgb.width() - 1, y, Rgb([220, 30, 30]));
        }
    }
    rgb
}

fn templates() -> Vec<(char, GrayImage)> {
    [
        'A', 'E', 'I', 'K', 'L', 'M', 'O', 'P', 'R', 'T', '0', '1', '2', '3', '5', '7',
    ]
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
        'A' => [
            "01110", "10001", "10001", "11111", "10001", "10001", "10001",
        ],
        'E' => [
            "11111", "10000", "10000", "11110", "10000", "10000", "11111",
        ],
        'I' => [
            "11111", "00100", "00100", "00100", "00100", "00100", "11111",
        ],
        'K' => [
            "10001", "10010", "10100", "11000", "10100", "10010", "10001",
        ],
        'L' => [
            "10000", "10000", "10000", "10000", "10000", "10000", "11111",
        ],
        'M' => [
            "10001", "11011", "10101", "10101", "10001", "10001", "10001",
        ],
        'O' => [
            "01110", "10001", "10001", "10001", "10001", "10001", "01110",
        ],
        'P' => [
            "11110", "10001", "10001", "11110", "10000", "10000", "10000",
        ],
        'R' => [
            "11110", "10001", "10001", "11110", "10100", "10010", "10001",
        ],
        'T' => [
            "11111", "00100", "00100", "00100", "00100", "00100", "00100",
        ],
        '0' => [
            "01110", "10001", "10011", "10101", "11001", "10001", "01110",
        ],
        '1' => [
            "00100", "01100", "00100", "00100", "00100", "00100", "01110",
        ],
        '2' => [
            "01110", "10001", "00001", "00010", "00100", "01000", "11111",
        ],
        '3' => [
            "11110", "00001", "00001", "01110", "00001", "00001", "11110",
        ],
        '5' => [
            "11111", "10000", "10000", "11110", "00001", "00001", "11110",
        ],
        '7' => [
            "11111", "00001", "00010", "00100", "01000", "01000", "01000",
        ],
        _ => ["00000"; GLYPH_HEIGHT as usize],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_receipt_bounds() {
        let receipt = render_receipt_scene(&RECEIPT_LINES, SCALE);
        let bounds = detect_receipt_bounds(&receipt, 238).unwrap();
        assert!(bounds.2 > 400);
        assert!(bounds.3 > 300);
    }

    #[test]
    fn recognizes_receipt_lines() {
        let source = render_receipt_scene(&RECEIPT_LINES, SCALE);
        let bounds = detect_receipt_bounds(&source, 238).unwrap();
        let crop = source
            .view(bounds.0, bounds.1, bounds.2, bounds.3)
            .to_image();
        let binary = threshold(&crop, 170);
        let line_ranges = segment_lines(&binary);
        let recognized = recognize_receipt_lines(&binary, &line_ranges);
        assert_eq!(recognized, RECEIPT_LINES);
    }
}
