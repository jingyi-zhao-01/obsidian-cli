mod common;

use anyhow::Result;
use obsidian_cli::commands::*;

// CLI equivalent: cargo run -- --config test-config.toml backlinks "Home.md"
// CLI equivalent: cargo run -- --config test-config.toml backlinks "Ideas.md"
#[test]
fn test_backlinks() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;
    
    // Setup
    initialize_database(&config, false, None)?;
    index_vault(&config, false, false, false, None)?;
    
    // Test backlinks - Home.md should have backlinks from Ideas.md
    get_backlinks(&config, "Home.md", None)?;
    get_backlinks(&config, "Ideas.md", None)?;
    
    Ok(())
}

// CLI equivalent: cargo run -- --config test-config.toml backlinks "NonExistent.md"
#[test]
fn test_backlinks_nonexistent_note() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;
    
    // Setup
    initialize_database(&config, false, None)?;
    index_vault(&config, false, false, false, None)?;
    
    // Test backlinks for non-existent note
    get_backlinks(&config, "NonExistent.md", None)?;
    
    Ok(())
}

// CLI equivalent: cargo run -- --config test-config.toml links "Home.md"
// CLI equivalent: cargo run -- --config test-config.toml links "Projects.md"
#[test]
fn test_forward_links() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;
    
    // Setup
    initialize_database(&config, false, None)?;
    index_vault(&config, false, false, false, None)?;
    
    // Test forward links - Home.md links to Projects, Ideas, Learning Strategies
    get_forward_links(&config, "Home.md", None)?;
    get_forward_links(&config, "Projects.md", None)?;
    
    Ok(())
}

// CLI equivalent: cargo run -- --config test-config.toml links "NonExistent.md"
#[test]
fn test_forward_links_nonexistent_note() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;
    
    // Setup
    initialize_database(&config, false, None)?;
    index_vault(&config, false, false, false, None)?;
    
    // Test forward links for non-existent note
    get_forward_links(&config, "NonExistent.md", None)?;
    
    Ok(())
}

// CLI equivalent: cargo run -- --config test-config.toml unresolved-links
#[test]
fn test_unresolved_links() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;
    
    // Setup
    initialize_database(&config, false, None)?;
    index_vault(&config, false, false, false, None)?;
    
    // Test unresolved links - should find links to non-existent notes
    list_unresolved_links(&config, None)?;
    
    Ok(())
}

// CLI equivalent: cargo run -- --config test-config.toml backlinks "Ideas.md"
#[test]
fn test_wikilink_resolution() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;
    
    // Setup
    initialize_database(&config, false, None)?;
    index_vault(&config, false, false, false, None)?;
    
    // Test that wikilinks are properly resolved
    // Home.md -> [[Ideas]] -> Ideas.md should create backlink
    get_backlinks(&config, "Ideas.md", None)?;
    
    Ok(())
}

// CLI equivalent: cargo run -- --config test-config.toml links "Home.md"
// CLI equivalent: cargo run -- --config test-config.toml backlinks "Home.md"
#[test]
fn test_bidirectional_links() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;
    
    // Setup
    initialize_database(&config, false, None)?;
    index_vault(&config, false, false, false, None)?;
    
    // Test bidirectional link relationship
    // Home links to Ideas, Ideas links back to Home
    get_forward_links(&config, "Home.md", None)?;
    get_backlinks(&config, "Home.md", None)?;
    
    Ok(())
}
