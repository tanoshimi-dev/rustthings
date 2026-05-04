use std::f64::consts::{PI, TAU};

use wasm_bindgen::{JsValue, prelude::*};

const MIN_DIMENSION: f64 = 64.0;
const STAR_COUNT: usize = 48;
const ORB_COUNT: usize = 10;
const GRID_BANDS: usize = 9;
const GRID_COLUMNS: i32 = 7;
const RIDGE_POINTS: usize = 22;

#[wasm_bindgen]
pub fn render_scene_svg(
    width: f64,
    height: f64,
    time: f64,
    theme: &str,
    motion: f64,
) -> Result<String, JsValue> {
    render_scene_svg_text(width, height, time, theme, motion)
        .map_err(|error| JsValue::from_str(&error))
}

fn render_scene_svg_text(
    width: f64,
    height: f64,
    time: f64,
    theme: &str,
    motion: f64,
) -> Result<String, String> {
    if width < MIN_DIMENSION || height < MIN_DIMENSION {
        return Err("width and height must both be at least 64".to_string());
    }

    if !(0.0..=2.0).contains(&motion) {
        return Err("motion must be between 0.0 and 2.0".to_string());
    }

    let palette = ScenePalette::from_name(theme)
        .ok_or_else(|| "theme must be one of: sunset, aurora, candy".to_string())?;

    let horizon = height * 0.58;
    let sun_radius = height * (0.12 + (time * 0.6).sin().abs() * 0.01 * motion);
    let sun_y = horizon - (height * 0.14) + (time * 0.35).sin() * height * 0.01 * motion;
    let lake_top = horizon;
    let foreground_base = height * 0.79;

    let mut svg = String::new();
    svg.push_str(&format!(
        r#"<svg viewBox="0 0 {} {}" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="Animated neon sunset scene">"#,
        trim_float(width),
        trim_float(height)
    ));
    svg.push_str(&build_defs(width, height, horizon, palette));
    svg.push_str(r#"<rect width="100%" height="100%" fill="url(#sky-gradient)" />"#);
    svg.push_str(&build_orbs(width, horizon, time, motion, palette));
    svg.push_str(&build_stars(width, horizon, time, palette));
    svg.push_str(&build_sun(width / 2.0, sun_y, sun_radius, palette));
    svg.push_str(&build_mountains(
        width,
        height,
        horizon + 6.0,
        horizon + height * 0.12,
        height * 0.12,
        time,
        motion,
        palette.mountain_back,
        0.8,
        0.35,
    ));
    svg.push_str(&build_mountains(
        width,
        height,
        horizon + 18.0,
        foreground_base,
        height * 0.18,
        time,
        motion,
        palette.mountain_front,
        1.3,
        1.25,
    ));
    svg.push_str(&format!(
        r#"<rect x="0" y="{}" width="{}" height="{}" fill="url(#lake-gradient)" />"#,
        trim_float(lake_top),
        trim_float(width),
        trim_float(height - lake_top)
    ));
    svg.push_str(&build_reflection(
        width, height, lake_top, time, motion, palette,
    ));
    svg.push_str(&build_grid(width, height, lake_top, palette, motion));
    svg.push_str("</svg>");

    Ok(svg)
}

#[wasm_bindgen]
pub fn describe_theme(theme: &str) -> String {
    ScenePalette::from_name(theme)
        .map(|palette| palette.description.to_string())
        .unwrap_or_else(|| "Unknown theme. Try sunset, aurora, or candy.".to_string())
}

fn build_defs(width: f64, height: f64, horizon: f64, palette: &ScenePalette) -> String {
    format!(
        r##"
<defs>
  <linearGradient id="sky-gradient" x1="0%" y1="0%" x2="0%" y2="100%">
    <stop offset="0%" stop-color="{sky_top}" />
    <stop offset="52%" stop-color="{sky_mid}" />
    <stop offset="100%" stop-color="{sky_bottom}" />
  </linearGradient>
  <linearGradient id="lake-gradient" x1="0%" y1="0%" x2="0%" y2="100%">
    <stop offset="0%" stop-color="{lake_top}" stop-opacity="0.92" />
    <stop offset="100%" stop-color="{lake_bottom}" stop-opacity="1" />
  </linearGradient>
  <radialGradient id="sun-gradient" cx="50%" cy="45%" r="65%">
    <stop offset="0%" stop-color="{sun_core}" />
    <stop offset="100%" stop-color="{sun_edge}" />
  </radialGradient>
  <clipPath id="sun-clip">
    <circle cx="{center_x}" cy="{sun_y}" r="{sun_radius}" />
  </clipPath>
</defs>
"##,
        sky_top = palette.sky_top,
        sky_mid = palette.sky_mid,
        sky_bottom = palette.sky_bottom,
        lake_top = palette.lake_top,
        lake_bottom = palette.lake_bottom,
        sun_core = palette.sun_core,
        sun_edge = palette.sun_edge,
        center_x = trim_float(width / 2.0),
        sun_y = trim_float(horizon - (height * 0.14)),
        sun_radius = trim_float(height * 0.12),
    )
}

fn build_stars(width: f64, horizon: f64, time: f64, palette: &ScenePalette) -> String {
    let mut stars = String::new();
    stars.push_str(r#"<g opacity="0.95">"#);

    for index in 0..STAR_COUNT {
        let index = index as f64;
        let x = hash(index, 0.37) * width;
        let y = hash(index, 1.91) * horizon * 0.72;
        let radius = 0.7 + hash(index, 2.73) * 2.2;
        let twinkle = 0.2
            + 0.8
                * (((time * (0.8 + hash(index, 3.61))) + (hash(index, 4.27) * TAU)).sin() * 0.5
                    + 0.5);

        stars.push_str(&format!(
            r#"<circle cx="{x}" cy="{y}" r="{radius}" fill="{fill}" opacity="{opacity}" />"#,
            x = trim_float(x),
            y = trim_float(y),
            radius = trim_float(radius),
            fill = palette.star,
            opacity = trim_float(twinkle)
        ));
    }

    stars.push_str("</g>");
    stars
}

fn build_orbs(width: f64, horizon: f64, time: f64, motion: f64, palette: &ScenePalette) -> String {
    let mut orbs = String::new();
    orbs.push_str(r#"<g opacity="0.35">"#);

    for index in 0..ORB_COUNT {
        let index = index as f64;
        let base_x = hash(index, 5.11) * width;
        let base_y = hash(index, 6.23) * horizon * 0.8;
        let radius = 18.0 + hash(index, 7.31) * 42.0;
        let drift_x = (time * (0.2 + hash(index, 8.03)) + index).sin() * 18.0 * motion;
        let drift_y = (time * (0.14 + hash(index, 8.97)) + index * 0.4).cos() * 10.0 * motion;

        orbs.push_str(&format!(
            r#"<circle cx="{x}" cy="{y}" r="{radius}" fill="{fill}" opacity="{opacity}" />"#,
            x = trim_float(base_x + drift_x),
            y = trim_float(base_y + drift_y),
            radius = trim_float(radius),
            fill = palette.orb,
            opacity = trim_float(0.12 + hash(index, 9.41) * 0.2)
        ));
    }

    orbs.push_str("</g>");
    orbs
}

fn build_sun(cx: f64, cy: f64, radius: f64, palette: &ScenePalette) -> String {
    let mut sun = String::new();
    sun.push_str(&format!(
        r#"<circle cx="{cx}" cy="{cy}" r="{radius}" fill="url(#sun-gradient)" />"#,
        cx = trim_float(cx),
        cy = trim_float(cy),
        radius = trim_float(radius)
    ));
    sun.push_str(r#"<g clip-path="url(#sun-clip)">"#);

    let stripe_count = 7;
    let stripe_height = radius * 0.18;
    for stripe in 0..stripe_count {
        let y = cy - radius + (stripe as f64 * stripe_height * 1.18);
        let height = stripe_height * (0.28 + stripe as f64 * 0.08);
        sun.push_str(&format!(
            r#"<rect x="{x}" y="{y}" width="{width}" height="{height}" fill="{fill}" opacity="0.22" />"#,
            x = trim_float(cx - radius),
            y = trim_float(y),
            width = trim_float(radius * 2.0),
            height = trim_float(height),
            fill = palette.sun_stripe
        ));
    }

    sun.push_str("</g>");
    sun
}

#[allow(clippy::too_many_arguments)]
fn build_mountains(
    width: f64,
    height: f64,
    horizon: f64,
    base_y: f64,
    amplitude: f64,
    time: f64,
    motion: f64,
    fill: &str,
    frequency: f64,
    seed: f64,
) -> String {
    let mut points = Vec::with_capacity(RIDGE_POINTS + 3);
    points.push(format!("0,{}", trim_float(height)));

    for step in 0..=RIDGE_POINTS {
        let progress = step as f64 / RIDGE_POINTS as f64;
        let x = width * progress;
        let wave = ridge_wave(progress, time, motion, frequency, seed);
        let y = base_y - amplitude * wave;
        points.push(format!("{},{}", trim_float(x), trim_float(y.max(horizon))));
    }

    points.push(format!("{},{}", trim_float(width), trim_float(height)));

    format!(
        r#"<polygon points="{points}" fill="{fill}" />"#,
        points = points.join(" "),
        fill = fill
    )
}

fn build_reflection(
    width: f64,
    height: f64,
    lake_top: f64,
    time: f64,
    motion: f64,
    palette: &ScenePalette,
) -> String {
    let mut reflection = String::new();
    reflection.push_str(r#"<g fill="none">"#);

    for band in 0..7 {
        let band = band as f64;
        let y = lake_top + 28.0 + band * ((height - lake_top) / 8.5);
        let amplitude = 5.0 + band * 2.2;
        let width_factor = 0.22 + band * 0.08;
        let left = width * (0.5 - width_factor);
        let right = width * (0.5 + width_factor);

        let mut points = Vec::with_capacity(18);
        for step in 0..=17 {
            let progress = step as f64 / 17.0;
            let x = left + (right - left) * progress;
            let wave = ((progress * PI * 4.0) + time * (1.1 + band * 0.15)).sin();
            let shimmer = ((progress * PI * 9.0) + time * 0.7 + band).cos();
            let dy = wave * amplitude * 0.45 * motion + shimmer * 1.4;
            points.push(format!("{},{}", trim_float(x), trim_float(y + dy)));
        }

        reflection.push_str(&format!(
            r#"<polyline points="{points}" stroke="{stroke}" stroke-width="{stroke_width}" stroke-linecap="round" opacity="{opacity}" />"#,
            points = points.join(" "),
            stroke = palette.reflection,
            stroke_width = trim_float(1.2 + band * 0.25),
            opacity = trim_float(0.22 + band * 0.08)
        ));
    }

    reflection.push_str("</g>");
    reflection
}

fn build_grid(
    width: f64,
    height: f64,
    horizon: f64,
    palette: &ScenePalette,
    motion: f64,
) -> String {
    let mut grid = String::new();
    grid.push_str(&format!(
        r#"<g stroke="{stroke}" stroke-linecap="round" opacity="0.78">"#,
        stroke = palette.grid
    ));

    for band in 0..GRID_BANDS {
        let progress = (band + 1) as f64 / GRID_BANDS as f64;
        let curve = progress * progress;
        let y = horizon + curve * (height - horizon);
        grid.push_str(&format!(
            r#"<line x1="0" y1="{y}" x2="{x2}" y2="{y}" stroke-width="{stroke_width}" opacity="{opacity}" />"#,
            y = trim_float(y),
            x2 = trim_float(width),
            stroke_width = trim_float(1.0 + progress * 1.8),
            opacity = trim_float(0.22 + progress * 0.58)
        ));
    }

    for column in -GRID_COLUMNS..=GRID_COLUMNS {
        let distance = column as f64 / GRID_COLUMNS as f64;
        let top_x = width / 2.0 + distance * width * (0.045 + motion * 0.005);
        let bottom_x = width / 2.0 + distance * width * 0.48;
        grid.push_str(&format!(
            r#"<line x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}" stroke-width="1.25" opacity="{opacity}" />"#,
            x1 = trim_float(top_x),
            y1 = trim_float(horizon),
            x2 = trim_float(bottom_x),
            y2 = trim_float(height),
            opacity = trim_float(0.3 + (1.0 - distance.abs()) * 0.45)
        ));
    }

    grid.push_str("</g>");
    grid
}

fn ridge_wave(progress: f64, time: f64, motion: f64, frequency: f64, seed: f64) -> f64 {
    let base = ((progress * PI * frequency * 2.2) + seed).sin().abs();
    let detail = ((progress * PI * frequency * 5.4) + seed * 1.7).cos().abs() * 0.45;
    let drift = ((progress * PI * frequency * 2.8) + time * 0.22 + seed)
        .sin()
        .abs()
        * 0.16
        * motion;
    0.28 + base * 0.72 + detail + drift
}

fn hash(value: f64, salt: f64) -> f64 {
    let mixed = (value * 12.9898 + salt * 78.233).sin() * 43_758.545_312_3;
    fract(mixed.abs())
}

fn fract(value: f64) -> f64 {
    value - value.floor()
}

fn trim_float(value: f64) -> String {
    let formatted = format!("{value:.2}");
    formatted
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string()
}

struct ScenePalette {
    sky_top: &'static str,
    sky_mid: &'static str,
    sky_bottom: &'static str,
    sun_core: &'static str,
    sun_edge: &'static str,
    sun_stripe: &'static str,
    mountain_back: &'static str,
    mountain_front: &'static str,
    lake_top: &'static str,
    lake_bottom: &'static str,
    reflection: &'static str,
    grid: &'static str,
    star: &'static str,
    orb: &'static str,
    description: &'static str,
}

impl ScenePalette {
    fn from_name(name: &str) -> Option<&'static Self> {
        match name.trim().to_ascii_lowercase().as_str() {
            "sunset" => Some(&SUNSET),
            "aurora" => Some(&AURORA),
            "candy" => Some(&CANDY),
            _ => None,
        }
    }
}

const SUNSET: ScenePalette = ScenePalette {
    sky_top: "#120524",
    sky_mid: "#51127c",
    sky_bottom: "#f97393",
    sun_core: "#fff3a3",
    sun_edge: "#ff8a5b",
    sun_stripe: "#66103b",
    mountain_back: "#28104d",
    mountain_front: "#140824",
    lake_top: "#35125e",
    lake_bottom: "#090312",
    reflection: "#ffd36e",
    grid: "#ff4dba",
    star: "#fff4d7",
    orb: "#ff7bc5",
    description: "Sunset mixes warm glow, magenta mountains, and a classic synthwave grid.",
};

const AURORA: ScenePalette = ScenePalette {
    sky_top: "#02111f",
    sky_mid: "#0d355d",
    sky_bottom: "#25b3a4",
    sun_core: "#d7fff7",
    sun_edge: "#5eead4",
    sun_stripe: "#0b2740",
    mountain_back: "#12374a",
    mountain_front: "#071c26",
    lake_top: "#0a3145",
    lake_bottom: "#020b10",
    reflection: "#8fffe2",
    grid: "#7cfcff",
    star: "#f4fffe",
    orb: "#61f9d4",
    description: "Aurora shifts the scene cooler with teal light, deep water, and crisp cyan lines.",
};

const CANDY: ScenePalette = ScenePalette {
    sky_top: "#24003d",
    sky_mid: "#7d2ae8",
    sky_bottom: "#ff7ac6",
    sun_core: "#fff1fb",
    sun_edge: "#ffb86b",
    sun_stripe: "#74246c",
    mountain_back: "#4a156e",
    mountain_front: "#240433",
    lake_top: "#5a2282",
    lake_bottom: "#12011a",
    reflection: "#ffd8fb",
    grid: "#ff9ef3",
    star: "#fff5ff",
    orb: "#ffc2eb",
    description: "Candy pushes the palette into dreamy pinks and violets for a softer arcade mood.",
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scene_svg_contains_expected_layers() {
        let svg = render_scene_svg_text(960.0, 540.0, 1.2, "sunset", 1.0).unwrap();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("sky-gradient"));
        assert!(svg.contains("lake-gradient"));
        assert!(svg.contains("Animated neon sunset scene"));
    }

    #[test]
    fn invalid_theme_returns_error() {
        let error = render_scene_svg_text(960.0, 540.0, 0.0, "forest", 1.0).unwrap_err();
        assert_eq!(error, "theme must be one of: sunset, aurora, candy");
    }

    #[test]
    fn theme_description_is_available() {
        let description = describe_theme("aurora");
        assert!(description.contains("teal"));
    }
}
