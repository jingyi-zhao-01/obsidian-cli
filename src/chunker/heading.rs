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
