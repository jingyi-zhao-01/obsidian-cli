use super::HeadingInfo;

pub fn parse_heading(line: &str) -> Option<HeadingInfo> {
    let trimmed = line.trim_start();

    if !trimmed.starts_with('#') {
        return None;
    }

    let mut level = 0;
    let mut chars = trimmed.chars();

    while let Some('#') = chars.next() {
        level += 1;
        if level > 6 {
            return None; // Not a valid heading
        }
    }

    // Must have space after #
    let rest = &trimmed[level..];
    if !rest.starts_with(' ') {
        return None;
    }

    let text = rest.trim().to_string();

    Some(HeadingInfo {
        level,
        text,
        byte_offset: 0, // Will be set by caller
    })
}

#[cfg(test)]
mod tests {
    use super::parse_heading;

    #[test]
    fn test_parse_heading_h1() {
        let result = parse_heading("# Main Title");
        assert!(result.is_some());
        let heading = result.unwrap();
        assert_eq!(heading.level, 1);
        assert_eq!(heading.text, "Main Title");
    }

    #[test]
    fn test_parse_heading_h2() {
        let result = parse_heading("## Subtitle");
        assert!(result.is_some());
        assert_eq!(result.unwrap().level, 2);
    }

    #[test]
    fn test_parse_heading_h6() {
        let result = parse_heading("###### H6");
        assert!(result.is_some());
        assert_eq!(result.unwrap().level, 6);
    }

    #[test]
    fn test_parse_heading_invalid_level() {
        let result = parse_heading("####### Too many");
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_heading_no_space() {
        let result = parse_heading("#NoSpace");
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_heading_not_heading() {
        let result = parse_heading("Regular text");
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_heading_leading_whitespace() {
        let result = parse_heading("   # Indented");
        assert!(result.is_some());
        assert_eq!(result.unwrap().level, 1);
    }

    #[test]
    fn test_parse_heading_empty_text() {
        let result = parse_heading("# ");
        assert!(result.is_some());
        assert!(result.unwrap().text.is_empty());
    }

    #[test]
    fn test_parse_heading_with_special_chars() {
        let result = parse_heading("# Heading with: special chars!");
        assert!(result.is_some());
        assert_eq!(result.unwrap().text, "Heading with: special chars!");
    }

    #[test]
    fn test_parse_heading_only_hashes() {
        let result = parse_heading("#");
        assert!(result.is_none());
    }
}
