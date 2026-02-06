use super::{Link, LinkType, normalize_note_identifier};

pub fn extract_wikilinks(content: &str) -> Vec<Link> {
    let mut links = Vec::new();
    let content_chars: Vec<char> = content.chars().collect();
    let mut pos = 0;

    while pos < content_chars.len() {
        // Check for wikilink or embed
        if pos + 1 < content_chars.len() {
            let is_embed = content_chars[pos] == '!' && content_chars[pos + 1] == '[';
            let is_wikilink = content_chars[pos] == '[' && content_chars[pos + 1] == '[';

            if is_embed {
                // ![[...]]
                if let Some(link) = parse_wikilink(&content_chars, pos + 1, true) {
                    links.push(link);
                }
                pos += 1;
            } else if is_wikilink {
                // [[...]]
                if let Some(link) = parse_wikilink(&content_chars, pos, false) {
                    links.push(link);
                }
                pos += 1;
            }
        }
        pos += 1;
    }

    links
}

pub fn parse_wikilink(chars: &[char], start: usize, is_embed: bool) -> Option<Link> {
    if start + 3 >= chars.len() || chars[start] != '[' || chars[start + 1] != '[' {
        return None;
    }

    let mut end = start + 2;
    while end + 1 < chars.len() && !(chars[end] == ']' && chars[end + 1] == ']') {
        end += 1;
    }

    if end + 1 >= chars.len() {
        return None;
    }

    let content: String = chars[start + 2..end].iter().collect();
    let (text, alias, heading_ref, block_ref) = parse_link_content(&content);
    let text = normalize_note_identifier(&text);

    if text.is_empty() {
        return None;
    }

    Some(Link {
        text,
        alias,
        heading_ref,
        block_ref,
        is_embed,
        link_type: LinkType::Wiki,
    })
}

fn parse_link_content(content: &str) -> (String, Option<String>, Option<String>, Option<String>) {
    let mut alias = None;
    let mut heading_ref = None;
    let mut block_ref = None;

    // Check for pipe (alias)
    let text = if let Some(pipe_pos) = content.find('|') {
        let t = content[..pipe_pos].trim().to_string();
        alias = Some(content[pipe_pos + 1..].trim().to_string());
        t
    } else {
        content.to_string()
    };

    // Check for heading reference
    let text = if let Some(hash_pos) = text.find('#') {
        let heading = text[hash_pos + 1..].trim().to_string();
        let t = text[..hash_pos].trim().to_string();

        if heading.starts_with('^') {
            block_ref = Some(heading[1..].to_string());
        } else {
            heading_ref = Some(heading);
        }
        t
    } else {
        text
    };

    (text, alias, heading_ref, block_ref)
}
