use anyhow::Result;
use obsidian_cli_inspector::config::Config;
use std::fs;
use tempfile::TempDir;

/// Helper function to create a test configuration with temporary directories
pub fn setup_test_config() -> Result<(TempDir, TempDir, Config)> {
    let vault_dir = TempDir::new()?;
    let db_dir = TempDir::new()?;

    // Create test notes
    fs::write(
        vault_dir.path().join("Home.md"),
        "---\ntags: [learning, productivity]\n---\n# Home\n\nWelcome to my vault.\n\n[[Projects]]\n[[Ideas]]\n[[Learning Strategies]]"
    )?;

    fs::write(
        vault_dir.path().join("Projects.md"),
        "---\ntags: [work, productivity]\n---\n# Projects\n\nCurrent projects:\n- [[Deep Work]]\n- [[Software Architecture]]"
    )?;

    fs::write(
        vault_dir.path().join("Ideas.md"),
        "---\ntags: [creativity]\n---\n# Ideas\n\nRandom thoughts and ideas.\n\n[[Home]]",
    )?;

    fs::write(
        vault_dir.path().join("Learning Strategies.md"),
        "---\ntags: [learning]\n---\n# Learning Strategies\n\nEffective learning techniques.\n\n[[Zettelkasten Method]]\n[[Pomodoro Technique]]"
    )?;

    fs::write(
        vault_dir.path().join("Deep Work.md"),
        "# Deep Work\n\nFocus and productivity strategies.",
    )?;

    let config = Config {
        vault_path: vault_dir.path().to_path_buf(),
        database_path: Some(db_dir.path().join("test.db")),
        log_path: Some(db_dir.path().join("logs")),
        exclude: Default::default(),
        search: Default::default(),
        graph: Default::default(),
        llm: None,
    };

    Ok((vault_dir, db_dir, config))
}
