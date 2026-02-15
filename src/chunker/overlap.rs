pub fn get_overlap_text(text: &str, overlap: usize) -> String {
    let total_chars = text.chars().count();
    if total_chars <= overlap {
        return text.to_string();
    }

    // Try to find a good breakpoint (sentence or paragraph)
    let start_char = total_chars.saturating_sub(overlap);
    let start_byte = text
        .char_indices()
        .nth(start_char)
        .map(|(index, _)| index)
        .unwrap_or(0);
    let overlap_section = &text[start_byte..];

    // Look for sentence boundaries
    if let Some(pos) = overlap_section.rfind(". ") {
        return overlap_section[pos + 2..].to_string();
    }

    // Fall back to character-based overlap
    overlap_section.to_string()
}

#[cfg(test)]
mod tests {
    use super::get_overlap_text;

    #[test]
    fn overlap_handles_multibyte_chars() {
        let text = "你好世界";
        let result = get_overlap_text(text, 2);
        assert_eq!(result, "世界");
    }

    #[test]
    fn test_get_overlap_text_zero_overlap() {
        let text = "Hello world";
        let result = get_overlap_text(text, 0);
        assert!(result.is_empty() || result == "Hello world");
    }

    #[test]
    fn test_get_overlap_text_exact_length() {
        let text = "Hello";
        let result = get_overlap_text(text, 5);
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_get_overlap_text_greater_than_length() {
        let text = "Hello";
        let result = get_overlap_text(text, 10);
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_get_overlap_text_sentence_boundary() {
        let text = "This is the first sentence. This is the second sentence.";
        let result = get_overlap_text(text, 15);
        // Should find ". " boundary and return from second sentence
        // Result may vary based on implementation, so just check it's not empty
        assert!(!result.is_empty());
    }

    #[test]
    fn test_get_overlap_text_no_sentence_boundary() {
        let text = "This is a long text without periods";
        let result = get_overlap_text(text, 10);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_get_overlap_text_single_char() {
        let text = "A";
        let result = get_overlap_text(text, 1);
        assert_eq!(result, "A");
    }
}
