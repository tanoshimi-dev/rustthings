use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn count_matches(text: &str, query: &str) -> usize {
    find_match_starts(text, query).len()
}

#[wasm_bindgen]
pub fn build_preview(text: &str, query: &str, radius: usize) -> String {
    if text.is_empty() {
        return String::new();
    }

    let Some(start) = find_match_starts(text, query).into_iter().next() else {
        return clipped_text(text, radius.saturating_mul(2).max(40));
    };

    let end = start + query.len();
    let snippet_start = previous_char_boundary(text, start.saturating_sub(radius));
    let snippet_end = next_char_boundary(text, (end + radius).min(text.len()));
    let mut preview = String::new();

    if snippet_start > 0 {
        preview.push_str("...");
    }

    preview.push_str(&text[snippet_start..snippet_end]);

    if snippet_end < text.len() {
        preview.push_str("...");
    }

    preview
}

fn find_match_starts(text: &str, query: &str) -> Vec<usize> {
    if query.is_empty() || query.len() > text.len() {
        return Vec::new();
    }

    let mut positions = Vec::new();
    let end = text.len() - query.len();

    for start in 0..=end {
        let finish = start + query.len();
        if !text.is_char_boundary(start) || !text.is_char_boundary(finish) {
            continue;
        }

        if text[start..finish].eq_ignore_ascii_case(query) {
            positions.push(start);
        }
    }

    positions
}

fn clipped_text(text: &str, max_len: usize) -> String {
    if text.len() <= max_len {
        return text.to_string();
    }

    let end = previous_char_boundary(text, max_len);
    format!("{}...", &text[..end])
}

fn previous_char_boundary(text: &str, mut index: usize) -> usize {
    while index > 0 && !text.is_char_boundary(index) {
        index -= 1;
    }

    index
}

fn next_char_boundary(text: &str, mut index: usize) -> usize {
    while index < text.len() && !text.is_char_boundary(index) {
        index += 1;
    }

    index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_matches_without_case_sensitivity() {
        let text = "Rust makes search fast. rust also makes previews easy.";
        assert_eq!(count_matches(text, "rust"), 2);
    }

    #[test]
    fn preview_wraps_the_matching_segment() {
        let text = "A tiny search box should show the nearest matching words to the query.";
        let preview = build_preview(text, "matching", 12);
        assert!(preview.contains("matching"));
        assert!(preview.starts_with("..."));
        assert!(preview.ends_with("..."));
    }

    #[test]
    fn preview_falls_back_to_clipped_text_when_no_match_exists() {
        let text = "This paragraph is still useful when the query does not exist.";
        let preview = build_preview(text, "wasm", 8);
        assert!(preview.starts_with("This paragraph"));
    }
}
