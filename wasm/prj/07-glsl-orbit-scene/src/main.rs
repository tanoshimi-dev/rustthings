use glsl_orbit_scene::{describe_theme, fragment_shader_source, scene_state};

fn main() {
    let payload = scene_state(0.0, "synthwave", 1.0).unwrap();

    println!("GLSL orbit wasm demo");
    println!("{}", describe_theme("synthwave"));
    println!("State payload: {payload}");
    println!(
        "Fragment shader size: {} characters",
        fragment_shader_source().len()
    );
    println!("Open web/index.html through a local server after building with wasm-pack.");
}
