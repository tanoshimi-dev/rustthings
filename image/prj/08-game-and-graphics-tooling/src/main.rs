use image::{DynamicImage, ImageBuffer, Rgba, RgbaImage, imageops::FilterType};
use std::{error::Error, fs, path::PathBuf};

type AppResult<T> = Result<T, Box<dyn Error>>;

fn main() -> AppResult<()> {
    let project_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let output_dir = project_dir.join("output");
    fs::create_dir_all(&output_dir)?;

    let sprites = [
        make_sprite([220, 90, 90, 255]),
        make_sprite([90, 220, 120, 255]),
        make_sprite([90, 140, 240, 255]),
        make_sprite([220, 200, 90, 255]),
    ];

    for (index, sprite) in sprites.iter().enumerate() {
        DynamicImage::ImageRgba8(sprite.clone())
            .save(output_dir.join(format!("sprite_{}.png", index + 1)))?;
    }

    let sheet = sprite_sheet(&sprites, 2, 2, 32, 32);
    DynamicImage::ImageRgba8(sheet.clone()).save(output_dir.join("sprite_sheet.png"))?;
    DynamicImage::ImageRgba8(sheet)
        .resize_exact(256, 256, FilterType::Nearest)
        .save(output_dir.join("sprite_sheet_preview.png"))?;

    println!("Created sprite assets in {}", output_dir.display());
    Ok(())
}

fn make_sprite(color: [u8; 4]) -> RgbaImage {
    ImageBuffer::from_fn(32, 32, |x, y| {
        let border = x < 2 || y < 2 || x >= 30 || y >= 30;
        let eye = (x == 10 || x == 21) && (y >= 10 && y <= 12);
        let mouth = y == 22 && (x >= 10 && x <= 21);

        if border {
            Rgba([20, 20, 30, 255])
        } else if eye || mouth {
            Rgba([250, 250, 250, 255])
        } else {
            Rgba(color)
        }
    })
}

fn sprite_sheet(
    sprites: &[RgbaImage],
    columns: u32,
    rows: u32,
    sprite_width: u32,
    sprite_height: u32,
) -> RgbaImage {
    let mut sheet = RgbaImage::new(columns * sprite_width, rows * sprite_height);

    for (index, sprite) in sprites.iter().enumerate() {
        let x_offset = (index as u32 % columns) * sprite_width;
        let y_offset = (index as u32 / columns) * sprite_height;

        for y in 0..sprite_height {
            for x in 0..sprite_width {
                let pixel = sprite.get_pixel(x, y);
                sheet.put_pixel(x_offset + x, y_offset + y, *pixel);
            }
        }
    }

    sheet
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sprite_sheet_has_expected_size() {
        let sprite = make_sprite([1, 2, 3, 255]);
        let sheet = sprite_sheet(
            &[sprite.clone(), sprite.clone(), sprite.clone(), sprite],
            2,
            2,
            32,
            32,
        );
        assert_eq!(sheet.dimensions(), (64, 64));
    }
}
