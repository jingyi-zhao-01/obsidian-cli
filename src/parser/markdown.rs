use super::{Link, LinkType, normalize_note_identifier};

pub fn extract_markdown_links(content: &str) -> Vec<Link> {
    let mut links = Vec::new();
    let chars: Vec<char> = content.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '[' {
            let is_image = i > 0 && chars[i - 1] == '!';
            if is_image {
                i += 1;
                continue;
            }

            if let Some((label_end, label)) = parse_bracket_section(&chars, i, '[', ']') {
                let next = label_end + 1;
                if next < chars.len() && chars[next] == '(' {
                    if let Some((dest_end, dest_raw)) =
                        parse_bracket_section(&chars, next, '(', ')')
                    {
                        let dest = clean_markdown_link_destination(&dest_raw);
                        if let Some(link) = build_markdown_link(&label, &dest, false) {
                            links.push(link);
                        }
                        i = dest_end + 1;
                        continue;
                    }
                }
            }
        }

        i += 1;
    }

    links
}

pub fn build_markdown_link(label: &str, dest: &str, is_embed: bool) -> Option<Link> {
    if dest.is_empty() {
        return None;
    }

    let dest_lower = dest.to_lowercase();
    if dest_lower.starts_with("http://")
        || dest_lower.starts_with("https://")
        || dest_lower.starts_with("mailto:")
        || dest_lower.starts_with('#')
    {
        return None;
    }

    let (path, heading_ref, block_ref) = split_heading_block(dest);
    let text = normalize_note_identifier(&path);
    if text.is_empty() {
        return None;
    }

    let alias = if label.trim().is_empty() {
        None
    } else {
        Some(label.trim().to_string())
    };

    Some(Link {
        text,
        alias,
        heading_ref,
        block_ref,
        is_embed,
        link_type: LinkType::Markdown,
    })
}

fn parse_bracket_section(
    chars: &[char],
    start: usize,
    open: char,
    close: char,
) -> Option<(usize, String)> {
    if start >= chars.len() || chars[start] != open {
        return None;
    }

    let mut idx = start + 1;
    while idx < chars.len() {
        if chars[idx] == close {
            let content: String = chars[start + 1..idx].iter().collect();
            return Some((idx, content));
        }
        idx += 1;
    }

    None
}

fn clean_markdown_link_destination(dest: &str) -> String {
    let trimmed = dest.trim();
    let trimmed = trimmed
        .trim_start_matches('<')
        .trim_end_matches('>')
        .trim();
    let mut parts = trimmed.split_whitespace();
    parts.next().unwrap_or("").to_string()
}

fn split_heading_block(dest: &str) -> (String, Option<String>, Option<String>) {
    if let Some(hash_pos) = dest.find('#') {
        let path = dest[..hash_pos].trim().to_string();
        let fragment = dest[hash_pos + 1..].trim();
        if fragment.starts_with('^') {
            return (
                path,
                None,
                Some(fragment.trim_start_matches('^').to_string()),
            );
        }
        return (path, Some(fragment.to_string()), None);
    }

    (dest.trim().to_string(), None, None)
}
