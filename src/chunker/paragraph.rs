use super::Chunk;
use super::overlap::get_overlap_text;

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
        current_chunk.push_str("\n\n");
        current_offset = para_offset + para_text.len();
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

    for line in content.lines() {
        let line_with_newline = format!("{}\n", line);
        
        if line.trim().is_empty() {
            // Blank line - end of paragraph
            if !current_para.trim().is_empty() {
                paragraphs.push((current_para.clone(), para_start_offset));
                current_para.clear();
            }
        } else {
            // Continue paragraph
            if current_para.is_empty() {
                para_start_offset = current_offset;
            }
            current_para.push_str(&line_with_newline);
        }

        current_offset += line_with_newline.len();
    }

    // Don't forget the last paragraph
    if !current_para.trim().is_empty() {
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
