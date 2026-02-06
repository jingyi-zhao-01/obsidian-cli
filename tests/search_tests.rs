mod common;

use anyhow::Result;
use obsidian_cli::commands::*;

#[test]
fn test_search_vault() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;
    
    // Setup
    initialize_database(&config, false, None)?;
    index_vault(&config, false, false, false, None)?;
    
    // Test search
    search_vault(&config, "productivity", 10, None)?;
    search_vault(&config, "learning", 10, None)?;
    
    Ok(())
}

#[test]
fn test_search_empty_query() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;
    
    // Setup
    initialize_database(&config, false, None)?;
    index_vault(&config, false, false, false, None)?;
    
    // Test empty query (should handle gracefully)
    search_vault(&config, "", 10, None)?;
    
    Ok(())
}

#[test]
fn test_search_no_results() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;
    
    // Setup
    initialize_database(&config, false, None)?;
    index_vault(&config, false, false, false, None)?;
    
    // Test query with no matches
    search_vault(&config, "xyznonexistentterm", 10, None)?;
    
    Ok(())
}

#[test]
fn test_search_with_limit() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;
    
    // Setup
    initialize_database(&config, false, None)?;
    index_vault(&config, false, false, false, None)?;
    
    // Test search with different limits
    search_vault(&config, "learning", 1, None)?;
    search_vault(&config, "learning", 5, None)?;
    search_vault(&config, "learning", 100, None)?;
    
    Ok(())
}

#[test]
fn test_search_various_terms() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;
    
    // Setup
    initialize_database(&config, false, None)?;
    index_vault(&config, false, false, false, None)?;
    
    // Test various search terms
    search_vault(&config, "vault", 10, None)?;
    search_vault(&config, "strategies", 10, None)?;
    search_vault(&config, "work", 10, None)?;
    
    Ok(())
}
