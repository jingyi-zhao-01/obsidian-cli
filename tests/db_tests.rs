use anyhow::Result;
use obsidian_cli_inspector::db::Database;
use tempfile::TempDir;

#[test]
fn test_database_open_and_initialize() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let db_path = temp_dir.path().join("test.db");
    
    let db = Database::open(&db_path)?;
    db.initialize(false)?;
    
    // Check version was set
    let version = db.get_version()?;
    assert_eq!(version, Some(1));
    
    Ok(())
}

#[test]
fn test_database_insert_note() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let db_path = temp_dir.path().join("test.db");
    
    let db = Database::open(&db_path)?;
    db.initialize(false)?;
    
    let note_id = db.insert_note("test.md", "Test Note", 1234567890, "hash123", None)?;
    assert!(note_id > 0);
    
    Ok(())
}

#[test]
fn test_database_get_note_by_path() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let db_path = temp_dir.path().join("test.db");
    
    let db = Database::open(&db_path)?;
    db.initialize(false)?;
    
    db.insert_note("test.md", "Test", 123, "hash", None)?;
    
    let note_id = db.get_note_by_path("test.md")?;
    assert!(note_id.is_some());
    
    let missing = db.get_note_by_path("missing.md")?;
    assert!(missing.is_none());
    
    Ok(())
}

#[test]
fn test_database_insert_tag() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let db_path = temp_dir.path().join("test.db");
    
    let db = Database::open(&db_path)?;
    db.initialize(false)?;
    
    let note_id = db.insert_note("test.md", "Test", 123, "hash", None)?;
    db.insert_tag(note_id, "tag1")?;
    db.insert_tag(note_id, "tag2")?;
    
    Ok(())
}

#[test]
fn test_database_insert_link() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let db_path = temp_dir.path().join("test.db");
    
    let db = Database::open(&db_path)?;
    db.initialize(false)?;
    
    let note_id = db.insert_note("test.md", "Test", 123, "hash", None)?;
    db.insert_link(note_id, "other", "wikilink", false, None, None, None)?;
    
    Ok(())
}

#[test]
fn test_database_insert_chunk() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let db_path = temp_dir.path().join("test.db");
    
    let db = Database::open(&db_path)?;
    db.initialize(false)?;
    
    let note_id = db.insert_note("test.md", "Test", 123, "hash", None)?;
    db.insert_chunk(note_id, Some("# Heading"), "Chunk text")?;
    db.insert_chunk_with_offset(note_id, None, "Another chunk", 0, 100)?;
    
    Ok(())
}

#[test]
fn test_database_clear_note_data() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let db_path = temp_dir.path().join("test.db");
    
    let db = Database::open(&db_path)?;
    db.initialize(false)?;
    
    let note_id = db.insert_note("test.md", "Test", 123, "hash", None)?;
    db.insert_tag(note_id, "tag")?;
    db.insert_link(note_id, "other", "wikilink", false, None, None, None)?;
    db.insert_chunk(note_id, None, "text")?;
    
    db.clear_note_data(note_id)?;
    
    Ok(())
}

#[test]
fn test_database_get_stats() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let db_path = temp_dir.path().join("test.db");
    
    let db = Database::open(&db_path)?;
    db.initialize(false)?;
    
    db.insert_note("test1.md", "Test 1", 123, "hash1", None)?;
    db.insert_note("test2.md", "Test 2", 456, "hash2", None)?;
    
    let stats = db.get_stats()?;
    assert_eq!(stats.note_count, 2);
    
    Ok(())
}
