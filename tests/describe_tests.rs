
mod common;

use anyhow::Result;
use obsidian_cli_inspector::commands::*;

// CLI equivalent: cargo run -- describe "Home.md"
#[test]
fn test_describe_note() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;

    // Setup
    initialize_database(&config, false, None)?;
    index_vault(&config, false, false, false, None)?;

    // Test describe command
    get_note_describe(&config, "Home.md", None)?;

    Ok(())
}

// CLI equivalent: cargo run -- describe "NonExistent.md"
#[test]
fn test_describe_nonexistent_note() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;

    // Setup
    initialize_database(&config, false, None)?;
    index_vault(&config, false, false, false, None)?;

    // Test describe for non-existent note
    get_note_describe(&config, "NonExistent.md", None)?;

    Ok(())
}

// Test partial matching
#[test]
fn test_describe_partial_match() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;

    // Setup
    initialize_database(&config, false, None)?;
    index_vault(&config, false, false, false, None)?;

    // Test describe with partial match
    get_note_describe(&config, "Home", None)?;

    Ok(())
}

// Test describe note with frontmatter
#[test]
fn test_describe_note_with_frontmatter() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;

    // Setup
    initialize_database(&config, false, None)?;
    index_vault(&config, false, false, false, None)?;

    // Test describe command - notes with frontmatter
    get_note_describe(&config, "Projects.md", None)?;

    Ok(())
}

// Test describe with exact title match
#[test]
fn test_describe_by_title() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;

    // Setup
    initialize_database(&config, false, None)?;
    index_vault(&config, false, false, false, None)?;

    // Test describe by title
    get_note_describe(&config, "Home", None)?;

    Ok(())
}

// Test describe - empty vault
#[test]
fn test_describe_empty_vault() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;

    // Setup - only initialize, don't index
    initialize_database(&config, false, None)?;

    // Test describe on empty vault - should fail gracefully
    let result = get_note_describe(&config, "Home.md", None);
    // Result may be Ok or Err depending on whether note exists
    Ok(())
}

// Test get_backlinks for a note
#[test]
fn test_backlinks_command() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;

    // Setup
    initialize_database(&config, false, None)?;
    index_vault(&config, false, false, false, None)?;

    // Test backlinks
    get_backlinks(&config, "Productivity.md", None)?;

    Ok(())
}

// Test get_forward_links for a note
#[test]
fn test_forward_links_command() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;

    // Setup
    initialize_database(&config, false, None)?;
    index_vault(&config, false, false, false, None)?;

    // Test forward links
    get_forward_links(&config, "Productivity.md", None)?;

    Ok(())
}

// Test list_unresolved_links
#[test]
fn test_unresolved_links_command() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;

    // Setup
    initialize_database(&config, false, None)?;
    index_vault(&config, false, false, false, None)?;

    // Test unresolved links
    list_unresolved_links(&config, None)?;

    Ok(())
}

// Test list_notes_by_tag with specific tag
#[test]
fn test_list_notes_by_tag_command() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;

    // Setup
    initialize_database(&config, false, None)?;
    index_vault(&config, false, false, false, None)?;

    // Test list notes by tag
    list_notes_by_tag(&config, &Some("productivity".to_string()), false, None)?;

    Ok(())
}

// Test list_notes_by_tag all
#[test]
fn test_list_all_tags_command() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;

    // Setup
    initialize_database(&config, false, None)?;
    index_vault(&config, false, false, false, None)?;

    // Test list all notes by tags
    list_notes_by_tag(&config, &None, true, None)?;

    Ok(())
}
