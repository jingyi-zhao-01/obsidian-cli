use std::collections::HashMap;

mod wikilink;
mod markdown;

pub use wikilink::{extract_wikilinks, parse_wikilink};
pub use markdown::{extract_markdown_links, build_markdown_link};

#[derive(Debug, Clone)]
pub struct ParsedNote {
    pub title: String,
    pub frontmatter: HashMap<String, String>,
    pub tags: Vec<String>,
    pub links: Vec<Link>,
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct Link {
    pub text: String,
    pub alias: Option<String>,
    pub heading_ref: Option<String>,
    pub block_ref: Option<String>,
    pub is_embed: bool,
    pub link_type: LinkType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkType {
    Wiki,
    Markdown,
}

impl LinkType {
    pub fn as_str(&self) -> &'static str {
        match self {
            LinkType::Wiki => "wikilink",
            LinkType::Markdown => "markdown",
        }
    }
}

pub struct MarkdownParser;

impl MarkdownParser {
    pub fn parse(content: &str) -> ParsedNote {
        let (frontmatter, rest) = Self::extract_frontmatter(content);
        let tags = Self::extract_tags(&frontmatter, &rest);
        let links = Self::extract_links(&rest);
        let title = Self::extract_title(&frontmatter, &rest);

        ParsedNote {
            title,
            frontmatter,
            tags,
            links,
            text: rest.to_string(),
        }
    }

    fn extract_frontmatter(content: &str) -> (HashMap<String, String>, &str) {
        let mut map = HashMap::new();

        if !content.starts_with("---") {
            return (map, content);
        }

        let rest = &content[3..];
        if let Some(end_pos) = rest.find("---") {
            let frontmatter_text = &rest[..end_pos];
            let content_after = &rest[end_pos + 3..].trim_start();

            for line in frontmatter_text.lines() {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }

                if let Some(colon_pos) = line.find(':') {
                    let key = line[..colon_pos].trim().to_lowercase();
                    let value = line[colon_pos + 1..].trim().to_string();

                    // Special handling for tags which might be arrays
                    if key == "tags" {
                        let tags_str = value.trim_start_matches('[').trim_end_matches(']');
                        for tag in tags_str.split(',') {
                            let clean_tag = tag.trim().trim_matches('"').trim_matches('\'');
                            if !clean_tag.is_empty() {
                                map.insert(
                                    format!("tag_{}", clean_tag),
                                    clean_tag.to_string(),
                                );
                            }
                        }
                    } else {
                        map.insert(key, value);
                    }
                }
            }

            return (map, content_after);
        }

        (map, content)
    }

    fn extract_title(frontmatter: &HashMap<String, String>, content: &str) -> String {
        // Try to get from frontmatter
        if let Some(title) = frontmatter.get("title") {
            return title.clone();
        }

        // Try to extract from first heading
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("# ") {
                return trimmed[2..].trim().to_string();
            }
        }

        // Fallback to empty string
        String::new()
    }

    fn extract_tags(frontmatter: &HashMap<String, String>, content: &str) -> Vec<String> {
        let mut tags = Vec::new();

        // From frontmatter
        for (key, value) in frontmatter {
            if key.starts_with("tag_") {
                tags.push(value.clone());
            }
        }

        // From inline tags in content
        for word in content.split_whitespace() {
            if word.starts_with('#') && word.len() > 1 {
                let tag = word.trim_matches(|c: char| !c.is_alphanumeric() && c != '/' && c != '_')
                    .trim_start_matches('#');
                if !tag.is_empty() && !tags.contains(&tag.to_string()) {
                    tags.push(tag.to_string());
                }
            }
        }

        tags.sort();
        tags.dedup();
        tags
    }

    fn extract_links(content: &str) -> Vec<Link> {
        let mut links = extract_wikilinks(content);
        links.extend(extract_markdown_links(content));
        links
    }
}

pub fn normalize_note_identifier(raw: &str) -> String {
    let mut value = raw.trim().to_string();
    if value.starts_with("./") {
        value = value.trim_start_matches("./").to_string();
    }
    value = value.replace('\\', "/");
    if value.ends_with(".md") || value.ends_with(".MD") {
        let len = value.len();
        value = value[..len.saturating_sub(3)].to_string();
    }
    value.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_wikilink_simple() {
        let parsed = MarkdownParser::parse("This is [[note]] link");
        assert_eq!(parsed.links.len(), 1);
        assert_eq!(parsed.links[0].text, "note");
    }

    #[test]
    fn test_parse_wikilink_with_alias() {
        let parsed = MarkdownParser::parse("This is [[note|alias]] link");
        assert_eq!(parsed.links.len(), 1);
        assert_eq!(parsed.links[0].text, "note");
        assert_eq!(parsed.links[0].alias, Some("alias".to_string()));
    }

    #[test]
    fn test_parse_markdown_link_basic() {
        let parsed = MarkdownParser::parse("See [Doc](docs/Note.md)");
        assert_eq!(parsed.links.len(), 1);
        assert_eq!(parsed.links[0].text, "docs/Note");
        assert_eq!(parsed.links[0].alias, Some("Doc".to_string()));
        assert_eq!(parsed.links[0].link_type, LinkType::Markdown);
    }

    #[test]
    fn test_normalize_note_identifier() {
        assert_eq!(normalize_note_identifier("./Note.md"), "Note");
        assert_eq!(normalize_note_identifier("Folder\\Note.md"), "Folder/Note");
    }
}
