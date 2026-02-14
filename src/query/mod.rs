// Query module for retrieving and searching vault data
mod links;
mod search;
mod tags;

pub use links::{get_backlinks, get_forward_links, get_unresolved_links, LinkResult};
pub use search::{search_chunks, SearchResult};
pub use tags::{
    get_notes_by_tag, get_notes_by_tags_and, get_notes_by_tags_or, list_tags, TagResult,
};

use rusqlite::Connection;
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
pub fn get_note_by_filename(conn: &Connection, filename: &str) -> rusqlite::Result<Option<NoteDescribeResult>> {
    // First try to find by exact path match
    let result = conn.query_row(
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
    ).optional();

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
        ).optional()
    } else {
        result
    }
}
