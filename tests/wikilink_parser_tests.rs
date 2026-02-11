use obsidian_cli_inspector::parser::{extract_wikilinks, LinkType};

#[test]
fn test_extract_wikilinks_simple() {
    let content = "Text with [[link]] inside";
    let links = extract_wikilinks(content);
    assert_eq!(links.len(), 1);
    assert_eq!(links[0].text, "link");
}

#[test]
fn test_extract_wikilinks_multiple() {
    let content = "[[link1]] and [[link2]]";
    let links = extract_wikilinks(content);
    assert_eq!(links.len(), 2);
    assert_eq!(links[0].text, "link1");
    assert_eq!(links[1].text, "link2");
}

#[test]
fn test_extract_wikilinks_embed() {
    let content = "Embed: ![[image]]";
    let links = extract_wikilinks(content);
    assert_eq!(links.len(), 1);
    assert!(links[0].is_embed);
    assert_eq!(links[0].text, "image");
}

#[test]
fn test_extract_wikilinks_with_alias() {
    let content = "[[Note|Display Name]]";
    let links = extract_wikilinks(content);
    assert_eq!(links.len(), 1);
    assert_eq!(links[0].text, "Note");
    assert_eq!(links[0].alias, Some("Display Name".to_string()));
}

#[test]
fn test_extract_wikilinks_with_heading() {
    let content = "[[Note#Section]]";
    let links = extract_wikilinks(content);
    assert_eq!(links.len(), 1);
    assert_eq!(links[0].text, "Note");
    assert_eq!(links[0].heading_ref, Some("Section".to_string()));
}

#[test]
fn test_extract_wikilinks_with_block() {
    let content = "[[Note#^block123]]";
    let links = extract_wikilinks(content);
    assert_eq!(links.len(), 1);
    assert_eq!(links[0].text, "Note");
    assert_eq!(links[0].block_ref, Some("block123".to_string()));
}

#[test]
fn test_extract_wikilinks_empty() {
    let content = "No links here";
    let links = extract_wikilinks(content);
    assert_eq!(links.len(), 0);
}

#[test]
fn test_extract_wikilinks_incomplete() {
    let content = "[[incomplete link";
    let links = extract_wikilinks(content);
    assert_eq!(links.len(), 0);
}

#[test]
fn test_extract_wikilinks_empty_brackets() {
    let content = "[[]]";
    let links = extract_wikilinks(content);
    assert_eq!(links.len(), 0);
}

#[test]
fn test_extract_wikilinks_nested() {
    let content = "[[outer [[inner]]]]";
    let links = extract_wikilinks(content);
    // Should handle gracefully - behavior may vary
    assert!(!links.is_empty());
}

#[test]
fn test_extract_wikilinks_with_spaces() {
    let content = "[[  Note Name  ]]";
    let links = extract_wikilinks(content);
    assert_eq!(links.len(), 1);
    assert_eq!(links[0].text, "Note Name");
}

#[test]
fn test_extract_wikilinks_path_with_slashes() {
    let content = "[[folder/subfolder/Note]]";
    let links = extract_wikilinks(content);
    assert_eq!(links.len(), 1);
    assert_eq!(links[0].text, "folder/subfolder/Note");
}

#[test]
fn test_extract_wikilinks_mixed_with_markdown() {
    let content = "[[wikilink]] and [markdown](link.md)";
    let links = extract_wikilinks(content);
    // Should only extract wikilinks
    assert_eq!(links.len(), 1);
    assert_eq!(links[0].text, "wikilink");
}

#[test]
fn test_extract_wikilinks_link_type() {
    let content = "[[note]]";
    let links = extract_wikilinks(content);
    assert_eq!(links.len(), 1);
    assert_eq!(links[0].link_type, LinkType::Wiki);
}

#[test]
fn test_extract_wikilinks_with_alias_and_heading() {
    let content = "[[Note#Heading|Alias]]";
    let links = extract_wikilinks(content);
    assert_eq!(links.len(), 1);
    assert_eq!(links[0].text, "Note");
    assert_eq!(links[0].heading_ref, Some("Heading".to_string()));
    assert_eq!(links[0].alias, Some("Alias".to_string()));
}

#[test]
fn test_extract_wikilinks_multiple_embeds() {
    let content = "![[image1]] and ![[image2]]";
    let links = extract_wikilinks(content);
    assert_eq!(links.len(), 2);
    assert!(links[0].is_embed);
    assert!(links[1].is_embed);
}

#[test]
fn test_extract_wikilinks_mixed_embed_and_regular() {
    let content = "![[embed]] and [[regular]]";
    let links = extract_wikilinks(content);
    assert_eq!(links.len(), 2);
    assert!(links[0].is_embed);
    assert!(!links[1].is_embed);
}

#[test]
fn test_extract_wikilinks_only_closing_brackets() {
    let content = "Some text ]] without opening";
    let links = extract_wikilinks(content);
    assert_eq!(links.len(), 0);
}

#[test]
fn test_extract_wikilinks_single_bracket() {
    let content = "Single [bracket]";
    let links = extract_wikilinks(content);
    assert_eq!(links.len(), 0);
}
