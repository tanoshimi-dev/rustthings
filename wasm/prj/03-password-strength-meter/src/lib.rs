#[cfg(feature = "wasm-export")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm-export", wasm_bindgen)]
pub fn score_password(password: &str) -> u8 {
    let mut score = 0_i32;

    score += match password.chars().count() {
        0..=7 => 0,
        8..=9 => 8,
        10..=11 => 16,
        12..=15 => 25,
        _ => 35,
    };

    let has_lower = password.chars().any(|ch| ch.is_ascii_lowercase());
    let has_upper = password.chars().any(|ch| ch.is_ascii_uppercase());
    let has_digit = password.chars().any(|ch| ch.is_ascii_digit());
    let has_symbol = password.chars().any(|ch| !ch.is_ascii_alphanumeric());
    let classes = [has_lower, has_upper, has_digit, has_symbol]
        .into_iter()
        .filter(|present| *present)
        .count();

    score += (classes as i32) * 10;

    if classes >= 3 {
        score += 10;
    }

    if classes == 4 {
        score += 5;
    }

    if password.chars().count() >= 14 && classes == 4 {
        score += 10;
    }

    let lowered = password.to_ascii_lowercase();
    for weak_pattern in ["password", "1234", "qwerty", "admin", "letmein"] {
        if lowered.contains(weak_pattern) {
            score -= 40;
            break;
        }
    }

    if has_repeated_run(password, 3) {
        score -= 15;
    }

    score.clamp(0, 100) as u8
}

#[cfg_attr(feature = "wasm-export", wasm_bindgen)]
pub fn strength_label(password: &str) -> String {
    match score_password(password) {
        0..=39 => "weak",
        40..=69 => "medium",
        70..=89 => "strong",
        _ => "very strong",
    }
    .to_string()
}

#[cfg_attr(feature = "wasm-export", wasm_bindgen)]
pub fn password_feedback(password: &str) -> String {
    let mut notes = Vec::new();

    if password.chars().count() < 12 {
        notes.push("use at least 12 characters");
    }
    if !password.chars().any(|ch| ch.is_ascii_uppercase()) {
        notes.push("add an uppercase letter");
    }
    if !password.chars().any(|ch| ch.is_ascii_lowercase()) {
        notes.push("add a lowercase letter");
    }
    if !password.chars().any(|ch| ch.is_ascii_digit()) {
        notes.push("add a digit");
    }
    if !password.chars().any(|ch| !ch.is_ascii_alphanumeric()) {
        notes.push("add a symbol");
    }
    if has_repeated_run(password, 3) {
        notes.push("avoid repeated character runs");
    }

    if notes.is_empty() {
        "ready to use in a signup form".to_string()
    } else {
        notes.join(", ")
    }
}

fn has_repeated_run(password: &str, threshold: usize) -> bool {
    let mut previous = None;
    let mut run = 0;

    for ch in password.chars() {
        if previous == Some(ch) {
            run += 1;
        } else {
            previous = Some(ch);
            run = 1;
        }

        if run >= threshold {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strong_password_scores_higher_than_common_pattern() {
        assert!(score_password("B3tter#BrowserWasm!") > score_password("password1234"));
    }

    #[test]
    fn repeated_runs_trigger_feedback() {
        assert!(password_feedback("AAAbbb111").contains("avoid repeated character runs"));
    }

    #[test]
    fn strong_password_gets_positive_feedback() {
        assert_eq!(
            password_feedback("WasmReady#2026"),
            "ready to use in a signup form"
        );
        assert_eq!(strength_label("WasmReady#2026"), "very strong");
    }
}
