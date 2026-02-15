use super::overlap::get_overlap_text;
use super::Chunk;

pub fn chunk_by_paragraphs(
    content: &str,
    heading_path: Option<&str>,
    base_offset: usize,
    max_chunk_size: usize,
    overlap: usize,
) -> Vec<Chunk> {
    let mut chunks = Vec::new();
    let paragraphs = split_into_paragraphs(content);

    if paragraphs.is_empty() {
        return chunks;
    }

    let mut current_chunk = String::new();
    let mut chunk_start_offset = base_offset;
    let mut current_offset = base_offset;

    for (para_text, para_offset) in paragraphs {
        // If adding this paragraph would exceed max size, save current chunk
        if !current_chunk.is_empty() && current_chunk.len() + para_text.len() > max_chunk_size {
            chunks.push(Chunk {
                heading_path: heading_path.map(|s| s.to_string()),
                text: current_chunk.clone(),
                byte_offset: chunk_start_offset,
                byte_length: current_chunk.len(),
                token_count: estimate_tokens(&current_chunk),
            });

            // Start new chunk with overlap
            let overlap_text = get_overlap_text(&current_chunk, overlap);
            current_chunk = overlap_text;
            chunk_start_offset = current_offset - current_chunk.len();
        }

        current_chunk.push_str(&para_text);
        current_offset = base_offset + para_offset + para_text.len();
    }

    // Save the last chunk
    if !current_chunk.trim().is_empty() {
        chunks.push(Chunk {
            heading_path: heading_path.map(|s| s.to_string()),
            text: current_chunk.clone(),
            byte_offset: chunk_start_offset,
            byte_length: current_chunk.len(),
            token_count: estimate_tokens(&current_chunk),
        });
    }

    chunks
}

pub fn split_into_paragraphs(content: &str) -> Vec<(String, usize)> {
    let mut paragraphs = Vec::new();
    let mut current_para = String::new();
    let mut para_start_offset = 0;
    let mut current_offset = 0;
    let mut in_trailing_blanks = false;

    for line in content.lines() {
        let line_with_newline = format!("{}\n", line);

        if line.trim().is_empty() {
            if !current_para.is_empty() {
                current_para.push_str(&line_with_newline);
                in_trailing_blanks = true;
            }
        } else {
            if current_para.is_empty() {
                para_start_offset = current_offset;
            } else if in_trailing_blanks {
                paragraphs.push((current_para.clone(), para_start_offset));
                current_para.clear();
                para_start_offset = current_offset;
                in_trailing_blanks = false;
            }

            current_para.push_str(&line_with_newline);
        }

        current_offset += line_with_newline.len();
    }

    if !current_para.is_empty() {
        paragraphs.push((current_para, para_start_offset));
    }

    paragraphs
}

fn estimate_tokens(text: &str) -> usize {
    // Simple heuristic: avg 4 chars per token
    // Also count whitespace-separated words for better accuracy
    let char_estimate = text.len() / 4;
    let word_count = text.split_whitespace().count();

    // Use average of both estimates
    (char_estimate + word_count) / 2
}

#[cfg(test)]
mod tests {
    use super::chunk_by_paragraphs;

    #[test]
    fn chunk_by_paragraphs_with_base_offset_does_not_underflow() {
        let content = "First paragraph.\n\nSecond paragraph.\n";
        let chunks = chunk_by_paragraphs(content, None, 100, 10, 5);
        assert!(chunks.len() >= 2);
        assert!(chunks.iter().all(|chunk| chunk.byte_offset >= 100));
    }

    #[test]
    fn test_split_into_paragraphs_empty() {
        let paragraphs = super::split_into_paragraphs("");
        assert!(paragraphs.is_empty());
    }

    #[test]
    fn test_split_into_paragraphs_single_para() {
        let content = "This is a single paragraph.";
        let paragraphs = super::split_into_paragraphs(content);
        assert_eq!(paragraphs.len(), 1);
    }

    #[test]
    fn test_split_into_paragraphs_multiple() {
        let content = "First paragraph.\n\nSecond paragraph.\n\nThird paragraph.";
        let paragraphs = super::split_into_paragraphs(content);
        assert_eq!(paragraphs.len(), 3);
    }

    #[test]
    fn test_split_into_paragraphs_leading_trailing_whitespace() {
        let content = "\n\nFirst paragraph.\n\nSecond paragraph.\n\n";
        let paragraphs = super::split_into_paragraphs(content);
        assert_eq!(paragraphs.len(), 2);
    }

    #[test]
    fn test_chunk_by_paragraphs_empty() {
        let chunks = chunk_by_paragraphs("", None, 0, 100, 10);
        assert!(chunks.is_empty());
    }

    #[test]
    fn test_chunk_by_paragraphs_small_para() {
        let content = "Short paragraph.";
        let chunks = chunk_by_paragraphs(content, Some("Heading"), 0, 100, 10);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].heading_path, Some("Heading".to_string()));
    }

    #[test]
    fn test_chunk_by_paragraphs_large_para() {
        // Create a paragraph larger than max_chunk_size
        let content =
            "This is a very long paragraph that should exceed the maximum chunk size. ".repeat(10);
        let chunks = chunk_by_paragraphs(content.as_str(), None, 0, 50, 10);
        // Should produce at least one chunk
        assert!(!chunks.is_empty());
    }

    #[test]
    fn test_estimate_tokens() {
        let tokens = super::estimate_tokens("Hello world test");
        // "Hello world test" = 17 chars / 4 = 4, word count = 3
        // (4 + 3) / 2 = 3
        assert!(tokens >= 3);
    }

    #[test]
    fn test_estimate_tokens_empty() {
        let tokens = super::estimate_tokens("");
        assert_eq!(tokens, 0);
    }
}
