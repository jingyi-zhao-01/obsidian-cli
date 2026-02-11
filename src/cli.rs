use clap::{Parser, Subcommand};
use std::path::PathBuf;

const LONG_ABOUT: &str = "\
Obsidian CLI Inspector - A local-first, read-only CLI/TUI for Obsidian vaults

ABOUT:
  This tool helps you inspect, query, and navigate your Obsidian vault from the terminal.
  All data is stored locally in a SQLite database for fast, offline access.

USE CASES:
  • Search: Find notes quickly with full-text search across your entire vault
  • Link Analysis: Discover backlinks, forward links, and unresolved references
  • Tag Management: Filter and organize notes by tags with AND/OR logic
  • Graph Exploration: Visualize note relationships and connection depths
  • Content Quality: Identify bloated notes that may need refactoring
  • Related Notes: Get AI-style suggestions for notes you might want to link
  • Scripting: Integrate with shell scripts and automation workflows
  • Interactive TUI: Browse your vault in a terminal user interface

WORKFLOW:
  1. Run 'init' to set up the database
  2. Run 'index' to scan and parse your vault
  3. Use query commands (search, backlinks, tags, etc.)
  4. Re-run 'index' periodically to catch changes

EXAMPLES:
  # Initialize and index your vault
  obsidian-cli-inspector init
  obsidian-cli-inspector index

  # Search for notes containing 'rust'
  obsidian-cli-inspector search rust --limit 10

  # Find all notes linking to 'Project Ideas'
  obsidian-cli-inspector backlinks \"Project Ideas\"

  # List all notes tagged with 'work'
  obsidian-cli-inspector tags work

  # Find large notes that might need splitting
  obsidian-cli-inspector bloat --threshold 100000

  # Launch interactive mode
  obsidian-cli-inspector tui

CONFIG:
  Place config at ~/.config/obsidian-cli/config.toml
  Specify vault path and database location there.
";

#[derive(Parser)]
#[command(name = "obsidian-cli-inspector")]
#[command(author, version)]
#[command(about = "Local-first CLI/TUI for indexing and querying Obsidian vaults")]
#[command(long_about = LONG_ABOUT)]
pub struct Cli {
    /// Path to config file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize or reinitialize the database
    Init {
        /// Force reinitialization (drops existing data)
        #[arg(short, long)]
        force: bool,
    },

    /// Index the vault (scan and parse all files)
    Index {
        /// Perform a dry run without writing to database
        #[arg(short = 'n', long)]
        dry_run: bool,

        /// Force full re-index (ignores change detection)
        #[arg(short, long)]
        force: bool,

        /// Show verbose output
        #[arg(short, long)]
        verbose: bool,
    },

    /// Search notes using full-text search
    Search {
        /// Search query
        query: String,

        /// Maximum number of results
        #[arg(short, long, default_value = "20")]
        limit: usize,
    },

    /// List backlinks to a note
    Backlinks {
        /// Note path or title
        note: String,
    },

    /// List forward links from a note
    Links {
        /// Note path or title
        note: String,
    },

    /// List all unresolved links in the vault
    UnresolvedLinks,

    /// List notes by tag
    Tags {
        /// Tag name (without #)
        tag: Option<String>,

        /// List all tags if no tag specified
        #[arg(short, long)]
        all: bool,
    },

    /// Suggest related notes not directly linked
    Suggest {
        /// Note path or title
        note: String,

        /// Maximum number of suggestions
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },

    /// Detect bloated notes and suggest refactoring
    Bloat {
        /// Minimum size threshold in bytes
        #[arg(short, long, default_value = "50000")]
        threshold: usize,

        /// Maximum number of notes to analyze
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },

    /// Show statistics about the vault
    Stats,

    /// Launch interactive TUI
    Tui,

    /// Display vault graph information
    Graph {
        /// Note path or title (if not specified, shows overall graph)
        note: Option<String>,

        /// Maximum traversal depth
        #[arg(short, long, default_value = "2")]
        depth: usize,
    },
}
