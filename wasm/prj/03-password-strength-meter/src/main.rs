use password_strength_meter::{password_feedback, score_password, strength_label};

fn main() {
    let password = "WasmReady#2026";

    println!("password: {password}");
    println!("score: {}", score_password(password));
    println!("label: {}", strength_label(password));
    println!("feedback: {}", password_feedback(password));
}
