use anyhow::Result;
use clap::Parser;
use obsidian_cli_inspector::{
    cli::{Cli, Commands},
    command_handlers,
    commands::*,
    logger::Logger,
};
use std::time::Instant;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let config = obsidian_cli_inspector::config_setup::load_config(cli.config.clone()).ok();
    let logger = config
        .as_ref()
        .and_then(|cfg| Logger::new(cfg.log_dir()).ok());

    let start = Instant::now();
    let (command_name, result) = match cli.command {
        Commands::Init { force } => command_handlers::run_init_command(cli.config.clone(), force),
        Commands::Stats => command_handlers::run_stats_command(&config, logger.as_ref()),
        Commands::Index {
            dry_run,
            force,
            verbose,
        } => command_handlers::run_index_command(
            &config,
            dry_run,
            force,
            verbose,
            logger.as_ref(),
        ),
        Commands::Search { query, limit } => {
            command_handlers::run_search_command(&config, &query, limit, logger.as_ref())
        }
        Commands::Backlinks { note } => {
            command_handlers::run_backlinks_command(&config, &note, logger.as_ref())
        }
        Commands::Links { note } => {
            command_handlers::run_links_command(&config, &note, logger.as_ref())
        }
        Commands::UnresolvedLinks => {
            command_handlers::run_unresolved_links_command(&config, logger.as_ref())
        }
        Commands::Tags { tag, all } => {
            command_handlers::run_tags_command(&config, &tag, all, logger.as_ref())
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
            command_handlers::run_describe_command(&config, &filename, logger.as_ref())
        }
        Commands::DiagnoseOrphans {
            exclude_templates,
            exclude_daily,
        } => command_handlers::run_diagnose_orphans_command(
            &config,
            exclude_templates,
            exclude_daily,
            logger.as_ref(),
        ),
        Commands::DiagnoseBrokenLinks => {
            command_handlers::run_diagnose_broken_links_command(&config, logger.as_ref())
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
