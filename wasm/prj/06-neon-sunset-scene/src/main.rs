use neon_sunset_scene::{describe_theme, render_scene_svg};

fn main() {
    let svg = render_scene_svg(960.0, 540.0, 0.0, "sunset", 1.0).unwrap();

    println!("Neon sunset wasm demo");
    println!("{}", describe_theme("sunset"));
    println!("Generated scene SVG: {} characters", svg.len());
    println!("Open web/index.html through a local server after building with wasm-pack.");
}
