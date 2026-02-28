use clap::CommandFactory;
use obsidian_cli_inspector::cli::Cli;

#[test]
fn test_cli_help_contains_use_cases() {
    let cmd = Cli::command();
    let long_help = cmd.get_long_about().expect("Long about should be set");
    let help_text = long_help.to_string();

    // Verify key sections are present
    assert!(help_text.contains("USE CASES:"));
    assert!(help_text.contains("WORKFLOW:"));
    assert!(help_text.contains("EXAMPLES:"));
    assert!(help_text.contains("CONFIG:"));
}

#[test]
fn test_cli_help_contains_search_use_case() {
    let cmd = Cli::command();
    let long_help = cmd.get_long_about().expect("Long about should be set");
    let help_text = long_help.to_string();

    assert!(help_text.contains("Search:"));
    assert!(help_text.contains("full-text search"));
}

#[test]
fn test_cli_help_contains_link_analysis() {
    let cmd = Cli::command();
    let long_help = cmd.get_long_about().expect("Long about should be set");
    let help_text = long_help.to_string();

    assert!(help_text.contains("Link Analysis:"));
    assert!(help_text.contains("backlinks"));
    assert!(help_text.contains("forward links"));
}

#[test]
fn test_cli_help_contains_tag_management() {
    let cmd = Cli::command();
    let long_help = cmd.get_long_about().expect("Long about should be set");
    let help_text = long_help.to_string();

    assert!(help_text.contains("Tag Management:"));
    assert!(help_text.contains("AND/OR logic"));
}

#[test]
fn test_cli_help_contains_graph_exploration() {
    let cmd = Cli::command();
    let long_help = cmd.get_long_about().expect("Long about should be set");
    let help_text = long_help.to_string();

    assert!(help_text.contains("Graph Exploration:"));
    assert!(help_text.contains("relationships"));
}

#[test]
fn test_cli_help_contains_bloat_detection() {
    let cmd = Cli::command();
    let long_help = cmd.get_long_about().expect("Long about should be set");
    let help_text = long_help.to_string();

    assert!(help_text.contains("Content Quality:"));
    assert!(help_text.contains("bloated notes"));
}

#[test]
fn test_cli_help_contains_workflow_steps() {
    let cmd = Cli::command();
    let long_help = cmd.get_long_about().expect("Long about should be set");
    let help_text = long_help.to_string();

    assert!(help_text.contains("1. Run 'init'"));
    assert!(help_text.contains("2. Run 'index'"));
    assert!(help_text.contains("3. Use query commands"));
}

#[test]
fn test_cli_help_contains_example_commands() {
    let cmd = Cli::command();
    let long_help = cmd.get_long_about().expect("Long about should be set");
    let help_text = long_help.to_string();

    assert!(help_text.contains("obsidian-cli-inspector init"));
    assert!(help_text.contains("obsidian-cli-inspector index"));
    assert!(help_text.contains("obsidian-cli-inspector search"));
    assert!(help_text.contains("obsidian-cli-inspector backlinks"));
    assert!(help_text.contains("obsidian-cli-inspector tags"));
    assert!(help_text.contains("obsidian-cli-inspector bloat"));
    assert!(help_text.contains("obsidian-cli-inspector tui"));
}

#[test]
fn test_cli_help_contains_config_location() {
    let cmd = Cli::command();
    let long_help = cmd.get_long_about().expect("Long about should be set");
    let help_text = long_help.to_string();

    assert!(help_text.contains("~/.config/obsidian-cli-inspector/config.toml"));
}

#[test]
fn test_cli_short_about_is_set() {
    let cmd = Cli::command();
    let about = cmd.get_about().expect("About should be set");

    assert!(about.to_string().contains("Local-first"));
    assert!(about.to_string().contains("CLI/TUI"));
}

#[test]
fn test_cli_version_is_set() {
    let cmd = Cli::command();
    let version = cmd.get_version();

    assert!(version.is_some());
}

#[test]
fn test_cli_author_is_set() {
    let cmd = Cli::command();
    let author = cmd.get_author();

    assert!(author.is_some());
}

#[test]
fn test_cli_all_subcommands_present() {
    let cmd = Cli::command();
    let subcommands: Vec<_> = cmd.get_subcommands().map(|s| s.get_name()).collect();

    assert!(subcommands.contains(&"init"));
    assert!(subcommands.contains(&"index"));
    assert!(subcommands.contains(&"search"));
    assert!(subcommands.contains(&"backlinks"));
    assert!(subcommands.contains(&"links"));
    assert!(subcommands.contains(&"unresolved-links"));
    assert!(subcommands.contains(&"tags"));
    assert!(subcommands.contains(&"suggest"));
    assert!(subcommands.contains(&"bloat"));
    assert!(subcommands.contains(&"stats"));
    assert!(subcommands.contains(&"tui"));
    assert!(subcommands.contains(&"graph"));
}

#[test]
fn test_cli_help_rendering_succeeds() {
    let mut cmd = Cli::command();

    // This tests that rendering the help doesn't panic
    let help_output = cmd.render_help();
    assert!(!help_output.to_string().is_empty());
}

#[test]
fn test_cli_long_help_rendering_succeeds() {
    let mut cmd = Cli::command();

    // This tests that rendering the long help doesn't panic
    let long_help_output = cmd.render_long_help();
    assert!(!long_help_output.to_string().is_empty());
}

#[test]
fn test_cli_help_all_use_cases_covered() {
    let cmd = Cli::command();
    let long_help = cmd.get_long_about().expect("Long about should be set");
    let help_text = long_help.to_string();

    // Verify all 8 use cases are documented
    let use_case_markers = [
        "Search:",
        "Link Analysis:",
        "Tag Management:",
        "Graph Exploration:",
        "Content Quality:",
        "Related Notes:",
        "Scripting:",
        "Interactive TUI:",
    ];

    for marker in &use_case_markers {
        assert!(help_text.contains(marker), "Missing use case: {marker}");
    }
}
