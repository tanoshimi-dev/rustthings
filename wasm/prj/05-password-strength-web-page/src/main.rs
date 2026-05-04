use password_strength_web_page::password_summary;

fn main() {
    let password = "WasmReady#2026";

    println!("Password strength demo");
    println!("{}", password_summary(password));
    println!("Open web/index.html through a local server after building with wasm-pack.");
}
