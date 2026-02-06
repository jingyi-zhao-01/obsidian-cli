/// Chunking module for splitting markdown notes into retrieval-ready text units
/// 
/// This module implements heading-based chunking with paragraph fallback,
/// preserving document structure and maintaining stable heading paths.

mod chunk;
mod heading;
mod paragraph;
mod overlap;

pub use chunk::Chunk;

#[derive(Debug, Clone)]
struct HeadingInfo {
    level: usize,
    text: String,
    byte_offset: usize,
}

pub struct MarkdownChunker {
    max_chunk_size: usize, // in characters (approximate)
    overlap: usize,        // overlap between chunks in characters
}

impl Default for MarkdownChunker {
    fn default() -> Self {
        Self::new(1000, 100)
    }
}

impl MarkdownChunker {
    pub fn new(max_chunk_size: usize, overlap: usize) -> Self {
        MarkdownChunker {
            max_chunk_size,
            overlap,
        }
    }

    /// Split a markdown document into chunks
    pub fn chunk(&self, content: &str) -> Vec<Chunk> {
        // First, try to split by headings
        let heading_sections = self.split_by_headings(content);
        
        if heading_sections.is_empty() {
            // No headings found, fall back to paragraph-based chunking
            return paragraph::chunk_by_paragraphs(
                content,
                None,
                0,
                self.max_chunk_size,
                self.overlap,
            );
        }

        let mut chunks = Vec::new();
        
        for section in heading_sections {
            // If section is small enough, create a single chunk
            if section.text.len() <= self.max_chunk_size {
                chunks.push(Chunk {
                    heading_path: section.heading_path.clone(),
                    text: section.text.clone(),
                    byte_offset: section.byte_offset,
                    byte_length: section.text.len(),
                    token_count: self.estimate_tokens(&section.text),
                });
            } else {
                // Section is too large, split by paragraphs
                let sub_chunks = paragraph::chunk_by_paragraphs(
                    &section.text,
                    section.heading_path.as_deref(),
                    section.byte_offset,
                    self.max_chunk_size,
                    self.overlap,
                );
                chunks.extend(sub_chunks);
            }
        }

        chunks
    }

    /// Split content by markdown headings
    fn split_by_headings(&self, content: &str) -> Vec<Section> {
        let mut sections = Vec::new();
        let mut heading_stack: Vec<HeadingInfo> = Vec::new();
        let mut current_text = String::new();
        let mut section_start_offset = 0;
        let mut current_offset = 0;

        for line in content.lines() {
            let line_with_newline = format!("{}\n", line);
            
            if let Some(heading) = heading::parse_heading(line) {
                // Save previous section if it has content
                if !current_text.trim().is_empty() {
                    let heading_path = self.build_heading_path(&heading_stack);
                    sections.push(Section {
                        heading_path,
                        text: current_text.clone(),
                        byte_offset: section_start_offset,
                    });
                }

                // Update heading stack
                self.update_heading_stack(&mut heading_stack, heading, current_offset);

                // Start new section
                current_text = line_with_newline.clone();
                section_start_offset = current_offset;
            } else {
                current_text.push_str(&line_with_newline);
            }

            current_offset += line_with_newline.len();
        }

        // Don't forget the last section
        if !current_text.trim().is_empty() {
            let heading_path = self.build_heading_path(&heading_stack);
            sections.push(Section {
                heading_path,
                text: current_text,
                byte_offset: section_start_offset,
            });
        }

        sections
    }

    /// Update the heading stack based on the new heading level
    fn update_heading_stack(&self, stack: &mut Vec<HeadingInfo>, mut new_heading: HeadingInfo, offset: usize) {
        new_heading.byte_offset = offset;
        
        // Pop headings at same or lower level
        while let Some(top) = stack.last() {
            if top.level >= new_heading.level {
                stack.pop();
            } else {
                break;
            }
        }
        
        stack.push(new_heading);
    }

    /// Build a heading path string from the stack (e.g., "# Main > ## Sub > ### Detail")
    fn build_heading_path(&self, stack: &[HeadingInfo]) -> Option<String> {
        if stack.is_empty() {
            return None;
        }

        let parts: Vec<String> = stack
            .iter()
            .map(|h| format!("{} {}", "#".repeat(h.level), h.text))
            .collect();

        Some(parts.join(" > "))
    }

    /// Estimate token count (rough approximation: 1 token ≈ 4 characters)
    fn estimate_tokens(&self, text: &str) -> usize {
        // Simple heuristic: avg 4 chars per token
        // Also count whitespace-separated words for better accuracy
        let char_estimate = text.len() / 4;
        let word_count = text.split_whitespace().count();
        
        // Use average of both estimates
        (char_estimate + word_count) / 2
    }
}

#[derive(Debug, Clone)]
struct Section {
    heading_path: Option<String>,
    text: String,
    byte_offset: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_heading() {
        let h1 = heading::parse_heading("# Main Title");
        assert!(h1.is_some());
        let h1 = h1.unwrap();
        assert_eq!(h1.level, 1);
        assert_eq!(h1.text, "Main Title");

        let h2 = heading::parse_heading("## Subtitle");
        assert!(h2.is_some());
        assert_eq!(h2.unwrap().level, 2);

        let not_heading = heading::parse_heading("#NoSpace");
        assert!(not_heading.is_none());

        let not_heading2 = heading::parse_heading("Regular text");
        assert!(not_heading2.is_none());
    }

    #[test]
    fn test_chunk_simple_document() {
        let chunker = MarkdownChunker::new(500, 50);
        let content = r#"# Introduction

This is the introduction paragraph.

## Background

Some background information here.

### Details

More detailed information.
"#;

        let chunks = chunker.chunk(content);
        assert!(!chunks.is_empty());

        // Should have chunks with proper heading paths
        assert!(chunks.iter().any(|c| c.heading_path.is_some()));
    }

    #[test]
    fn test_chunk_no_headings() {
        let chunker = MarkdownChunker::new(100, 20);
        let content = "This is a simple paragraph without any headings. It should still be chunked properly.";

        let chunks = chunker.chunk(content);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].heading_path, None);
    }

    #[test]
    fn test_estimate_tokens() {
        let chunker = MarkdownChunker::default();
        
        let text = "This is a test sentence.";
        let tokens = chunker.estimate_tokens(text);
        
        // Should be roughly 5-6 tokens (within reasonable range)
        assert!(tokens >= 4 && tokens <= 8);
    }

    #[test]
    fn test_split_into_paragraphs() {
        let content = r#"First paragraph.

Second paragraph.

Third paragraph."#;

        let paragraphs = paragraph::split_into_paragraphs(content);
        assert_eq!(paragraphs.len(), 3);
    }

    #[test]
    fn test_heading_path_generation() {
        let chunker = MarkdownChunker::default();
        let content = r#"# Main
Some text.
## Sub1
More text.
### Detail
Details here.
"#;

        let chunks = chunker.chunk(content);
        
        // Should have a chunk with nested heading path
        let nested_chunk = chunks.iter().find(|c| {
            c.heading_path.as_ref().map_or(false, |p| p.contains(" > "))
        });
        assert!(nested_chunk.is_some());
    }
}
