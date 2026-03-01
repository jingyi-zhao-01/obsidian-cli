use clap::{Parser, Subcommand};
use std::path::PathBuf;

const LONG_ABOUT: &str = r#"
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
  1. Run 'init init' to set up the database
  2. Run 'index index' to scan and parse your vault
  3. Use query commands (search, backlinks, tags, etc.)
  4. Re-run 'index index' periodically to catch changes

CLI STRUCTURE:
  obsidian-cli-inspector <group> <function> [args...]

  Groups:
    init     - Database initialization
    index    - Vault indexing (scan, status)
    query    - Search and retrieval (search, backlinks, links, tags, unresolved)
    graph    - Graph operations (neighbors, paths, centrality, components)
    analyze  - Content analysis (bloat, related, similar, quality)
    diagnose - Diagnostics (orphans, broken-links, conflicts)
    view     - Display commands (stats, describe, health)

EXAMPLES:
  # Initialize and index your vault
  obsidian-cli-inspector init init
  obsidian-cli-inspector index index

  # Search for notes containing 'rust'
  obsidian-cli-inspector query search rust --limit 10

  # Find all notes linking to 'Project Ideas'
  obsidian-cli-inspector query backlinks "Project Ideas"

  # List all notes tagged with 'work'
  obsidian-cli-inspector query tags work

  # Find large notes that might need splitting
  obsidian-cli-inspector analyze bloat --threshold 100000

  # Launch interactive mode
  obsidian-cli-inspector tui

CONFIG:
  Place config at ~/.config/obsidian-cli-inspector/config.toml
  Specify vault path and database location there.
"#;

#[derive(Parser)]
#[command(name = "obsidian-cli-inspector")]
#[command(author, version)]
#[command(about = "Local-first CLI/TUI for indexing and querying Obsidian vaults")]
#[command(long_about = LONG_ABOUT)]
pub struct Cli {
    /// Path to config file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Output in JSON format (for machine integration)
    #[arg(short = 'o', long = "output", value_name = "FORMAT", global = true)]
    pub output: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
#[command(arg_required_else_help(true))]
pub enum Commands {
    /// Database initialization commands
    #[command(subcommand)]
    Init(InitCommands),

    /// Vault indexing commands
    #[command(subcommand)]
    Index(IndexCommands),

    /// Search and retrieval commands
    #[command(subcommand)]
    Query(QueryCommands),

    // /// Graph operations commands
    // #[command(subcommand)]
    // Graph(GraphCommands),
    /// Content analysis commands
    #[command(subcommand)]
    Analyze(AnalyzeCommands),

    /// Diagnostic commands
    #[command(subcommand)]
    Diagnose(DiagnoseCommands),

    /// Display commands
    #[command(subcommand)]
    View(ViewCommands),

    /// Launch interactive TUI
    Tui,
}

// ============================================================================
// INIT Commands
// ============================================================================
#[derive(Subcommand)]
pub enum InitCommands {
    /// Initialize or reinitialize the database
    Init {
        /// Force reinitialization (drops existing data)
        #[arg(short, long)]
        force: bool,
    },
}

// ============================================================================
// INDEX Commands
// ============================================================================
#[derive(Subcommand)]
pub enum IndexCommands {
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
    // /// Show indexing status
    // Status,

    // /// Scan vault for changes (without indexing)
    // Scan,
}

// ============================================================================
// QUERY Commands
// ============================================================================
#[derive(Subcommand)]
pub enum QueryCommands {
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
    Unresolved,

    /// List notes by tag
    Tags {
        /// Tag name (without #)
        tag: Option<String>,

        /// List all tags if no tag specified
        #[arg(short, long)]
        list: bool,
    },
}

// ============================================================================
// GRAPH Commands
// ============================================================================
// #[derive(Subcommand)]
// pub enum GraphCommands {
//     /// Find neighboring notes (BFS traversal)
//     Neighbors {
//         /// Note path or title
//         note: String,
//         /// Maximum traversal depth
//         #[arg(short, long, default_value = "2")]
//         depth: usize,
//     },
//     /// Find shortest paths between notes
//     Paths {
//         /// Source note path or title
//         source: String,
//         /// Target note path or title
//         target: String,
//     },
//     /// Calculate centrality metrics
//     Centrality,
//     /// Find connected components
//     Components,
// }

// ============================================================================
// ANALYZE Commands
// ============================================================================
#[derive(Subcommand)]
pub enum AnalyzeCommands {
    /// Detect bloated notes and suggest refactoring
    Bloat {
        /// Minimum size threshold in bytes
        #[arg(short, long, default_value = "50000")]
        threshold: usize,

        /// Maximum number of notes to analyze
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },

    /// Suggest related notes not directly linked
    Related {
        /// Note path or title
        note: String,

        /// Maximum number of suggestions
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },
    // /// Find similar notes based on content
    // Similar {
    //     /// Note path or title
    //     note: String,
    //     /// Maximum number of similar notes
    //     #[arg(short, long, default_value = "10")]
    //     limit: usize,
    // },
    // /// Analyze note quality metrics
    // Quality {
    //     /// Note path or title
    //     note: String,
    // },
}

// ============================================================================
// DIAGNOSE Commands
// ============================================================================
#[derive(Subcommand)]
pub enum DiagnoseCommands {
    /// Diagnose orphan notes (no incoming + no outgoing links)
    Orphans {
        /// Exclude template notes
        #[arg(long)]
        exclude_templates: bool,

        /// Exclude daily notes
        #[arg(long)]
        exclude_daily: bool,
    },

    /// Diagnose broken links (unresolved and ambiguous)
    BrokenLinks,
    // /// Diagnose note conflicts
    // Conflicts,
}

// ============================================================================
// VIEW Commands
// ============================================================================
#[derive(Subcommand)]
pub enum ViewCommands {
    /// Show statistics about the vault
    Stats,

    /// Describe file metadata (without displaying paragraphs)
    Describe {
        /// File path or title to describe
        filename: String,
    },
    // /// Show health metrics
    // Health,
}
