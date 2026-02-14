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
            let config = load_config(cli.config)?;
            if let Some(ref log) = logger {
                let _ = log.log_section("init", "Starting Init Command");
            }
            ("init", initialize_database(&config, force, logger.as_ref()))
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
            ("describe", get_note_describe(&config, &filename, logger.as_ref()))
        }
    };
    let elapsed = start.elapsed();
    if result.is_ok() {
        println!("Command '{}' completed in {:.2?}", command_name, elapsed);
    } else {
        eprintln!("Command '{}' failed after {:.2?}", command_name, elapsed);
    }

    result
}

fn load_config(config_path: Option<PathBuf>) -> Result<Config> {
    let path = config_path.unwrap_or_else(|| {
        let mut p = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        p.push("obsidian-cli-inspector");
        p.push("config.toml");
        p
    });

    if !path.exists() {
        anyhow::bail!(
            "Config file not found at: {}\nCreate one using config.toml.example as template",
            path.display()
        );
    }

    Config::from_file(&path).context("Failed to load config file")
}
