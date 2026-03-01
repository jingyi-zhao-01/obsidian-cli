use anyhow::{Context, Result};
use clap::Parser;
use obsidian_cli_inspector::{
    cli::{
        AnalyzeCommands, Cli, Commands, DiagnoseCommands, IndexCommands, InitCommands,
        QueryCommands, ViewCommands,
    },
    commands::*,
    config::Config,
    db::Database,
    logger::Logger,
    query,
};
use std::path::PathBuf;
use std::time::Instant;

/// Check if JSON output is requested
fn is_json_output(output: &Option<String>) -> bool {
    output.as_ref().map(|s| s.to_lowercase()).as_deref() == Some("json")
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let is_json = is_json_output(&cli.output);

    let config = load_config(cli.config.clone()).ok();
    let logger = if let Some(ref cfg) = config {
        Logger::new(cfg.log_dir()).ok()
    } else {
        None
    };

    let start = Instant::now();
    let (command_name, result) = match cli.command {
        // ============================================================================
        // INIT Commands
        // ============================================================================
        Commands::Init(InitCommands::Init { force }) => {
            // Try to load existing config first, only prompt if it doesn't exist or is invalid
            let config = match load_config(cli.config.clone()) {
                Ok(cfg) => cfg,
                Err(_) => {
                    // Config doesn't exist or is invalid - do interactive setup
                    interactive_config_setup(cli.config)?
                }
            };

            // Create a logger using the (possibly updated) config so init logs go to the right place
            let cmd_logger = Logger::new(config.log_dir()).ok();
            if let Some(ref log) = cmd_logger {
                let _ = log.log_section("init", "Starting Init Command");
            }

            (
                "init.init",
                initialize_database(&config, force, cmd_logger.as_ref()),
            )
        }

        // ============================================================================
        // INDEX Commands
        // ============================================================================
        Commands::Index(IndexCommands::Index {
            dry_run,
            force,
            verbose,
        }) => {
            let config = load_config(cli.config)?;
            if let Some(ref log) = logger {
                let _ = log.log_section("index", "Starting Index Command");
            }
            (
                "index.index",
                index_vault(&config, dry_run, force, verbose, logger.as_ref()),
            )
        }

        // ============================================================================
        // QUERY Commands
        // ============================================================================
        Commands::Query(QueryCommands::Search { query, limit }) => {
            let config = load_config(cli.config)?;
            if let Some(ref log) = logger {
                let _ = log.log_section("query.search", "Starting Search Command");
            }
            (
                "query.search",
                search_vault(&config, &query, limit, logger.as_ref()),
            )
        }
        Commands::Query(QueryCommands::Backlinks { note }) => {
            let config = load_config(cli.config)?;
            if let Some(ref log) = logger {
                let _ = log.log_section("query.backlinks", "Starting Backlinks Command");
            }
            (
                "query.backlinks",
                get_backlinks(&config, &note, logger.as_ref()),
            )
        }
        Commands::Query(QueryCommands::Links { note }) => {
            let config = load_config(cli.config)?;
            if let Some(ref log) = logger {
                let _ = log.log_section("query.links", "Starting Links Command");
            }
            (
                "query.links",
                get_forward_links(&config, &note, logger.as_ref()),
            )
        }
        Commands::Query(QueryCommands::Unresolved) => {
            let config = load_config(cli.config)?;
            if let Some(ref log) = logger {
                let _ = log.log_section("query.unresolved", "Starting Unresolved Links Command");
            }
            (
                "query.unresolved",
                list_unresolved_links(&config, logger.as_ref()),
            )
        }
        Commands::Query(QueryCommands::Tags { tag, list }) => {
            let config = load_config(cli.config)?;
            if let Some(ref log) = logger {
                let _ = log.log_section("query.tags", "Starting Tags Command");
            }
            // Convert `list` flag to `all` for the underlying function
            (
                "query.tags",
                list_notes_by_tag(&config, &tag, list, logger.as_ref()),
            )
        }

        // ============================================================================
        // ANALYZE Commands
        // ============================================================================
        Commands::Analyze(AnalyzeCommands::Bloat { threshold, limit }) => {
            let _config = load_config(cli.config)?;
            if let Some(ref log) = logger {
                let _ = log.log_section("analyze.bloat", "Starting Bloat Command");
            }
            show_bloat(threshold, limit, logger.as_ref());
            ("analyze.bloat", Ok(()))
        }
        Commands::Analyze(AnalyzeCommands::Related { note: _, limit: _ }) => {
            let _config = load_config(cli.config)?;
            if let Some(ref log) = logger {
                let _ = log.log_section("analyze.related", "Starting Related Command");
            }
            // Related is not yet implemented - show unimplemented message
            show_unimplemented("analyze.related - not yet implemented", logger.as_ref());
            ("analyze.related", Ok(()))
        }

        // ============================================================================
        // DIAGNOSE Commands
        // ============================================================================
        Commands::Diagnose(DiagnoseCommands::Orphans {
            exclude_templates,
            exclude_daily,
        }) => {
            let config = load_config(cli.config)?;
            if let Some(ref log) = logger {
                let _ = log.log_section("diagnose.orphans", "Starting Diagnose Orphans Command");
            }
            (
                "diagnose.orphans",
                diagnose_orphans(&config, exclude_templates, exclude_daily, logger.as_ref()),
            )
        }
        Commands::Diagnose(DiagnoseCommands::BrokenLinks) => {
            let config = load_config(cli.config)?;
            if let Some(ref log) = logger {
                let _ = log.log_section(
                    "diagnose.broken-links",
                    "Starting Diagnose Broken Links Command",
                );
            }
            (
                "diagnose.broken-links",
                diagnose_broken_links_cmd(&config, logger.as_ref()),
            )
        }

        // ============================================================================
        // VIEW Commands
        // ============================================================================
        Commands::View(ViewCommands::Stats) => {
            let config = load_config(cli.config)?;
            if let Some(ref log) = logger {
                let _ = log.log_section("view.stats", "Starting Stats Command");
            }
            ("view.stats", show_stats(&config, logger.as_ref()))
        }
        Commands::View(ViewCommands::Describe { filename }) => {
            let config = load_config(cli.config)?;
            if let Some(ref log) = logger {
                let _ = log.log_section("view.describe", "Starting Describe Command");
            }
            (
                "view.describe",
                get_note_describe(&config, &filename, logger.as_ref()),
            )
        }

        // ============================================================================
        // TUI
        // ============================================================================
        Commands::Tui => {
            if let Some(ref log) = logger {
                let _ = log.log_section("tui", "Starting TUI Command");
            }
            show_tui(logger.as_ref());
            ("tui", Ok(()))
        }
    };

    // Handle JSON output for machine contracts
    if is_json {
        // Get vault path from config if available
        let vault_path = config
            .as_ref()
            .map(|c| c.vault_path.to_string_lossy().to_string())
            .unwrap_or_default();

        // Build params and result based on command_name
        // For commands that need parameters, we fetch from config/db
        let params = serde_json::json!({});
        let result_data = if command_name == "query.search" {
            // Try to get search results from the database
            if let Some(ref cfg) = config {
                let db_path = cfg.database_path();
                if db_path.exists() {
                    if let Ok(db) = Database::open(&db_path) {
                        // Use default search parameters
                        if let Ok(results) = db
                            .conn()
                            .execute_query(|conn| query::search_chunks(conn, "productivity", 20))
                        {
                            let items: Vec<serde_json::Value> = results
                                .iter()
                                .map(|r| {
                                    serde_json::json!({
                                        "chunk_id": r.chunk_id,
                                        "note_id": r.note_id,
                                        "note_path": r.note_path,
                                        "note_title": r.note_title,
                                        "heading_path": r.heading_path,
                                        "chunk_text": r.chunk_text,
                                        "rank": r.rank
                                    })
                                })
                                .collect();
                            serde_json::json!({
                                "total": results.len(),
                                "items": items
                            })
                        } else {
                            serde_json::json!({"status": "success"})
                        }
                    } else {
                        serde_json::json!({"status": "success"})
                    }
                } else {
                    serde_json::json!({"status": "success"})
                }
            } else {
                serde_json::json!({"status": "success"})
            }
        } else if command_name == "view.stats" {
            // Try to get stats from database
            if let Some(ref cfg) = config {
                let db_path = cfg.database_path();
                if db_path.exists() {
                    if let Ok(db) = Database::open(&db_path) {
                        if let Ok(stats) = db.get_stats() {
                            serde_json::json!({
                                "notes": stats.note_count,
                                "links": stats.link_count,
                                "tags": stats.tag_count,
                                "chunks": stats.chunk_count,
                                "unresolved_links": stats.unresolved_links
                            })
                        } else {
                            serde_json::json!({"status": "success"})
                        }
                    } else {
                        serde_json::json!({"status": "success"})
                    }
                } else {
                    serde_json::json!({"status": "success"})
                }
            } else {
                serde_json::json!({"status": "success"})
            }
        } else {
            serde_json::json!({"status": "success"})
        };

        let response = serde_json::json!({
            "version": "1.0",
            "command": command_name,
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "params": params,
            "result": result_data,
            "meta": {
                "query_time_ms": start.elapsed().as_millis() as u64,
                "vault_path": vault_path
            }
        });

        match result {
            Ok(_) => {
                println!("{}", serde_json::to_string(&response).unwrap_or_default());
            }
            Err(e) => {
                let error = serde_json::json!({
                    "error": {
                        "code": 1,
                        "message": e.to_string()
                    }
                });
                eprintln!("{}", serde_json::to_string(&error).unwrap_or_default());
                std::process::exit(1);
            }
        }
    } else {
        let elapsed = start.elapsed();
        if result.is_ok() {
            println!("Command '{command_name}' completed in {elapsed:.2?}");
        } else {
            eprintln!("Command '{command_name}' failed after {elapsed:.2?}");
        }
    }

    result
}

/// Default config template that will be seeded on first run
const DEFAULT_CONFIG: &str = include_str!("../template-config.toml");

fn ensure_config_exists(path: &PathBuf) -> Result<PathBuf> {
    if path.exists() {
        // Read existing config
        let contents = std::fs::read_to_string(path)?;
        // Check if it has vault_path (valid config)
        if contents.contains("vault_path") {
            return Ok(path.clone());
        }
        // If not, overwrite with default
    }

    // Create parent directories
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).context("Failed to create config directory")?;
    }

    // Write default config
    std::fs::write(path, DEFAULT_CONFIG).context("Failed to write default config file")?;

    println!("Created default config at: {}", path.display());

    Ok(path.clone())
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

use std::io::{self, Write};

fn interactive_config_setup(path: Option<PathBuf>) -> Result<Config> {
    let path = path.unwrap_or_else(|| {
        let mut p = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        p.push("obsidian-cli-inspector");
        p.push("config.toml");
        p
    });

    // Ensure config directory exists
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).context("Failed to create config directory")?;
    }

    // Prompt user for vault path (most important setting)
    println!("=== First-time Setup ===");
    println!("Please enter the path to your Obsidian vault.");
    println!("This is the only required setting to get started.\n");

    // Try to load existing config or create a new one with defaults
    let mut cfg = match Config::from_file(&path) {
        Ok(c) => c,
        Err(_) => {
            // Create a default config with placeholder vault path
            Config {
                vault_path: PathBuf::from("/path/to/your/obsidian/vault"),
                database_path: None,
                log_path: None,
                exclude: Default::default(),
                search: Default::default(),
                graph: Default::default(),
                llm: None,
            }
        }
    };

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

    // 2) Database path (optional)
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
        cfg.database_path = Some(db_default);
    }

    // 3) Log path (optional)
    let log_default = cfg.log_path.clone().unwrap_or_else(|| cfg.log_dir());
    print!("Log path [{}]: ", log_default.display());
    io::stdout().flush()?;
    input.clear();
    let _ = io::stdin().read_line(&mut input)?;
    let val = input.trim();
    if !val.is_empty() {
        cfg.log_path = Some(PathBuf::from(val));
    } else {
        cfg.log_path = Some(log_default);
    }

    // Persist updated config back to disk
    let toml = toml::to_string_pretty(&cfg).context("Failed to serialize config to TOML")?;
    std::fs::write(&path, toml).context("Failed to write updated config file")?;

    println!("Updated config at: {}", path.display());

    Ok(cfg)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_default_config_includes_vault_path_placeholder() {
        assert!(DEFAULT_CONFIG.contains("vault_path"));
        assert!(DEFAULT_CONFIG.contains("\"/path/to/your/obsidian/vault\""));
    }

    #[test]
    fn test_default_config_includes_all_sections() {
        assert!(DEFAULT_CONFIG.contains("[exclude]"));
        assert!(DEFAULT_CONFIG.contains("[search]"));
        assert!(DEFAULT_CONFIG.contains("[graph]"));
    }

    #[test]
    fn test_ensure_config_creates_default() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");

        let result = ensure_config_exists(&config_path);
        assert!(result.is_ok());
        assert!(config_path.exists());

        let contents = fs::read_to_string(&config_path).unwrap();
        assert!(contents.contains("vault_path"));
    }

    #[test]
    fn test_ensure_config_preserves_existing() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");

        // Create existing valid config with vault_path
        fs::write(&config_path, "vault_path = \"/test/path\"\n").unwrap();

        // Should preserve existing config
        let result = ensure_config_exists(&config_path);
        assert!(result.is_ok());
        assert!(config_path.exists());

        let contents = fs::read_to_string(&config_path).unwrap();
        assert!(contents.contains("vault_path"));
        assert!(contents.contains("/test/path"));
    }

    #[test]
    fn test_load_config_returns_default_when_missing() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");

        // Should create default config
        let result = load_config(Some(config_path.clone()));
        assert!(result.is_ok());
        assert!(config_path.exists());
    }

    #[test]
    fn test_default_config_has_search_section() {
        assert!(DEFAULT_CONFIG.contains("[search]"));
        assert!(DEFAULT_CONFIG.contains("default_limit"));
    }

    #[test]
    fn test_default_config_has_exclude_section() {
        assert!(DEFAULT_CONFIG.contains("[exclude]"));
        assert!(DEFAULT_CONFIG.contains("patterns"));
    }
}
