mod common;

use anyhow::Result;
use obsidian_cli::commands::*;

// CLI equivalent: cargo run -- --config test-config.toml tags learning
// CLI equivalent: cargo run -- --config test-config.toml tags productivity
#[test]
fn test_tags_by_single_tag() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;
    
    // Setup
    initialize_database(&config, false, None)?;
    index_vault(&config, false, false, false, None)?;
    
    // Test finding notes by tag
    list_notes_by_tag(&config, &Some("learning".to_string()), false, None)?;
    list_notes_by_tag(&config, &Some("productivity".to_string()), false, None)?;
    
    Ok(())
}

// CLI equivalent: cargo run -- --config test-config.toml tags --all
#[test]
fn test_tags_list_all() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;
    
    // Setup
    initialize_database(&config, false, None)?;
    index_vault(&config, false, false, false, None)?;
    
    // Test listing all tags
    list_notes_by_tag(&config, &None, true, None)?;
    
    Ok(())
}

// CLI equivalent: cargo run -- --config test-config.toml tags nonexistenttag
#[test]
fn test_tags_nonexistent_tag() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;
    
    // Setup
    initialize_database(&config, false, None)?;
    index_vault(&config, false, false, false, None)?;
    
    // Test querying non-existent tag
    list_notes_by_tag(&config, &Some("nonexistenttag".to_string()), false, None)?;
    
    Ok(())
}

// CLI equivalent: cargo run -- --config test-config.toml tags learning
// CLI equivalent: cargo run -- --config test-config.toml tags productivity
#[test]
fn test_multiple_tags_per_note() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;
    
    // Setup
    initialize_database(&config, false, None)?;
    index_vault(&config, false, false, false, None)?;
    
    // Home.md has tags: [learning, productivity]
    // Should appear in both tag queries
    list_notes_by_tag(&config, &Some("learning".to_string()), false, None)?;
    list_notes_by_tag(&config, &Some("productivity".to_string()), false, None)?;
    
    Ok(())
}

// CLI equivalent: cargo run -- --config test-config.toml tags work
// CLI equivalent: cargo run -- --config test-config.toml tags creativity
#[test]
fn test_tags_various_categories() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;
    
    // Setup
    initialize_database(&config, false, None)?;
    index_vault(&config, false, false, false, None)?;
    
    // Test various tag categories
    list_notes_by_tag(&config, &Some("work".to_string()), false, None)?;
    list_notes_by_tag(&config, &Some("creativity".to_string()), false, None)?;
    
    Ok(())
}

// CLI equivalent: cargo run -- --config test-config.toml tags Learning
// CLI equivalent: cargo run -- --config test-config.toml tags LEARNING
#[test]
fn test_tags_case_sensitive() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;
    
    // Setup
    initialize_database(&config, false, None)?;
    index_vault(&config, false, false, false, None)?;
    
    // Test if tags are case-sensitive
    list_notes_by_tag(&config, &Some("Learning".to_string()), false, None)?;
    list_notes_by_tag(&config, &Some("LEARNING".to_string()), false, None)?;
    
    Ok(())
}
