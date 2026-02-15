// Query module for retrieving and searching vault data
mod links;
mod search;
mod tags;

pub use links::{
    diagnose_broken_links, get_backlinks, get_dead_ends, get_forward_links, get_orphans,
    get_unresolved_links, BrokenLinkResult, DiagnoseResult, LinkResult,
};
pub use search::{search_chunks, SearchResult};
pub use tags::{
    get_notes_by_tag, get_notes_by_tags_and, get_notes_by_tags_or, list_tags, TagResult,
};

use rusqlite::{Connection, OptionalExtension};
use serde::Serialize;

/// Metadata for a single note
#[derive(Debug, Serialize)]
pub struct NoteDescribeResult {
    pub id: i64,
    pub path: String,
    pub title: String,
    pub mtime: i64,
    pub hash: String,
    pub created_at: String,
    pub updated_at: String,
    pub frontmatter: Option<String>,
}

/// Get note metadata by path or title
pub fn get_note_by_filename(
    conn: &Connection,
    filename: &str,
) -> rusqlite::Result<Option<NoteDescribeResult>> {
    // First try to find by exact path match
    let result = conn
        .query_row(
            "SELECT id, path, title, mtime, hash, created_at, updated_at, frontmatter_json
             FROM notes
             WHERE path = ?1 OR title = ?1",
            [filename],
            |row| {
                Ok(NoteDescribeResult {
                    id: row.get(0)?,
                    path: row.get(1)?,
                    title: row.get(2)?,
                    mtime: row.get(3)?,
                    hash: row.get(4)?,
                    created_at: row.get(5)?,
                    updated_at: row.get(6)?,
                    frontmatter: row.get(7)?,
                })
            },
        )
        .optional();

    // If not found by exact match, try partial match on path or title
    if let Ok(None) = result {
        conn.query_row(
            "SELECT id, path, title, mtime, hash, created_at, updated_at, frontmatter_json
             FROM notes
             WHERE path LIKE ?1 OR title LIKE ?1
             LIMIT 1",
            [format!("%{}%", filename)],
            |row| {
                Ok(NoteDescribeResult {
                    id: row.get(0)?,
                    path: row.get(1)?,
                    title: row.get(2)?,
                    mtime: row.get(3)?,
                    hash: row.get(4)?,
                    created_at: row.get(5)?,
                    updated_at: row.get(6)?,
                    frontmatter: row.get(7)?,
                })
            },
        )
        .optional()
    } else {
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_describe_result_creation() {
        let note = NoteDescribeResult {
            id: 1,
            path: "test.md".to_string(),
            title: "Test".to_string(),
            mtime: 1234567890,
            hash: "abc123".to_string(),
            created_at: "2024-01-01".to_string(),
            updated_at: "2024-01-02".to_string(),
            frontmatter: Some("{}".to_string()),
        };

        assert_eq!(note.id, 1);
        assert_eq!(note.path, "test.md");
        assert!(note.frontmatter.is_some());
    }

    #[test]
    fn test_note_describe_result_no_frontmatter() {
        let note = NoteDescribeResult {
            id: 1,
            path: "test.md".to_string(),
            title: "Test".to_string(),
            mtime: 1234567890,
            hash: "abc123".to_string(),
            created_at: "2024-01-01".to_string(),
            updated_at: "2024-01-02".to_string(),
            frontmatter: None,
        };

        assert!(note.frontmatter.is_none());
    }

    #[test]
    fn test_get_note_by_filename_exact_path() {
        let conn = Connection::open_in_memory().unwrap();

        // Create table
        conn.execute(
            "CREATE TABLE notes (id INTEGER PRIMARY KEY, path TEXT, title TEXT, mtime INTEGER, hash TEXT, created_at TEXT, updated_at TEXT, frontmatter_json TEXT)",
            [],
        ).unwrap();

        // Insert test data using params
        conn.execute(
            "INSERT INTO notes (path, title, mtime, hash, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params!["test.md", "Test Note", 1234567890_i64, "hash123", "2024-01-01", "2024-01-02"],
        ).unwrap();

        // Test exact path match
        let result = get_note_by_filename(&conn, "test.md").unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap().title, "Test Note");
    }

    #[test]
    fn test_get_note_by_filename_exact_title() {
        let conn = Connection::open_in_memory().unwrap();

        // Create table
        conn.execute(
            "CREATE TABLE notes (id INTEGER PRIMARY KEY, path TEXT, title TEXT, mtime INTEGER, hash TEXT, created_at TEXT, updated_at TEXT, frontmatter_json TEXT)",
            [],
        ).unwrap();

        // Insert test data
        conn.execute(
            "INSERT INTO notes (path, title, mtime, hash, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params!["test.md", "Test Note", 1234567890_i64, "hash123", "2024-01-01", "2024-01-02"],
        ).unwrap();

        // Test exact title match
        let result = get_note_by_filename(&conn, "Test Note").unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap().title, "Test Note");
    }

    #[test]
    fn test_get_note_by_filename_partial_match() {
        let conn = Connection::open_in_memory().unwrap();

        // Create table
        conn.execute(
            "CREATE TABLE notes (id INTEGER PRIMARY KEY, path TEXT, title TEXT, mtime INTEGER, hash TEXT, created_at TEXT, updated_at TEXT, frontmatter_json TEXT)",
            [],
        ).unwrap();

        // Insert test data
        conn.execute(
            "INSERT INTO notes (path, title, mtime, hash, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params!["test.md", "Test Note", 1234567890_i64, "hash123", "2024-01-01", "2024-01-02"],
        ).unwrap();

        // Test partial match
        let result = get_note_by_filename(&conn, "Test").unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn test_get_note_by_filename_not_found() {
        let conn = Connection::open_in_memory().unwrap();

        // Create table
        conn.execute(
            "CREATE TABLE notes (id INTEGER PRIMARY KEY, path TEXT, title TEXT, mtime INTEGER, hash TEXT, created_at TEXT, updated_at TEXT, frontmatter_json TEXT)",
            [],
        ).unwrap();

        // Insert test data
        conn.execute(
            "INSERT INTO notes (path, title, mtime, hash, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params!["test.md", "Test Note", 1234567890_i64, "hash123", "2024-01-01", "2024-01-02"],
        ).unwrap();

        // Test not found
        let result = get_note_by_filename(&conn, "nonexistent.md").unwrap();
        assert!(result.is_none());
    }
}
