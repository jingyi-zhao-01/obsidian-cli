mod common;

use anyhow::Result;
use obsidian_cli::commands::*;

#[test]
fn test_index_vault() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;
    
    // Initialize and index
    initialize_database(&config, false, None)?;
    index_vault(&config, false, false, false, None)?;
    
    // Verify database has data using stats
    show_stats(&config, None)?;
    
    Ok(())
}

#[test]
fn test_index_vault_dry_run() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;
    
    // Initialize database
    initialize_database(&config, false, None)?;
    
    // Test dry run (should not modify database)
    index_vault(&config, true, false, false, None)?;
    
    Ok(())
}

#[test]
fn test_index_vault_force() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;
    
    // Setup
    initialize_database(&config, false, None)?;
    index_vault(&config, false, false, false, None)?;
    
    // Force re-index
    index_vault(&config, false, true, false, None)?;
    
    Ok(())
}

#[test]
fn test_index_vault_verbose() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;
    
    // Setup
    initialize_database(&config, false, None)?;
    
    // Index with verbose output
    index_vault(&config, false, false, true, None)?;
    
    Ok(())
}
