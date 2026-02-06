use rusqlite::Connection;

#[derive(Debug)]
pub struct DatabaseStats {
    pub note_count: usize,
    pub link_count: usize,
    pub tag_count: usize,
    pub chunk_count: usize,
    pub unresolved_links: usize,
}

pub fn get_stats(conn: &Connection) -> rusqlite::Result<DatabaseStats> {
    let note_count: i32 = conn
        .query_row("SELECT COUNT(*) FROM notes", [], |row| row.get(0))?;

    let link_count: i32 = conn
        .query_row("SELECT COUNT(*) FROM links", [], |row| row.get(0))?;

    let tag_count: i32 = conn.query_row(
        "SELECT COUNT(DISTINCT tag) FROM tags",
        [],
        |row| row.get(0),
    )?;

    let chunk_count: i32 = conn
        .query_row("SELECT COUNT(*) FROM chunks", [], |row| row.get(0))?;

    let unresolved_links: i32 = conn.query_row(
        "SELECT COUNT(*) FROM links WHERE dst_note_id IS NULL",
        [],
        |row| row.get(0),
    )?;

    Ok(DatabaseStats {
        note_count: note_count as usize,
        link_count: link_count as usize,
        tag_count: tag_count as usize,
        chunk_count: chunk_count as usize,
        unresolved_links: unresolved_links as usize,
    })
}
