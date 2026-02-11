use anyhow::Result;
use obsidian_cli_inspector::scanner::VaultScanner;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_scanner_basic() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let vault_path = temp_dir.path();
    
    // Create some markdown files
    fs::write(vault_path.join("note1.md"), "# Note 1")?;
    fs::write(vault_path.join("note2.md"), "# Note 2")?;
    
    let scanner = VaultScanner::new(vault_path.to_path_buf(), vec![]);
    let entries = scanner.scan()?;
    
    assert_eq!(entries.len(), 2);
    Ok(())
}

#[test]
fn test_scanner_excludes_patterns() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let vault_path = temp_dir.path();
    
    // Create directories
    fs::create_dir_all(vault_path.join(".obsidian"))?;
    
    // Create files
    fs::write(vault_path.join("note.md"), "# Note")?;
    fs::write(vault_path.join(".obsidian/config.md"), "# Config")?;
    
    let scanner = VaultScanner::new(vault_path.to_path_buf(), vec![".obsidian/".to_string()]);
    let entries = scanner.scan()?;
    
    assert_eq!(entries.len(), 1);
    assert!(entries[0].relative_path.contains("note.md"));
    Ok(())
}

#[test]
fn test_scanner_nested_directories() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let vault_path = temp_dir.path();
    
    fs::create_dir_all(vault_path.join("folder/subfolder"))?;
    fs::write(vault_path.join("root.md"), "# Root")?;
    fs::write(vault_path.join("folder/mid.md"), "# Mid")?;
    fs::write(vault_path.join("folder/subfolder/deep.md"), "# Deep")?;
    
    let scanner = VaultScanner::new(vault_path.to_path_buf(), vec![]);
    let entries = scanner.scan()?;
    
    assert_eq!(entries.len(), 3);
    Ok(())
}

#[test]
fn test_scanner_ignores_non_markdown() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let vault_path = temp_dir.path();
    
    fs::write(vault_path.join("note.md"), "# Note")?;
    fs::write(vault_path.join("image.png"), "fake image")?;
    fs::write(vault_path.join("document.txt"), "text")?;
    
    let scanner = VaultScanner::new(vault_path.to_path_buf(), vec![]);
    let entries = scanner.scan()?;
    
    assert_eq!(entries.len(), 1);
    Ok(())
}

#[test]
fn test_scanner_file_metadata() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let vault_path = temp_dir.path();
    
    let content = "# Test Note\n\nSome content here";
    fs::write(vault_path.join("note.md"), content)?;
    
    let scanner = VaultScanner::new(vault_path.to_path_buf(), vec![]);
    let entries = scanner.scan()?;
    
    assert_eq!(entries.len(), 1);
    assert!(entries[0].mtime > 0);
    assert_eq!(entries[0].size, content.len() as u64);
    Ok(())
}

#[test]
fn test_scanner_empty_vault() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let vault_path = temp_dir.path();
    
    let scanner = VaultScanner::new(vault_path.to_path_buf(), vec![]);
    let entries = scanner.scan()?;
    
    assert_eq!(entries.len(), 0);
    Ok(())
}

#[test]
fn test_scanner_multiple_exclude_patterns() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let vault_path = temp_dir.path();
    
    fs::create_dir_all(vault_path.join(".obsidian"))?;
    fs::create_dir_all(vault_path.join(".git"))?;
    
    fs::write(vault_path.join("note.md"), "# Note")?;
    fs::write(vault_path.join(".obsidian/config.md"), "# Config")?;
    fs::write(vault_path.join(".git/log.md"), "# Log")?;
    
    let scanner = VaultScanner::new(
        vault_path.to_path_buf(),
        vec![".obsidian/".to_string(), ".git/".to_string()],
    );
    let entries = scanner.scan()?;
    
    assert_eq!(entries.len(), 1);
    Ok(())
}

#[test]
fn test_scanner_paths_info() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let vault_path = temp_dir.path();
    fs::create_dir_all(vault_path.join("dir"))?;
    fs::write(vault_path.join("dir/test.md"), "content")?;
    let scanner = VaultScanner::new(vault_path.to_path_buf(), vec![]);
    let entries = scanner.scan()?;
    assert_eq!(entries.len(), 1);
    assert!(entries[0].path.ends_with("test.md"));
    Ok(())
}
