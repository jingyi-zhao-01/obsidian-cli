use anyhow::{Context, Result};
use crate::config::Config;
use crate::db::Database;
use crate::logger::Logger;
use crate::query;

/// Search vault using full-text search
pub fn search_vault(
    config: &Config,
    query_str: &str,
    limit: usize,
    logger: Option<&Logger>,
) -> Result<()> {
    let db_path = config.database_path();

    if !db_path.exists() {
        anyhow::bail!(
            "Database not found at: {}\nRun 'obsidian-cli init' first",
            db_path.display()
        );
    }

    let db = Database::open(&db_path)
        .with_context(|| format!("Failed to open database: {}", db_path.display()))?;

    if query_str.is_empty() {
        if let Some(log) = logger {
            let _ = log.print_and_log("search", "Search query cannot be empty");
        } else {
            println!("Search query cannot be empty");
        }
        return Ok(());
    }

    let results = db.conn()
        .execute_query(|conn| query::search_chunks(conn, query_str, limit))
        .context("Failed to execute search")?;

    if results.is_empty() {
        let msg = format!("No results found for: {}", query_str);
        if let Some(log) = logger {
            let _ = log.print_and_log("search", &msg);
        } else {
            println!("{}", msg);
        }
        return Ok(());
    }

    let msg = format!("Search Results for '{}' ({} results):", query_str, results.len());
    if let Some(log) = logger {
        let _ = log.print_and_log("search", &msg);
    } else {
        println!("{}", msg);
    }

    for (idx, result) in results.iter().enumerate() {
        let heading_info = result.heading_path
            .as_ref()
            .map(|h| format!(" [{}]", h))
            .unwrap_or_default();
        let msg = format!(
            "{}. {} ({}){}\n   {}",
            idx + 1,
            result.note_title,
            result.note_path,
            heading_info,
            result.chunk_text.lines().next().unwrap_or("").chars().take(80).collect::<String>()
        );
        if let Some(log) = logger {
            let _ = log.print_and_log("search", &msg);
        } else {
            println!("{}", msg);
        }
    }

    Ok(())
}

/// Get all notes that link to a specific note (backlinks)
pub fn get_backlinks(
    config: &Config,
    note: &str,
    logger: Option<&Logger>,
) -> Result<()> {
    let db_path = config.database_path();

    if !db_path.exists() {
        anyhow::bail!(
            "Database not found at: {}\nRun 'obsidian-cli init' first",
            db_path.display()
        );
    }

    let db = Database::open(&db_path)
        .with_context(|| format!("Failed to open database: {}", db_path.display()))?;

    let backlinks = db.conn()
        .execute_query(|conn| query::get_backlinks(conn, note))
        .context("Failed to get backlinks")?;

    if backlinks.is_empty() {
        let msg = format!("No backlinks found for: {}", note);
        if let Some(log) = logger {
            let _ = log.print_and_log("backlinks", &msg);
        } else {
            println!("{}", msg);
        }
        return Ok(());
    }

    let msg = format!("Backlinks to '{}' ({} found):", note, backlinks.len());
    if let Some(log) = logger {
        let _ = log.print_and_log("backlinks", &msg);
    } else {
        println!("{}", msg);
    }

    for (idx, link) in backlinks.iter().enumerate() {
        let link_type = if link.is_embed { "embed" } else { "link" };
        let msg = format!(
            "{}. {} ({})\n   Type: {} {}",
            idx + 1,
            link.note_title,
            link.note_path,
            link_type,
            link.alias.as_ref().map(|a| format!("(alias: {})", a)).unwrap_or_default()
        );
        if let Some(log) = logger {
            let _ = log.print_and_log("backlinks", &msg);
        } else {
            println!("{}", msg);
        }
    }

    Ok(())
}

/// Get all notes that a specific note links to (forward links)
pub fn get_forward_links(
    config: &Config,
    note: &str,
    logger: Option<&Logger>,
) -> Result<()> {
    let db_path = config.database_path();

    if !db_path.exists() {
        anyhow::bail!(
            "Database not found at: {}\nRun 'obsidian-cli init' first",
            db_path.display()
        );
    }

    let db = Database::open(&db_path)
        .with_context(|| format!("Failed to open database: {}", db_path.display()))?;

    let forward_links = db.conn()
        .execute_query(|conn| query::get_forward_links(conn, note))
        .context("Failed to get forward links")?;

    if forward_links.is_empty() {
        let msg = format!("No forward links found for: {}", note);
        if let Some(log) = logger {
            let _ = log.print_and_log("links", &msg);
        } else {
            println!("{}", msg);
        }
        return Ok(());
    }

    let msg = format!("Forward links from '{}' ({} found):", note, forward_links.len());
    if let Some(log) = logger {
        let _ = log.print_and_log("links", &msg);
    } else {
        println!("{}", msg);
    }

    for (idx, link) in forward_links.iter().enumerate() {
        let status = if link.note_id < 0 { "[unresolved]" } else { "[resolved]" };
        let msg = format!(
            "{}. {} ({})\n   {}",
            idx + 1,
            link.note_title,
            link.note_path,
            status
        );
        if let Some(log) = logger {
            let _ = log.print_and_log("links", &msg);
        } else {
            println!("{}", msg);
        }
    }

    Ok(())
}

/// List all unresolved links in the vault
pub fn list_unresolved_links(
    config: &Config,
    logger: Option<&Logger>,
) -> Result<()> {
    let db_path = config.database_path();

    if !db_path.exists() {
        anyhow::bail!(
            "Database not found at: {}\nRun 'obsidian-cli init' first",
            db_path.display()
        );
    }

    let db = Database::open(&db_path)
        .with_context(|| format!("Failed to open database: {}", db_path.display()))?;

    let unresolved = db.conn()
        .execute_query(|conn| query::get_unresolved_links(conn))
        .context("Failed to get unresolved links")?;

    if unresolved.is_empty() {
        let msg = "No unresolved links found!";
        if let Some(log) = logger {
            let _ = log.print_and_log("unresolved-links", msg);
        } else {
            println!("{}", msg);
        }
        return Ok(());
    }

    let msg = format!("Unresolved Links ({} found):", unresolved.len());
    if let Some(log) = logger {
        let _ = log.print_and_log("unresolved-links", &msg);
    } else {
        println!("{}", msg);
    }

    for (idx, link) in unresolved.iter().enumerate() {
        let msg = format!(
            "{}. {} → {}\n   In: {}",
            idx + 1,
            link.note_title,
            link.note_path,
            link.note_path
        );
        if let Some(log) = logger {
            let _ = log.print_and_log("unresolved-links", &msg);
        } else {
            println!("{}", msg);
        }
    }

    Ok(())
}

/// List notes by tag or show all tags
pub fn list_notes_by_tag(
    config: &Config,
    tag: &Option<String>,
    all: bool,
    logger: Option<&Logger>,
) -> Result<()> {
    let db_path = config.database_path();

    if !db_path.exists() {
        anyhow::bail!(
            "Database not found at: {}\nRun 'obsidian-cli init' first",
            db_path.display()
        );
    }

    let db = Database::open(&db_path)
        .with_context(|| format!("Failed to open database: {}", db_path.display()))?;

    if all || tag.is_none() {
        // List all tags
        let all_tags = db.conn()
            .execute_query(|conn| query::list_tags(conn))
            .context("Failed to list tags")?;

        if all_tags.is_empty() {
            let msg = "No tags found in vault";
            if let Some(log) = logger {
                let _ = log.print_and_log("tags", msg);
            } else {
                println!("{}", msg);
            }
            return Ok(());
        }

        let msg = format!("All Tags ({} total):", all_tags.len());
        if let Some(log) = logger {
            let _ = log.print_and_log("tags", &msg);
        } else {
            println!("{}", msg);
        }

        for (idx, tag_name) in all_tags.iter().enumerate() {
            let msg = format!("{}. {}", idx + 1, tag_name);
            if let Some(log) = logger {
                let _ = log.print_and_log("tags", &msg);
            } else {
                println!("{}", msg);
            }
        }
    } else if let Some(tag_name) = tag {
        // List notes with specific tag
        let notes = db.conn()
            .execute_query(|conn| query::get_notes_by_tag(conn, tag_name))
            .context("Failed to get notes by tag")?;

        if notes.is_empty() {
            let msg = format!("No notes found with tag: {}", tag_name);
            if let Some(log) = logger {
                let _ = log.print_and_log("tags", &msg);
            } else {
                println!("{}", msg);
            }
            return Ok(());
        }

        let msg = format!("Notes with tag '{}' ({} found):", tag_name, notes.len());
        if let Some(log) = logger {
            let _ = log.print_and_log("tags", &msg);
        } else {
            println!("{}", msg);
        }

        for (idx, note) in notes.iter().enumerate() {
            let msg = format!(
                "{}. {} ({})\n   Tags: {}",
                idx + 1,
                note.note_title,
                note.note_path,
                note.tags.join(", ")
            );
            if let Some(log) = logger {
                let _ = log.print_and_log("tags", &msg);
            } else {
                println!("{}", msg);
            }
        }
    }

    Ok(())
}

/// Show placeholder for unimplemented commands
pub fn show_unimplemented(command: &str, logger: Option<&Logger>) {
    let msg = format!("{} command not yet implemented", command);
    if let Some(log) = logger {
        let _ = log.print_and_log(command, &msg);
    } else {
        println!("{}", msg);
    }
}

pub fn show_search(query: &str, limit: usize, logger: Option<&Logger>) {
    if let Some(log) = logger {
        let _ = log.print_and_log("search", "Search command not yet implemented");
        let _ = log.print_and_log("search", &format!("  query: {}", query));
        let _ = log.print_and_log("search", &format!("  limit: {}", limit));
    } else {
        println!("Search command not yet implemented");
        println!("  query: {}", query);
        println!("  limit: {}", limit);
    }
}

pub fn show_backlinks(note: &str, logger: Option<&Logger>) {
    if let Some(log) = logger {
        let _ = log.print_and_log("backlinks", "Backlinks command not yet implemented");
        let _ = log.print_and_log("backlinks", &format!("  note: {}", note));
    } else {
        println!("Backlinks command not yet implemented");
        println!("  note: {}", note);
    }
}

pub fn show_links(note: &str, logger: Option<&Logger>) {
    if let Some(log) = logger {
        let _ = log.print_and_log("links", "Links command not yet implemented");
        let _ = log.print_and_log("links", &format!("  note: {}", note));
    } else {
        println!("Links command not yet implemented");
        println!("  note: {}", note);
    }
}

pub fn show_tags(tag: &Option<String>, all: bool, logger: Option<&Logger>) {
    if let Some(log) = logger {
        let _ = log.print_and_log("tags", "Tags command not yet implemented");
        let _ = log.print_and_log("tags", &format!("  tag: {:?}", tag));
        let _ = log.print_and_log("tags", &format!("  all: {}", all));
    } else {
        println!("Tags command not yet implemented");
        println!("  tag: {:?}", tag);
        println!("  all: {}", all);
    }
}

pub fn show_suggest(note: &str, limit: usize, logger: Option<&Logger>) {
    if let Some(log) = logger {
        let _ = log.print_and_log("suggest", "Suggest command not yet implemented");
        let _ = log.print_and_log("suggest", &format!("  note: {}", note));
        let _ = log.print_and_log("suggest", &format!("  limit: {}", limit));
    } else {
        println!("Suggest command not yet implemented");
        println!("  note: {}", note);
        println!("  limit: {}", limit);
    }
}

pub fn show_bloat(threshold: usize, limit: usize, logger: Option<&Logger>) {
    if let Some(log) = logger {
        let _ = log.print_and_log("bloat", "Bloat command not yet implemented");
        let _ = log.print_and_log("bloat", &format!("  threshold: {}", threshold));
        let _ = log.print_and_log("bloat", &format!("  limit: {}", limit));
    } else {
        println!("Bloat command not yet implemented");
        println!("  threshold: {}", threshold);
        println!("  limit: {}", limit);
    }
}

pub fn show_tui(logger: Option<&Logger>) {
    if let Some(log) = logger {
        let _ = log.print_and_log("tui", "TUI not yet implemented");
    } else {
        println!("TUI not yet implemented");
    }
}

pub fn show_graph(note: &Option<String>, depth: usize, logger: Option<&Logger>) {
    if let Some(log) = logger {
        let _ = log.print_and_log("graph", "Graph command not yet implemented");
        let _ = log.print_and_log("graph", &format!("  note: {:?}", note));
        let _ = log.print_and_log("graph", &format!("  depth: {}", depth));
    } else {
        println!("Graph command not yet implemented");
        println!("  note: {:?}", note);
        println!("  depth: {}", depth);
    }
}
