use obsidian_cli_inspector::parser::{extract_markdown_links, build_markdown_link, LinkType};

#[test]
fn test_extract_markdown_links_simple() {
    let content = "Check out [this link](path/to/note.md)";
    let links = extract_markdown_links(content);
    assert_eq!(links.len(), 1);
    assert_eq!(links[0].text, "path/to/note");
    assert_eq!(links[0].link_type, LinkType::Markdown);
}

#[test]
fn test_extract_markdown_links_multiple() {
    let content = "Here is [link 1](note1.md) and [link 2](note2.md)";
    let links = extract_markdown_links(content);
    assert_eq!(links.len(), 2);
    assert_eq!(links[0].text, "note1");
    assert_eq!(links[1].text, "note2");
}

#[test]
fn test_extract_markdown_links_with_heading() {
    let content = "Check [link](note.md#heading)";
    let links = extract_markdown_links(content);
    assert_eq!(links.len(), 1);
    assert_eq!(links[0].text, "note");
    assert_eq!(links[0].heading_ref, Some("heading".to_string()));
}

#[test]
fn test_extract_markdown_links_with_block_ref() {
    let content = "Check [link](note.md#^blockid)";
    let links = extract_markdown_links(content);
    assert_eq!(links.len(), 1);
    assert_eq!(links[0].text, "note");
    assert_eq!(links[0].block_ref, Some("blockid".to_string()));
}

#[test]
fn test_extract_markdown_links_ignores_images() {
    let content = "![image](image.png) and [link](note.md)";
    let links = extract_markdown_links(content);
    assert_eq!(links.len(), 1);
    assert_eq!(links[0].text, "note");
}

#[test]
fn test_extract_markdown_links_ignores_external() {
    let content = "[external](https://example.com) and [internal](note.md)";
    let links = extract_markdown_links(content);
    assert_eq!(links.len(), 1);
    assert_eq!(links[0].text, "note");
}

#[test]
fn test_extract_markdown_links_ignores_http() {
    let content = "[link](http://example.com)";
    let links = extract_markdown_links(content);
    assert_eq!(links.len(), 0);
}

#[test]
fn test_extract_markdown_links_ignores_mailto() {
    let content = "[email](mailto:test@example.com)";
    let links = extract_markdown_links(content);
    assert_eq!(links.len(), 0);
}

#[test]
fn test_extract_markdown_links_ignores_anchor() {
    let content = "[anchor](#section)";
    let links = extract_markdown_links(content);
    assert_eq!(links.len(), 0);
}

#[test]
fn test_extract_markdown_links_empty() {
    let content = "No links here";
    let links = extract_markdown_links(content);
    assert_eq!(links.len(), 0);
}

#[test]
fn test_extract_markdown_links_incomplete_brackets() {
    let content = "[incomplete link without closing paren";
    let links = extract_markdown_links(content);
    assert_eq!(links.len(), 0);
}

#[test]
fn test_build_markdown_link_simple() {
    let link = build_markdown_link("Label", "note.md", false);
    assert!(link.is_some());
    let link = link.unwrap();
    assert_eq!(link.text, "note");
    assert_eq!(link.alias, Some("Label".to_string()));
    assert!(!link.is_embed);
}

#[test]
fn test_build_markdown_link_empty_label() {
    let link = build_markdown_link("", "note.md", false);
    assert!(link.is_some());
    let link = link.unwrap();
    assert_eq!(link.text, "note");
    assert_eq!(link.alias, None);
}

#[test]
fn test_build_markdown_link_empty_dest() {
    let link = build_markdown_link("Label", "", false);
    assert!(link.is_none());
}

#[test]
fn test_build_markdown_link_with_heading() {
    let link = build_markdown_link("Label", "note.md#heading", false);
    assert!(link.is_some());
    let link = link.unwrap();
    assert_eq!(link.text, "note");
    assert_eq!(link.heading_ref, Some("heading".to_string()));
    assert_eq!(link.block_ref, None);
}

#[test]
fn test_build_markdown_link_with_block() {
    let link = build_markdown_link("Label", "note.md#^block123", false);
    assert!(link.is_some());
    let link = link.unwrap();
    assert_eq!(link.text, "note");
    assert_eq!(link.heading_ref, None);
    assert_eq!(link.block_ref, Some("block123".to_string()));
}

#[test]
fn test_build_markdown_link_embed() {
    let link = build_markdown_link("Label", "note.md", true);
    assert!(link.is_some());
    let link = link.unwrap();
    assert!(link.is_embed);
}

#[test]
fn test_build_markdown_link_ignores_http() {
    let link = build_markdown_link("Label", "http://example.com", false);
    assert!(link.is_none());
}

#[test]
fn test_build_markdown_link_ignores_https() {
    let link = build_markdown_link("Label", "https://example.com", false);
    assert!(link.is_none());
}

#[test]
fn test_build_markdown_link_ignores_mailto() {
    let link = build_markdown_link("Label", "mailto:test@example.com", false);
    assert!(link.is_none());
}

#[test]
fn test_build_markdown_link_ignores_anchor() {
    let link = build_markdown_link("Label", "#anchor", false);
    assert!(link.is_none());
}

#[test]
fn test_build_markdown_link_with_angle_brackets() {
    let link = build_markdown_link("Label", "<note.md>", false);
    assert!(link.is_some());
    let link = link.unwrap();
    // The text keeps the angle brackets and extension as-is
    assert_eq!(link.text, "<note.md>");
}

#[test]
fn test_build_markdown_link_whitespace_label() {
    let link = build_markdown_link("  Label  ", "note.md", false);
    assert!(link.is_some());
    let link = link.unwrap();
    assert_eq!(link.alias, Some("Label".to_string()));
}

#[test]
fn test_extract_markdown_links_nested_brackets() {
    let content = "Text with [nested [brackets]](note.md)";
    let links = extract_markdown_links(content);
    // Should handle this gracefully - exact behavior may vary
    assert!(links.len() <= 1); // At most one link extracted
}
