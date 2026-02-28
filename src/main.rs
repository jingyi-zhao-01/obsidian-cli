use anyhow::{Context, Result};
use clap::Parser;
use obsidian_cli_inspector::{
    cli::{Cli, Commands},
    commands::*,
    config::Config,
    logger::Logger,
};
use std::path::PathBuf;
use std::time::Instant;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let config = load_config(cli.config.clone()).ok();
    let logger = if let Some(ref cfg) = config {
        Logger::new(cfg.log_dir()).ok()
    } else {
        None
    };

    let start = Instant::now();
    let (command_name, result) = match cli.command {
        Commands::Init { force } => {
            // Prompt the user to confirm/override vault/database/log locations and persist them
            let config = interactive_config_setup(cli.config)?;

            // Create a logger using the (possibly updated) config so init logs go to the right place
            let cmd_logger = Logger::new(config.log_dir()).ok();
            if let Some(ref log) = cmd_logger {
                let _ = log.log_section("init", "Starting Init Command");
            }

            (
                "init",
                initialize_database(&config, force, cmd_logger.as_ref()),
            )
        }
        Commands::Stats => {
            let config = load_config(cli.config)?;
            if let Some(ref log) = logger {
                let _ = log.log_section("stats", "Starting Stats Command");
            }
            ("stats", show_stats(&config, logger.as_ref()))
        }
        Commands::Index {
            dry_run,
            force,
            verbose,
        } => {
            let config = load_config(cli.config)?;
            if let Some(ref log) = logger {
                let _ = log.log_section("index", "Starting Index Command");
            }
            (
                "index",
                index_vault(&config, dry_run, force, verbose, logger.as_ref()),
            )
        }
        Commands::Search { query, limit } => {
            let config = load_config(cli.config)?;
            if let Some(ref log) = logger {
                let _ = log.log_section("search", "Starting Search Command");
            }
            (
                "search",
                search_vault(&config, &query, limit, logger.as_ref()),
            )
        }
        Commands::Backlinks { note } => {
            let config = load_config(cli.config)?;
            if let Some(ref log) = logger {
                let _ = log.log_section("backlinks", "Starting Backlinks Command");
            }
            ("backlinks", get_backlinks(&config, &note, logger.as_ref()))
        }
        Commands::Links { note } => {
            let config = load_config(cli.config)?;
            if let Some(ref log) = logger {
                let _ = log.log_section("links", "Starting Links Command");
            }
            ("links", get_forward_links(&config, &note, logger.as_ref()))
        }
        Commands::UnresolvedLinks => {
            let config = load_config(cli.config)?;
            if let Some(ref log) = logger {
                let _ = log.log_section("unresolved", "Starting Unresolved Links Command");
            }
            (
                "unresolved-links",
                list_unresolved_links(&config, logger.as_ref()),
            )
        }
        Commands::Tags { tag, all } => {
            let config = load_config(cli.config)?;
            if let Some(ref log) = logger {
                let _ = log.log_section("tags", "Starting Tags Command");
            }
            (
                "tags",
                list_notes_by_tag(&config, &tag, all, logger.as_ref()),
            )
        }
        Commands::Suggest { note, limit } => {
            show_suggest(&note, limit, logger.as_ref());
            ("suggest", Ok(()))
        }
        Commands::Bloat { threshold, limit } => {
            show_bloat(threshold, limit, logger.as_ref());
            ("bloat", Ok(()))
        }
        Commands::Tui => {
            show_tui(logger.as_ref());
            ("tui", Ok(()))
        }
        Commands::Graph { note, depth } => {
            show_graph(&note, depth, logger.as_ref());
            ("graph", Ok(()))
        }
        Commands::Describe { filename } => {
            let config = load_config(cli.config)?;
            if let Some(ref log) = logger {
                let _ = log.log_section("describe", "Starting Describe Command");
            }
            (
                "describe",
                get_note_describe(&config, &filename, logger.as_ref()),
            )
        }
        Commands::DiagnoseOrphans {
            exclude_templates,
            exclude_daily,
        } => {
            let config = load_config(cli.config)?;
            if let Some(ref log) = logger {
                let _ = log.log_section("diagnose-orphans", "Starting Diagnose Orphans Command");
            }
            (
                "diagnose-orphans",
                diagnose_orphans(&config, exclude_templates, exclude_daily, logger.as_ref()),
            )
        }
        Commands::DiagnoseBrokenLinks => {
            let config = load_config(cli.config)?;
            if let Some(ref log) = logger {
                let _ = log.log_section(
                    "diagnose-broken-links",
                    "Starting Diagnose Broken Links Command",
                );
            }
            (
                "diagnose-broken-links",
                diagnose_broken_links_cmd(&config, logger.as_ref()),
            )
        }
    };
    let elapsed = start.elapsed();
    if result.is_ok() {
        println!("Command '{command_name}' completed in {elapsed:.2?}");
    } else {
        eprintln!("Command '{command_name}' failed after {elapsed:.2?}");
    }

    result
}

/// Default config template that will be seeded on first run
const DEFAULT_CONFIG: &str = include_str!("../template-config.toml");

fn ensure_config_exists(path: &PathBuf) -> Result<PathBuf> {
    // Skip creation if config already exists
    if !path.exists() {
        // Create parent directories if they don't exist
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).context("Failed to create config directory")?;
        }

        // Write default config
        std::fs::write(path, DEFAULT_CONFIG).context("Failed to write default config file")?;

        println!(
            "Created default config at: {}\n\
             Please edit this file and set your vault_path.",
            path.display()
        );
    }

    Ok(path.clone())
}

/// Ensure the config file exists and return its path (does not parse the file)
fn config_file_path(config_path: Option<PathBuf>) -> Result<PathBuf> {
    let path = config_path.unwrap_or_else(|| {
        let mut p = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        p.push("obsidian-cli-inspector");
        p.push("config.toml");
        p
    });

    ensure_config_exists(&path)
}

/// Interactively prompt the user to confirm or override key config values and persist them.
/// Prompts: vault_path, database_path, log_path — each shows a sensible default and accepts ENTER to keep it.
fn interactive_config_setup(config_path: Option<PathBuf>) -> Result<Config> {
    use std::io::{self, Write};

    let config_file = config_file_path(config_path)?;

    // Load current config (template will already contain a placeholder)
    let mut cfg = Config::from_file(&config_file).context("Failed to parse config file")?;

    // 1) Vault path (required)
    let current_vault = cfg.vault_path.to_string_lossy();
    print!("Vault path [{current_vault}]: ");
    io::stdout().flush()?;
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input)?;
    let val = input.trim();
    if !val.is_empty() {
        cfg.vault_path = PathBuf::from(val);
    }

    // 2) Database path (optional) — show explicit default
    let db_default = cfg
        .database_path
        .clone()
        .unwrap_or_else(|| cfg.database_path());
    print!("Database path [{}]: ", db_default.display());
    io::stdout().flush()?;
    input.clear();
    let _ = io::stdin().read_line(&mut input)?;
    let val = input.trim();
    if !val.is_empty() {
        cfg.database_path = Some(PathBuf::from(val));
    } else {
        // store the explicit default so config.toml contains the concrete path
        cfg.database_path = Some(db_default);
    }

    // 3) Log path (optional) — show explicit default
    let log_default = cfg.log_path.clone().unwrap_or_else(|| cfg.log_dir());
    print!("Log path [{}]: ", log_default.display());
    io::stdout().flush()?;
    input.clear();
    let _ = io::stdin().read_line(&mut input)?;
    let val = input.trim();
    if !val.is_empty() {
        cfg.log_path = Some(PathBuf::from(val));
    } else {
        // store the explicit default so config.toml contains the concrete path
        cfg.log_path = Some(log_default);
    }

    // Persist updated config back to disk (overwrite)
    let toml = toml::to_string_pretty(&cfg).context("Failed to serialize config to TOML")?;
    std::fs::write(&config_file, toml).context("Failed to write updated config file")?;

    println!("Updated config at: {}", config_file.display());

    Ok(cfg)
}

fn load_config(config_path: Option<PathBuf>) -> Result<Config> {
    let path = config_path.unwrap_or_else(|| {
        let mut p = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        p.push("obsidian-cli-inspector");
        p.push("config.toml");
        p
    });

    // Ensure config file exists (create default if needed)
    let config_file_path = ensure_config_exists(&path)?;

    Config::from_file(&config_file_path).context("Failed to load config file")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_default_config_includes_vault_path_placeholder() {
        // Verify the template contains the vault_path placeholder
        assert!(DEFAULT_CONFIG.contains("vault_path"));
        assert!(DEFAULT_CONFIG.contains("\"/path/to/your/obsidian/vault\""));
    }

    #[test]
    fn test_default_config_includes_all_sections() {
        // Verify all expected config sections are present
        assert!(DEFAULT_CONFIG.contains("[exclude]"));
        assert!(DEFAULT_CONFIG.contains("[search]"));
        assert!(DEFAULT_CONFIG.contains("[graph]"));
    }

    #[test]
    fn test_ensure_config_returns_existing_path() {
        // Create a temp directory with an existing config file
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");
        fs::write(&config_path, "vault_path = \"/test/vault\"").unwrap();

        let result = ensure_config_exists(&config_path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), config_path);
    }

    #[test]
    fn test_ensure_config_creates_default_config() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");

        let result = ensure_config_exists(&config_path);
        assert!(result.is_ok());
        assert!(config_path.exists());

        let content = fs::read_to_string(&config_path).unwrap();
        assert!(content.contains("vault_path"));
    }

    #[test]
    fn test_ensure_config_creates_parent_directories() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("subdir").join("config.toml");

        let result = ensure_config_exists(&config_path);
        assert!(result.is_ok());
        assert!(config_path.exists());
    }

    #[test]
    fn test_ensure_config_does_not_overwrite_existing() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");
        let original_content = "vault_path = \"/my/custom/vault\"";
        fs::write(&config_path, original_content).unwrap();

        let result = ensure_config_exists(&config_path);
        assert!(result.is_ok());

        let content = fs::read_to_string(&config_path).unwrap();
        assert_eq!(content, original_content);
    }

    #[test]
    fn test_load_config_with_existing_file() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");
        fs::write(
            &config_path,
            r#"vault_path = "/test/vault"
[search]
default_limit = 50
"#,
        )
        .unwrap();

        let config = load_config(Some(config_path)).unwrap();
        assert_eq!(config.vault_path.to_string_lossy(), "/test/vault");
        assert_eq!(config.search.default_limit, 50);
    }

    #[test]
    fn test_load_config_creates_default_when_missing() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");

        // Should create default config
        let result = load_config(Some(config_path.clone()));
        assert!(result.is_ok());
        assert!(config_path.exists());
    }

    #[test]
    fn test_default_config_has_search_section() {
        // Verify search config section is present
        assert!(DEFAULT_CONFIG.contains("[search]"));
        assert!(DEFAULT_CONFIG.contains("default_limit"));
    }

    #[test]
    fn test_default_config_has_exclude_section() {
        // Verify exclude config section is present
        assert!(DEFAULT_CONFIG.contains("[exclude]"));
        assert!(DEFAULT_CONFIG.contains("patterns"));
    }
}
