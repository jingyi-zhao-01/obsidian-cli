use obsidian_cli_inspector::parser::{normalize_note_identifier, MarkdownParser};

#[test]
fn test_normalize_note_identifier_removes_md_extension() {
    assert_eq!(normalize_note_identifier("note.md"), "note");
    assert_eq!(normalize_note_identifier("note.MD"), "note");
}

#[test]
fn test_normalize_note_identifier_removes_leading_dot_slash() {
    assert_eq!(normalize_note_identifier("./note.md"), "note");
}

#[test]
fn test_normalize_note_identifier_replaces_backslash() {
    assert_eq!(normalize_note_identifier("folder\\note.md"), "folder/note");
}

#[test]
fn test_normalize_note_identifier_trims_whitespace() {
    assert_eq!(normalize_note_identifier("  note.md  "), "note");
}

#[test]
fn test_normalize_note_identifier_combined() {
    assert_eq!(normalize_note_identifier("  ./folder\\note.md  "), "folder/note");
}

#[test]
fn test_normalize_note_identifier_no_extension() {
    assert_eq!(normalize_note_identifier("note"), "note");
}

#[test]
fn test_normalize_note_identifier_empty() {
    assert_eq!(normalize_note_identifier(""), "");
}

#[test]
fn test_markdown_parser_simple() {
    let content = "# Test Note\n\nSome content";
    let parsed = MarkdownParser::parse(content);
    assert_eq!(parsed.title, "Test Note");
    assert!(parsed.text.contains("Some content"));
}

#[test]
fn test_markdown_parser_with_frontmatter() {
    let content = "---\ntitle: My Title\ntag_custom: mytag\n---\n\nContent";
    let parsed = MarkdownParser::parse(content);
    assert_eq!(parsed.title, "My Title");
    assert!(parsed.tags.contains(&"mytag".to_string()));
}

#[test]
fn test_markdown_parser_extracts_inline_tags() {
    let content = "# Note\n\nThis has #tag1 and #tag2";
    let parsed = MarkdownParser::parse(content);
    assert!(parsed.tags.contains(&"tag1".to_string()));
    assert!(parsed.tags.contains(&"tag2".to_string()));
}

#[test]
fn test_markdown_parser_extracts_wikilinks() {
    let content = "# Note\n\nLink to [[other note]]";
    let parsed = MarkdownParser::parse(content);
    assert_eq!(parsed.links.len(), 1);
    assert_eq!(parsed.links[0].text, "other note");
}

#[test]
fn test_markdown_parser_extracts_markdown_links() {
    let content = "# Note\n\nLink to [other](note.md)";
    let parsed = MarkdownParser::parse(content);
    assert_eq!(parsed.links.len(), 1);
    assert_eq!(parsed.links[0].text, "note");
}

#[test]
fn test_markdown_parser_extracts_both_link_types() {
    let content = "[[wiki]] and [markdown](note.md)";
    let parsed = MarkdownParser::parse(content);
    assert_eq!(parsed.links.len(), 2);
}

#[test]
fn test_markdown_parser_no_title() {
    let content = "Just content without heading";
    let parsed = MarkdownParser::parse(content);
    assert_eq!(parsed.title, "");
}

#[test]
fn test_markdown_parser_empty_content() {
    let content = "";
    let parsed = MarkdownParser::parse(content);
    assert_eq!(parsed.title, "");
    assert!(parsed.tags.is_empty());
    assert!(parsed.links.is_empty());
}

#[test]
fn test_markdown_parser_title_from_frontmatter_priority() {
    let content = "---\ntitle: Frontmatter Title\n---\n# Heading Title\n\nContent";
    let parsed = MarkdownParser::parse(content);
    // Frontmatter title takes priority
    assert_eq!(parsed.title, "Frontmatter Title");
}

#[test]
fn test_markdown_parser_tag_deduplication() {
    let content = "# Note\n\n#tag1 #tag2 #tag1 #tag2";
    let parsed = MarkdownParser::parse(content);
    assert_eq!(parsed.tags.len(), 2);
}

#[test]
fn test_markdown_parser_tags_sorted() {
    let content = "#zebra #apple #middle";
    let parsed = MarkdownParser::parse(content);
    assert!(parsed.tags[0] < parsed.tags[parsed.tags.len() - 1]);
}

#[test]
fn test_normalize_note_identifier_with_path() {
    assert_eq!(
        normalize_note_identifier("folder/subfolder/note.md"),
        "folder/subfolder/note"
    );
}

#[test]
fn test_normalize_note_identifier_only_md() {
    assert_eq!(normalize_note_identifier(".md"), "");
}

#[test]
fn test_markdown_parser_complex_frontmatter() {
    let content = "---\ntitle: Complex\ntag_work: work\ntag_project: project\nauthor: Test\n---\n\n# Heading\n\nContent with #inline";
    let parsed = MarkdownParser::parse(content);
    assert_eq!(parsed.title, "Complex");
    assert!(parsed.tags.contains(&"work".to_string()));
    assert!(parsed.tags.contains(&"project".to_string()));
    assert!(parsed.tags.contains(&"inline".to_string()));
}

#[test]
fn test_markdown_parser_tag_with_slashes() {
    let content = "#category/subcategory and #simple";
    let parsed = MarkdownParser::parse(content);
    assert!(parsed.tags.contains(&"category/subcategory".to_string()));
}

#[test]
fn test_markdown_parser_tag_with_underscores() {
    let content = "#tag_with_underscores";
    let parsed = MarkdownParser::parse(content);
    assert!(parsed.tags.contains(&"tag_with_underscores".to_string()));
}

#[test]
fn test_normalize_note_identifier_mixed_case_extension() {
    assert_eq!(normalize_note_identifier("Note.Md"), "Note.Md");
    assert_eq!(normalize_note_identifier("Note.mD"), "Note.mD");
}
