// CLI package - re-exports core modules and provides CLI-specific modules
pub mod cli_cmd;
pub mod commands;

pub use obsidian_cli_core::chunker;
pub use obsidian_cli_core::config::Config;
pub use obsidian_cli_core::db;
pub use obsidian_cli_core::logger::Logger;
pub use obsidian_cli_core::parser;
pub use obsidian_cli_core::query;
pub use obsidian_cli_core::scanner;

// Re-export command functions
pub use commands::diagnose_broken_links_cmd;
pub use commands::diagnose_orphans;
pub use commands::get_backlinks;
pub use commands::get_forward_links;
pub use commands::get_note_describe;
pub use commands::index_vault;
pub use commands::initialize_database;
pub use commands::list_notes_by_tag;
pub use commands::list_unresolved_links;
pub use commands::search_vault;
pub use commands::show_bloat;
pub use commands::show_graph;
pub use commands::show_stats;
pub use commands::show_suggest;
pub use commands::show_tui;
pub use commands::show_unimplemented;
