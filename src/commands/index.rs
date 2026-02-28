use anyhow::{Context, Result};
use std::fs;

use crate::chunker::MarkdownChunker;
use crate::config::Config;
use crate::db::Database;
use crate::logger::Logger;
use crate::parser::MarkdownParser;
use crate::scanner::VaultScanner;

pub fn index_vault(
    config: &Config,
    dry_run: bool,
    force: bool,
    verbose: bool,
    logger: Option<&Logger>,
) -> Result<()> {
    let db_path = config.database_path();

    if !db_path.exists() {
        anyhow::bail!(
            "Database not found at: {}\nRun 'obsidian-cli init' first",
            db_path.display()
        );
    }

    let mut db = Database::open(&db_path)
        .with_context(|| format!("Failed to open database: {}", db_path.display()))?;

    if verbose {
        let msg = "Starting vault indexing...";
        if let Some(log) = logger {
            let _ = log.print_and_log("index", msg);
        } else {
            println!("{msg}");
        }
        let msg = format!("Vault path: {}", config.vault_path.display());
        if let Some(log) = logger {
            let _ = log.print_and_log("index", &msg);
        } else {
            println!("{msg}");
        }
    }

    // Scan the vault
    let scanner = VaultScanner::new(config.vault_path.clone(), config.exclude.patterns.clone());
    let files = scanner.scan().context("Failed to scan vault")?;

    if verbose {
        let msg = format!("Found {} markdown files", files.len());
        if let Some(log) = logger {
            let _ = log.print_and_log("index", &msg);
        } else {
            println!("{msg}");
        }
    }

    if dry_run {
        let msg = format!("[DRY RUN] Would index {} files", files.len());
        if let Some(log) = logger {
            let _ = log.print_and_log("index", &msg);
        } else {
            println!("{msg}");
        }
        return Ok(());
    }

    let chunker = MarkdownChunker::default();
    let tx = db
        .transaction()
        .context("Failed to start database transaction")?;

    // Index each file
    let mut indexed_count = 0;
    let mut skipped_count = 0;
    for file in files {
        let hash = format!("{:x}:{}", file.size, file.mtime);
        let existing = tx
            .get_note_metadata_by_path(&file.relative_path)
            .context("Failed to look up note metadata")?;

        if !force {
            if let Some(meta) = &existing {
                if meta.hash == hash {
                    skipped_count += 1;
                    if verbose {
                        let msg = format!("Skipping unchanged: {}", file.relative_path);
                        if let Some(log) = logger {
                            let _ = log.print_and_log("index", &msg);
                        } else {
                            println!("{msg}");
                        }
                    }
                    continue;
                }
            }
        }

        if verbose {
            let msg = format!("Indexing: {}", file.relative_path);
            if let Some(log) = logger {
                let _ = log.print_and_log("index", &msg);
            } else {
                println!("{msg}");
            }
        }

        // Read file content
        let content = fs::read_to_string(&file.path)
            .with_context(|| format!("Failed to read file: {}", file.path.display()))?;

        // Parse markdown
        let parsed = MarkdownParser::parse(&content);

        if let Some(meta) = existing {
            tx.clear_note_data(meta.id)
                .context("Failed to clear note data")?;
        }

        // Insert note
        let note_id = tx
            .insert_note(&file.relative_path, &parsed.title, file.mtime, &hash, None)
            .context("Failed to insert note")?;

        if verbose {
            let msg = format!("  → Note: {} (id: {})", parsed.title, note_id);
            if let Some(log) = logger {
                let _ = log.print_and_log("index", &msg);
            } else {
                println!("{msg}");
            }
        }

        // Insert tags
        for tag in &parsed.tags {
            tx.insert_tag(note_id, tag)
                .context("Failed to insert tag")?;
            if verbose {
                let msg = format!("    • Tag: {tag}");
                if let Some(log) = logger {
                    let _ = log.print_and_log("index", &msg);
                } else {
                    println!("{msg}");
                }
            }
        }

        // Insert links
        for link in &parsed.links {
            tx.insert_link(
                note_id,
                &link.text,
                link.link_type.as_str(),
                link.is_embed,
                link.alias.as_deref(),
                link.heading_ref.as_deref(),
                link.block_ref.as_deref(),
            )
            .context("Failed to insert link")?;
            if verbose {
                let link_kind = if link.is_embed { "Embed" } else { "Link" };
                let msg = format!("    • {}: [[{}]]", link_kind, link.text);
                if let Some(log) = logger {
                    let _ = log.print_and_log("index", &msg);
                } else {
                    println!("{msg}");
                }
            }
        }

        // Create chunker and split content into chunks
        let chunks = chunker.chunk(&content);

        if verbose {
            let msg = format!("    • Created {} chunk(s)", chunks.len());
            if let Some(log) = logger {
                let _ = log.print_and_log("index", &msg);
            } else {
                println!("{msg}");
            }
        }

        // Insert chunks
        for chunk in chunks {
            tx.insert_chunk_with_offset(
                note_id,
                chunk.heading_path.as_deref(),
                &chunk.text,
                chunk.byte_offset as i32,
                chunk.byte_length as i32,
            )
            .context("Failed to insert chunk")?;

            if verbose {
                let heading_info = chunk
                    .heading_path
                    .as_ref()
                    .map(|h| format!(" [{h}]"))
                    .unwrap_or_default();
                let msg = format!(
                    "      - Chunk: {} chars, ~{} tokens{}",
                    chunk.text.len(),
                    chunk.token_count,
                    heading_info
                );
                if let Some(log) = logger {
                    let _ = log.print_and_log("index", &msg);
                } else {
                    println!("{msg}");
                }
            }
        }

        indexed_count += 1;
    }

    tx.commit().context("Failed to commit transaction")?;

    let msg = if skipped_count > 0 {
        format!("Indexed {indexed_count} notes successfully (skipped {skipped_count} unchanged)")
    } else {
        format!("Indexed {indexed_count} notes successfully")
    };
    if let Some(log) = logger {
        let _ = log.print_and_log("index", &msg);
    } else {
        println!("{msg}");
    }

    Ok(())
}
