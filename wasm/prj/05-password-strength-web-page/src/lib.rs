use password_strength_meter::{
    password_feedback as core_password_feedback, score_password as core_score_password,
    strength_label as core_strength_label,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn score_password(password: &str) -> u8 {
    core_score_password(password)
}

#[wasm_bindgen]
pub fn strength_label(password: &str) -> String {
    core_strength_label(password)
}

#[wasm_bindgen]
pub fn password_feedback(password: &str) -> String {
    core_password_feedback(password)
}

#[wasm_bindgen]
pub fn password_summary(password: &str) -> String {
    format!(
        "score: {}\nlabel: {}\nfeedback: {}",
        score_password(password),
        strength_label(password),
        password_feedback(password),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summary_includes_score_and_label() {
        let summary = password_summary("WasmReady#2026");
        assert!(summary.contains("score:"));
        assert!(summary.contains("label: very strong"));
    }

    #[test]
    fn wrappers_forward_the_core_logic() {
        let password = "password1234";
        assert_eq!(score_password(password), core_score_password(password));
        assert_eq!(strength_label(password), core_strength_label(password));
    }
}
