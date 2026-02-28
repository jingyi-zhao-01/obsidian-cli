use anyhow::{Context, Result};

use crate::config::Config;
use crate::db::Database;
use crate::logger::Logger;

pub fn initialize_database(config: &Config, force: bool, logger: Option<&Logger>) -> Result<()> {
    let db_path = config.database_path();

    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent).with_context(|| {
            format!("Failed to create database directory: {}", parent.display())
        })?;
    }

    let msg = format!("Initializing database at: {}", db_path.display());
    if let Some(log) = logger {
        let _ = log.print_and_log("init", &msg);
    } else {
        println!("{msg}");
    }

    let db = Database::open(&db_path)
        .with_context(|| format!("Failed to open database: {}", db_path.display()))?;

    db.initialize(force)
        .context("Failed to initialize database schema")?;

    let version = db.get_version()?.unwrap_or(0);
    let msg = format!("Database initialized successfully (schema version: {version})");
    if let Some(log) = logger {
        let _ = log.print_and_log("init", &msg);
    } else {
        println!("{msg}");
    }

    Ok(())
}
