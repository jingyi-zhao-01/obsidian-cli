use rusqlite::{Connection, OptionalExtension, Result};

use super::NoteMetadata;

pub fn insert_note(
    conn: &Connection,
    path: &str,
    title: &str,
    mtime: u64,
    hash: &str,
    frontmatter_json: Option<&str>,
) -> Result<i64> {
    conn.execute(
        "INSERT INTO notes (path, title, mtime, hash, frontmatter_json)
         VALUES (?1, ?2, ?3, ?4, ?5)
         ON CONFLICT(path) DO UPDATE SET
            title = excluded.title,
            mtime = excluded.mtime,
            hash = excluded.hash,
            frontmatter_json = excluded.frontmatter_json,
            updated_at = CURRENT_TIMESTAMP",
        rusqlite::params![path, title, mtime as i64, hash, frontmatter_json],
    )?;

    let note_id: i64 = conn.query_row("SELECT id FROM notes WHERE path = ?1", [path], |row| {
        row.get(0)
    })?;

    Ok(note_id)
}

pub fn get_note_by_path(conn: &Connection, path: &str) -> Result<Option<i64>> {
    conn.query_row("SELECT id FROM notes WHERE path = ?1", [path], |row| {
        row.get(0)
    })
    .optional()
}

pub fn get_note_metadata_by_path(conn: &Connection, path: &str) -> Result<Option<NoteMetadata>> {
    conn.query_row(
        "SELECT id, mtime, hash FROM notes WHERE path = ?1",
        [path],
        |row| {
            Ok(NoteMetadata {
                id: row.get(0)?,
                mtime: row.get(1)?,
                hash: row.get(2)?,
            })
        },
    )
    .optional()
}

pub fn insert_tag(conn: &Connection, note_id: i64, tag: &str) -> Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO tags (note_id, tag) VALUES (?1, ?2)",
        rusqlite::params![note_id, tag],
    )?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn insert_link(
    conn: &Connection,
    src_note_id: i64,
    dst_text: &str,
    kind: &str,
    is_embed: bool,
    alias: Option<&str>,
    heading_ref: Option<&str>,
    block_ref: Option<&str>,
) -> Result<()> {
    conn.execute(
        "INSERT INTO links (src_note_id, dst_text, kind, is_embed, alias, heading_ref, block_ref)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        rusqlite::params![
            src_note_id,
            dst_text,
            kind,
            if is_embed { 1 } else { 0 },
            alias,
            heading_ref,
            block_ref
        ],
    )?;
    Ok(())
}

pub fn insert_chunk(
    conn: &Connection,
    note_id: i64,
    heading_path: Option<&str>,
    text: &str,
) -> Result<()> {
    let text_len = text.len() as i32;
    conn.execute(
        "INSERT INTO chunks (note_id, heading_path, text, byte_offset, byte_length)
         VALUES (?1, ?2, ?3, 0, ?4)",
        rusqlite::params![note_id, heading_path, text, text_len],
    )?;
    Ok(())
}

pub fn insert_chunk_with_offset(
    conn: &Connection,
    note_id: i64,
    heading_path: Option<&str>,
    text: &str,
    byte_offset: i32,
    byte_length: i32,
) -> Result<()> {
    conn.execute(
        "INSERT INTO chunks (note_id, heading_path, text, byte_offset, byte_length)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params![note_id, heading_path, text, byte_offset, byte_length],
    )?;
    Ok(())
}

pub fn clear_note_data(conn: &Connection, note_id: i64) -> Result<()> {
    conn.execute("DELETE FROM links WHERE src_note_id = ?1", [note_id])?;
    conn.execute("DELETE FROM tags WHERE note_id = ?1", [note_id])?;
    conn.execute("DELETE FROM chunks WHERE note_id = ?1", [note_id])?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_db() -> (TempDir, Connection) {
        let temp_dir = TempDir::new().unwrap();
        let conn = Connection::open(temp_dir.path().join("test.db")).unwrap();

        // Create minimal schema for testing
        conn.execute(
            "CREATE TABLE IF NOT EXISTS notes (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                path TEXT NOT NULL UNIQUE,
                title TEXT NOT NULL,
                mtime INTEGER NOT NULL,
                hash TEXT NOT NULL,
                frontmatter_json TEXT,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )
        .unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS tags (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                note_id INTEGER NOT NULL,
                tag TEXT NOT NULL,
                UNIQUE(note_id, tag)
            )",
            [],
        )
        .unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS links (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                src_note_id INTEGER NOT NULL,
                dst_note_id INTEGER,
                dst_text TEXT NOT NULL,
                kind TEXT NOT NULL,
                is_embed INTEGER NOT NULL DEFAULT 0,
                alias TEXT,
                heading_ref TEXT,
                block_ref TEXT
            )",
            [],
        )
        .unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS chunks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                note_id INTEGER NOT NULL,
                heading_path TEXT,
                text TEXT NOT NULL,
                byte_offset INTEGER NOT NULL DEFAULT 0,
                byte_length INTEGER NOT NULL DEFAULT 0
            )",
            [],
        )
        .unwrap();

        (temp_dir, conn)
    }

    #[test]
    fn test_insert_note() {
        let (_temp_dir, conn) = create_test_db();

        let note_id = insert_note(
            &conn,
            "test.md",
            "Test Note",
            1234567890,
            "hash123",
            Some("{}"),
        )
        .unwrap();

        assert!(note_id > 0);
    }

    #[test]
    fn test_insert_note_duplicate() {
        let (_temp_dir, conn) = create_test_db();

        let note_id1 =
            insert_note(&conn, "test.md", "Test Note", 1234567890, "hash123", None).unwrap();

        let note_id2 = insert_note(
            &conn,
            "test.md",
            "Updated Note",
            1234567891,
            "hash456",
            None,
        )
        .unwrap();

        // Should update existing note
        assert_eq!(note_id1, note_id2);
    }

    #[test]
    fn test_get_note_by_path() {
        let (_temp_dir, conn) = create_test_db();

        let note_id =
            insert_note(&conn, "test.md", "Test Note", 1234567890, "hash123", None).unwrap();

        let found_id = get_note_by_path(&conn, "test.md").unwrap();
        assert_eq!(found_id, Some(note_id));
    }

    #[test]
    fn test_get_note_by_path_not_found() {
        let (_temp_dir, conn) = create_test_db();

        let found_id = get_note_by_path(&conn, "nonexistent.md").unwrap();
        assert!(found_id.is_none());
    }

    #[test]
    fn test_get_note_metadata_by_path() {
        let (_temp_dir, conn) = create_test_db();

        insert_note(&conn, "test.md", "Test Note", 1234567890, "hash123", None).unwrap();

        let metadata = get_note_metadata_by_path(&conn, "test.md").unwrap();
        assert!(metadata.is_some());
        assert_eq!(metadata.unwrap().hash, "hash123");
    }

    #[test]
    fn test_insert_tag() {
        let (_temp_dir, conn) = create_test_db();

        let note_id =
            insert_note(&conn, "test.md", "Test Note", 1234567890, "hash123", None).unwrap();

        let result = insert_tag(&conn, note_id, "rust");
        assert!(result.is_ok());
    }

    #[test]
    fn test_insert_link() {
        let (_temp_dir, conn) = create_test_db();

        let note_id =
            insert_note(&conn, "test.md", "Test Note", 1234567890, "hash123", None).unwrap();

        let result = insert_link(
            &conn, note_id, "other.md", "wikilink", false, None, None, None,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_insert_link_with_embed() {
        let (_temp_dir, conn) = create_test_db();

        let note_id =
            insert_note(&conn, "test.md", "Test Note", 1234567890, "hash123", None).unwrap();

        let result = insert_link(
            &conn,
            note_id,
            "other.md",
            "wikilink",
            true, // is_embed
            Some("alias"),
            Some("heading"),
            Some("block"),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_insert_chunk() {
        let (_temp_dir, conn) = create_test_db();

        let note_id =
            insert_note(&conn, "test.md", "Test Note", 1234567890, "hash123", None).unwrap();

        let result = insert_chunk(&conn, note_id, Some("# Heading"), "Some chunk text");
        assert!(result.is_ok());
    }

    #[test]
    fn test_insert_chunk_with_offset() {
        let (_temp_dir, conn) = create_test_db();

        let note_id =
            insert_note(&conn, "test.md", "Test Note", 1234567890, "hash123", None).unwrap();

        let result = insert_chunk_with_offset(
            &conn,
            note_id,
            Some("# Heading"),
            "Some chunk text",
            100,
            50,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_clear_note_data() {
        let (_temp_dir, conn) = create_test_db();

        let note_id =
            insert_note(&conn, "test.md", "Test Note", 1234567890, "hash123", None).unwrap();

        // Insert some tags and links
        insert_tag(&conn, note_id, "tag1").unwrap();
        insert_link(
            &conn, note_id, "other.md", "wikilink", false, None, None, None,
        )
        .unwrap();
        insert_chunk(&conn, note_id, None, "chunk text").unwrap();

        // Clear the data
        let result = clear_note_data(&conn, note_id);
        assert!(result.is_ok());
    }
}
