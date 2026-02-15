use anyhow::Result;
use obsidian_cli_inspector::config::{Config, ExcludeConfig, GraphConfig, SearchConfig};
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn test_config_from_file() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let config_path = temp_dir.path().join("config.toml");

    let config_content = r#"
vault_path = "/tmp/test-vault"

[exclude]
patterns = [".obsidian/", ".git/"]

[search]
default_limit = 30

[graph]
max_depth = 5
"#;

    fs::write(&config_path, config_content)?;

    let config = Config::from_file(&config_path)?;
    assert_eq!(config.vault_path, PathBuf::from("/tmp/test-vault"));
    assert_eq!(config.exclude.patterns.len(), 2);
    assert_eq!(config.search.default_limit, 30);
    assert_eq!(config.graph.max_depth, 5);

    Ok(())
}

#[test]
fn test_config_from_file_not_found() {
    let config_path = PathBuf::from("/nonexistent/path/config.toml");
    let result = Config::from_file(&config_path);
    assert!(result.is_err());
}

#[test]
fn test_config_from_invalid_toml() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let config_path = temp_dir.path().join("config.toml");

    // Invalid TOML content
    fs::write(&config_path, "invalid toml [[[")?;

    let result = Config::from_file(&config_path);
    assert!(result.is_err());

    Ok(())
}

#[test]
fn test_config_database_path_default() {
    let config = Config {
        vault_path: PathBuf::from("/tmp/vault"),
        database_path: None,
        log_path: None,
        exclude: ExcludeConfig::default(),
        search: SearchConfig::default(),
        graph: GraphConfig::default(),
        llm: None,
    };

    let db_path = config.database_path();
    assert!(db_path.to_string_lossy().contains("obsidian-cli-inspector"));
    assert!(db_path.to_string_lossy().ends_with("index.db"));
}

#[test]
fn test_config_database_path_custom() {
    let custom_db = PathBuf::from("/custom/path/db.db");
    let config = Config {
        vault_path: PathBuf::from("/tmp/vault"),
        database_path: Some(custom_db.clone()),
        log_path: None,
        exclude: ExcludeConfig::default(),
        search: SearchConfig::default(),
        graph: GraphConfig::default(),
        llm: None,
    };

    let db_path = config.database_path();
    assert_eq!(db_path, custom_db);
}

#[test]
fn test_config_config_dir() {
    let config = Config {
        vault_path: PathBuf::from("/tmp/vault"),
        database_path: None,
        log_path: None,
        exclude: ExcludeConfig::default(),
        search: SearchConfig::default(),
        graph: GraphConfig::default(),
        llm: None,
    };

    let config_dir = config.config_dir();
    assert!(config_dir
        .to_string_lossy()
        .ends_with("obsidian-cli-inspector"));
}

#[test]
fn test_config_log_dir_default() {
    let config = Config {
        vault_path: PathBuf::from("/tmp/vault"),
        database_path: None,
        log_path: None,
        exclude: ExcludeConfig::default(),
        search: SearchConfig::default(),
        graph: GraphConfig::default(),
        llm: None,
    };

    let log_dir = config.log_dir();
    assert!(log_dir.to_string_lossy().contains("obsidian-cli-inspector"));
    assert!(log_dir.to_string_lossy().ends_with("logs"));
}

#[test]
fn test_config_log_dir_custom() {
    let custom_log = PathBuf::from("/custom/logs");
    let config = Config {
        vault_path: PathBuf::from("/tmp/vault"),
        database_path: None,
        log_path: Some(custom_log.clone()),
        exclude: ExcludeConfig::default(),
        search: SearchConfig::default(),
        graph: GraphConfig::default(),
        llm: None,
    };

    let log_dir = config.log_dir();
    assert_eq!(log_dir, custom_log);
}

#[test]
fn test_search_config_default() {
    let config = SearchConfig::default();
    assert_eq!(config.default_limit, 20);
}

#[test]
fn test_graph_config_default() {
    let config = GraphConfig::default();
    assert_eq!(config.max_depth, 3);
}

#[test]
fn test_exclude_config_default() {
    let config = ExcludeConfig::default();
    // ExcludeConfig::default() returns empty patterns vec (default_exclude_patterns is only used via serde)
    assert_eq!(config.patterns.len(), 0);
}

#[test]
fn test_exclude_config_from_toml() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let config_path = temp_dir.path().join("config.toml");

    // Test that default_exclude_patterns is used when patterns field is missing but exclude is present
    let config_content = r#"
vault_path = "/tmp/test-vault"

[exclude]
"#;

    fs::write(&config_path, config_content)?;
    let config = Config::from_file(&config_path)?;
    // When exclude section exists but patterns field is missing, serde default kicks in
    assert_eq!(config.exclude.patterns.len(), 3);
    assert!(config.exclude.patterns.contains(&".obsidian/".to_string()));
    assert!(config.exclude.patterns.contains(&".git/".to_string()));
    assert!(config.exclude.patterns.contains(&".trash/".to_string()));

    Ok(())
}

#[test]
fn test_config_with_llm() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let config_path = temp_dir.path().join("config.toml");

    let config_content = r#"
vault_path = "/tmp/test-vault"

[llm]
api_url = "http://localhost:11434"
model = "llama2"
timeout_seconds = 60
"#;

    fs::write(&config_path, config_content)?;

    let config = Config::from_file(&config_path)?;
    assert!(config.llm.is_some());

    let llm = config.llm.unwrap();
    assert_eq!(llm.api_url, "http://localhost:11434");
    assert_eq!(llm.model, "llama2");
    assert_eq!(llm.timeout_seconds, 60);

    Ok(())
}

#[test]
fn test_config_llm_default_timeout() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let config_path = temp_dir.path().join("config.toml");

    let config_content = r#"
vault_path = "/tmp/test-vault"

[llm]
api_url = "http://localhost:11434"
model = "llama2"
"#;

    fs::write(&config_path, config_content)?;

    let config = Config::from_file(&config_path)?;
    assert!(config.llm.is_some());

    let llm = config.llm.unwrap();
    assert_eq!(llm.timeout_seconds, 30);

    Ok(())
}
