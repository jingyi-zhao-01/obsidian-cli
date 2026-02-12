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
