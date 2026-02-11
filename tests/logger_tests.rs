use anyhow::Result;
use obsidian_cli_inspector::logger::Logger;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_logger_creation() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let log_dir = temp_dir.path().join("logs");

    let _logger = Logger::new(log_dir.clone())?;

    // Verify the log directory was created
    assert!(log_dir.exists());

    Ok(())
}

#[test]
fn test_logger_get_log_file() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let log_dir = temp_dir.path().join("logs");

    let logger = Logger::new(log_dir.clone())?;
    let log_file = logger.get_log_file("test_command");

    // Verify the log file path contains the command name
    assert!(log_file.to_string_lossy().contains("test_command"));
    assert!(log_file.to_string_lossy().ends_with(".log"));

    Ok(())
}

#[test]
fn test_logger_log() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let log_dir = temp_dir.path().join("logs");

    let logger = Logger::new(log_dir.clone())?;
    logger.log("test_command", "Test log message")?;

    // Find the created log file
    let entries = fs::read_dir(&log_dir)?;
    let log_files: Vec<_> = entries
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("log"))
        .collect();

    assert_eq!(log_files.len(), 1);

    let content = fs::read_to_string(log_files[0].path())?;
    assert!(content.contains("Test log message"));

    Ok(())
}

#[test]
fn test_logger_log_section() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let log_dir = temp_dir.path().join("logs");

    let logger = Logger::new(log_dir.clone())?;
    logger.log_section("test_command", "Test Section")?;

    // Find the created log file
    let entries = fs::read_dir(&log_dir)?;
    let log_files: Vec<_> = entries
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("log"))
        .collect();

    assert_eq!(log_files.len(), 1);

    let content = fs::read_to_string(log_files[0].path())?;
    assert!(content.contains("=== Test Section ==="));

    Ok(())
}

#[test]
fn test_logger_print_and_log() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let log_dir = temp_dir.path().join("logs");

    let logger = Logger::new(log_dir.clone())?;
    logger.print_and_log("test_command", "Print and log message")?;

    // Find the created log file
    let entries = fs::read_dir(&log_dir)?;
    let log_files: Vec<_> = entries
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("log"))
        .collect();

    assert_eq!(log_files.len(), 1);

    let content = fs::read_to_string(log_files[0].path())?;
    assert!(content.contains("Print and log message"));

    Ok(())
}

#[test]
fn test_logger_multiple_logs() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let log_dir = temp_dir.path().join("logs");

    let logger = Logger::new(log_dir.clone())?;

    logger.log("test_command", "First message")?;
    logger.log("test_command", "Second message")?;
    logger.log_section("test_command", "New Section")?;
    logger.log("test_command", "Third message")?;

    // Find the created log file
    let entries = fs::read_dir(&log_dir)?;
    let log_files: Vec<_> = entries
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("log"))
        .collect();

    assert_eq!(log_files.len(), 1);

    let content = fs::read_to_string(log_files[0].path())?;
    assert!(content.contains("First message"));
    assert!(content.contains("Second message"));
    assert!(content.contains("=== New Section ==="));
    assert!(content.contains("Third message"));

    Ok(())
}
