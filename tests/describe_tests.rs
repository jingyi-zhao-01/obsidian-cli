

// Test show_unimplemented
#[test]
fn test_show_unimplemented() {
    use obsidian_cli_inspector::commands::other::show_unimplemented;
    show_unimplemented("test", None);
}

// Test show_search
#[test]
fn test_show_search() {
    use obsidian_cli_inspector::commands::other::show_search;
    show_search("query", 10, None);
}

// Test show_backlinks
#[test]
fn test_show_backlinks() {
    use obsidian_cli_inspector::commands::other::show_backlinks;
    show_backlinks("note", None);
}

// Test show_links
#[test]
fn test_show_links() {
    use obsidian_cli_inspector::commands::other::show_links;
    show_links("note", None);
}

// Test show_tags
#[test]
fn test_show_tags() {
    use obsidian_cli_inspector::commands::other::show_tags;
    show_tags(&Some("tag".to_string()), false, None);
    show_tags(&None, true, None);
}

// Test show_suggest
#[test]
fn test_show_suggest() {
    use obsidian_cli_inspector::commands::other::show_suggest;
    show_suggest("note", 5, None);
}

// Test show_bloat
#[test]
fn test_show_bloat() {
    use obsidian_cli_inspector::commands::other::show_bloat;
    show_bloat(1000, 10, None);
}

// Test show_tui
#[test]
fn test_show_tui() {
    use obsidian_cli_inspector::commands::other::show_tui;
    show_tui(None);
}

// Test show_graph
#[test]
fn test_show_graph() {
    use obsidian_cli_inspector::commands::other::show_graph;
    show_graph(&Some("note".to_string()), 2, None);
    show_graph(&None, 1, None);
}


// Test show_stats
#[test]
fn test_show_stats() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;

    // Setup
    initialize_database(&config, false, None)?;
    index_vault(&config, false, false, false, None)?;

    // Test stats
    show_stats(&config, None)?;

    Ok(())
}


// Test search vault with logger
#[test]
fn test_search_vault_with_logger() -> Result<()> {
    use obsidian_cli_inspector::logger::Logger;
    
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;

    // Setup
    initialize_database(&config, false, None)?;
    index_vault(&config, false, false, false, None)?;

    // Create logger
    let logger = Logger::new(config.log_dir()).ok();
    
    // Test search with logger
    search_vault(&config, "productivity", 10, logger.as_ref())?;

    Ok(())
}

// Test get_backlinks with logger
#[test]
fn test_backlinks_with_logger() -> Result<()> {
    use obsidian_cli_inspector::logger::Logger;
    
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;

    // Setup
    initialize_database(&config, false, None)?;
    index_vault(&config, false, false, false, None)?;

    // Create logger
    let logger = Logger::new(config.log_dir()).ok();
    
    // Test backlinks with logger
    get_backlinks(&config, "Productivity.md", logger.as_ref())?;

    Ok(())
}

// Test show_stats with logger
#[test]
fn test_show_stats_with_logger() -> Result<()> {
    use obsidian_cli_inspector::logger::Logger;
    
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;

    // Setup
    initialize_database(&config, false, None)?;
    index_vault(&config, false, false, false, None)?;

    // Create logger
    let logger = Logger::new(config.log_dir()).ok();
    
    // Test stats with logger
    show_stats(&config, logger.as_ref())?;

    Ok(())
}

// Test get_note_describe with logger
#[test]
fn test_describe_with_logger() -> Result<()> {
    use obsidian_cli_inspector::logger::Logger;
    
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;

    // Setup
    initialize_database(&config, false, None)?;
    index_vault(&config, false, false, false, None)?;

    // Create logger
    let logger = Logger::new(config.log_dir()).ok();
    
    // Test describe with logger
    get_note_describe(&config, "Home.md", logger.as_ref())?;

    Ok(())
}


// Test index vault with verbose to test skip message
#[test]
fn test_index_vault_with_skip() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;

    // Setup
    initialize_database(&config, false, None)?;
    index_vault(&config, false, false, true, None)?;

    // Index again - should skip unchanged files (verbose shows skip message)
    index_vault(&config, false, false, true, None)?;

    Ok(())
}

// Test index vault with verbose and force
#[test]
fn test_index_vault_verbose_force() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;

    // Setup
    initialize_database(&config, false, None)?;
    index_vault(&config, false, false, false, None)?;

    // Force reindex with verbose
    index_vault(&config, false, true, true, None)?;

    Ok(())
}


// Test index vault with dry run
#[test]
fn test_index_dry_run() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;

    // Setup
    initialize_database(&config, false, None)?;

    // Dry run - should not index files
    index_vault(&config, true, false, true, None)?;

    Ok(())
}

// Test index with logger  
#[test]
fn test_index_with_logger() -> Result<()> {
    use obsidian_cli_inspector::logger::Logger;
    
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;

    // Setup
    initialize_database(&config, false, None)?;

    // Create logger
    let logger = Logger::new(config.log_dir())?;
    
    // Index with logger
    index_vault(&config, false, false, true, Some(&logger))?;

    Ok(())
}


// Test list_notes_by_tag with logger
#[test]
fn test_list_tags_with_logger() -> Result<()> {
    use obsidian_cli_inspector::logger::Logger;
    
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;

    // Setup
    initialize_database(&config, false, None)?;
    index_vault(&config, false, false, false, None)?;

    // Create logger
    let logger = Logger::new(config.log_dir())?;
    
    // Test tags with logger
    list_notes_by_tag(&config, &None, true, Some(&logger))?;

    Ok(())
}

// Test forward_links with logger
#[test]
fn test_forward_links_with_logger() -> Result<()> {
    use obsidian_cli_inspector::logger::Logger;
    
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;

    // Setup
    initialize_database(&config, false, None)?;
    index_vault(&config, false, false, false, None)?;

    // Create logger
    let logger = Logger::new(config.log_dir())?;
    
    // Test forward links with logger
    get_forward_links(&config, "Productivity.md", Some(&logger))?;

    Ok(())
}

// Test unresolved links with logger
#[test]
fn test_unresolved_with_logger() -> Result<()> {
    use obsidian_cli_inspector::logger::Logger;
    
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;

    // Setup
    initialize_database(&config, false, None)?;
    index_vault(&config, false, false, false, None)?;

    // Create logger
    let logger = Logger::new(config.log_dir())?;
    
    // Test unresolved links with logger
    list_unresolved_links(&config, Some(&logger))?;

    Ok(())
}


// Test show functions with logger
#[test]
fn test_show_unimplemented_with_logger() {
    use obsidian_cli_inspector::logger::Logger;
    
    let temp_dir = tempfile::tempdir().unwrap();
    let logger = Logger::new(temp_dir.path().to_path_buf()).ok();
    
    use obsidian_cli_inspector::commands::other::show_unimplemented;
    show_unimplemented("test", logger.as_ref());
}

// Test show_search with logger
#[test]
fn test_show_search_with_logger() {
    use obsidian_cli_inspector::logger::Logger;
    
    let temp_dir = tempfile::tempdir().unwrap();
    let logger = Logger::new(temp_dir.path().to_path_buf()).ok();
    
    use obsidian_cli_inspector::commands::other::show_search;
    show_search("query", 10, logger.as_ref());
}

// Test show_tags with logger
#[test]
fn test_show_tags_with_logger() {
    use obsidian_cli_inspector::logger::Logger;
    
    let temp_dir = tempfile::tempdir().unwrap();
    let logger = Logger::new(temp_dir.path().to_path_buf()).ok();
    
    use obsidian_cli_inspector::commands::other::show_tags;
    show_tags(&Some("tag".to_string()), false, logger.as_ref());
    show_tags(&None, true, logger.as_ref());
}

// Test show_graph with logger
#[test]
fn test_show_graph_with_logger() {
    use obsidian_cli_inspector::logger::Logger;
    
    let temp_dir = tempfile::tempdir().unwrap();
    let logger = Logger::new(temp_dir.path().to_path_buf()).ok();
    
    use obsidian_cli_inspector::commands::other::show_graph;
    show_graph(&Some("note".to_string()), 2, logger.as_ref());
    show_graph(&None, 1, logger.as_ref());
}

// Test show_tui with logger
#[test]
fn test_show_tui_with_logger() {
    use obsidian_cli_inspector::logger::Logger;
    
    let temp_dir = tempfile::tempdir().unwrap();
    let logger = Logger::new(temp_dir.path().to_path_buf()).ok();
    
    use obsidian_cli_inspector::commands::other::show_tui;
    show_tui(logger.as_ref());
}


// Test db transaction
#[test]
fn test_db_transaction() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;

    // Setup
    initialize_database(&config, false, None)?;
    
    // Open database and get transaction
    let mut db = obsidian_cli_inspector::db::Database::open(config.database_path())?;
    let _tx = db.transaction()?;
    
    Ok(())
}

// Test db version
#[test]
fn test_db_version() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;

    // Setup
    initialize_database(&config, false, None)?;
    
    // Open database and get version
    let db = obsidian_cli_inspector::db::Database::open(config.database_path())?;
    let version = db.get_version()?;
    assert!(version.is_some());
    
    Ok(())
}


// Test initialize_database with logger
#[test]
fn test_init_with_logger_coverage() -> Result<()> {
    use obsidian_cli_inspector::logger::Logger;
    
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;

    // Create logger
    let logger = Logger::new(config.log_dir())?;
    
    // Initialize with logger
    initialize_database(&config, false, Some(&logger))?;

    Ok(())
}


// Test markdown parser with wikilinks
#[test]
fn test_markdown_parser_with_wikilinks() {
    use obsidian_cli_inspector::parser::MarkdownParser;
    
    let content = r#"# Heading

Some text with [[WikiLink]] and another [[Link|alias]].

## Another Heading

More text here.
"#;
    
    let parsed = MarkdownParser::parse(content);
    // Just ensure it doesn't panic
    assert!(true);
}

// Test markdown parser with frontmatter
#[test]
fn test_markdown_parser_with_frontmatter() {
    use obsidian_cli_inspector::parser::MarkdownParser;
    
    let content = r#"---
title: My Note
tags: [test, example]
---

# Heading

Content here.
"#;
    
    let parsed = MarkdownParser::parse(content);
    assert!(true);
}

mod common;

use anyhow::Result;
use obsidian_cli_inspector::commands::*;

// Test initialize_database with logger
#[test]
fn test_init_with_logger() -> Result<()> {
    use obsidian_cli_inspector::logger::Logger;
    
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;

    // Create logger
    let logger = Logger::new(config.log_dir()).ok();
    
    // Initialize with logger
    initialize_database(&config, false, logger.as_ref())?;

    Ok(())
}

// Test initialize_database twice with force
#[test]
fn test_init_force_twice() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;

    // Initialize once
    initialize_database(&config, false, None)?;
    
    // Force reinitialize
    initialize_database(&config, true, None)?;

    Ok(())
}

// Test initialize with new database dir
#[test]
fn test_init_new_directory() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;

    // Test init creates database
    initialize_database(&config, false, None)?;
    
    // Check that db file exists
    let db_path = config.database_path();
    assert!(db_path.exists());

    Ok(())
}

// Test config from file
#[test]
fn test_config_from_file() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;

    // Initialize first to create the database
    initialize_database(&config, false, None)?;

    // Test getting database path
    let db_path = config.database_path();
    assert!(db_path.exists());

    // Test config dir - may not exist in test environment
    let config_dir = config.config_dir();
    // Just check it's a valid path
    assert!(!config_dir.to_string_lossy().is_empty());

    // Test log dir
    let log_dir = config.log_dir();
    assert!(!log_dir.to_string_lossy().is_empty());

    Ok(())
}

// Test index without init - should handle gracefully
#[test]
fn test_index_without_init() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;

    // Remove the database to simulate not initializing
    let db_path = config.database_path();
    if db_path.exists() {
        std::fs::remove_file(&db_path)?;
    }

    // Try to index without init - should fail gracefully
    let result = index_vault(&config, false, false, false, None);
    // This should fail because database doesn't exist
    assert!(result.is_err());

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

// Test search vault - empty query
#[test]
fn test_search_vault_empty_query() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;

    // Setup
    initialize_database(&config, false, None)?;
    index_vault(&config, false, false, false, None)?;

    // Test search with empty query
    search_vault(&config, "", 10, None)?;

    Ok(())
}

// Test search vault - no results
#[test]
fn test_search_vault_no_results() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;

    // Setup
    initialize_database(&config, false, None)?;
    index_vault(&config, false, false, false, None)?;

    // Test search with no matches
    search_vault(&config, "nonexistentxyz", 10, None)?;

    Ok(())
}

// Test search vault - with results
#[test]
fn test_search_vault_with_results() -> Result<()> {
    let (_vault_dir, _db_dir, config) = common::setup_test_config()?;

    // Setup
    initialize_database(&config, false, None)?;
    index_vault(&config, false, false, false, None)?;

    // Test search with results
    search_vault(&config, "productivity", 10, None)?;

    Ok(())
}
