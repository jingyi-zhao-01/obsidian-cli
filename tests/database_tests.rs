mod common;

use anyhow::Result;
use obsidian_cli::commands::*;

#[test]
fn test_init_database() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;
    
    // Initialize database
    initialize_database(&config, false, None)?;
    
    // Verify database exists
    assert!(config.database_path().exists());
    
    Ok(())
}

#[test]
fn test_reinit_database() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;
    
    // Initialize database
    initialize_database(&config, false, None)?;
    index_vault(&config, false, false, false, None)?;
    
    // Force reinitialize (should clear data)
    initialize_database(&config, true, None)?;
    
    // Verify database still exists
    assert!(config.database_path().exists());
    
    Ok(())
}

#[test]
fn test_stats() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;
    
    // Setup
    initialize_database(&config, false, None)?;
    index_vault(&config, false, false, false, None)?;
    
    // Test stats
    show_stats(&config, None)?;
    
    Ok(())
}
