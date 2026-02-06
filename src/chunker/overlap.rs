pub fn get_overlap_text(text: &str, overlap: usize) -> String {
    if text.len() <= overlap {
        return text.to_string();
    }

    // Try to find a good breakpoint (sentence or paragraph)
    let start_pos = text.len().saturating_sub(overlap);
    let overlap_section = &text[start_pos..];

    // Look for sentence boundaries
    if let Some(pos) = overlap_section.rfind(". ") {
        return overlap_section[pos + 2..].to_string();
    }

    // Fall back to character-based overlap
    text[text.len() - overlap..].to_string()
}
