use rusqlite::{Connection, Result};

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub chunk_id: i64,
    pub note_id: i64,
    pub note_path: String,
    pub note_title: String,
    pub heading_path: Option<String>,
    pub chunk_text: String,
    pub rank: f32,
}

/// Search chunks using FTS5 full-text search with BM25 ranking
pub fn search_chunks(
    conn: &Connection,
    query: &str,
    limit: usize,
) -> Result<Vec<SearchResult>> {
    let mut stmt = conn.prepare(
        "SELECT 
            c.id,
            n.id,
            n.path,
            n.title,
            c.heading_path,
            c.text,
            rank
         FROM fts_chunks fc
         JOIN chunks c ON fc.rowid = c.id
         JOIN notes n ON c.note_id = n.id
         WHERE fts_chunks MATCH ?1
         ORDER BY rank
         LIMIT ?2"
    )?;

    let results = stmt.query_map([query, &limit.to_string()], |row| {
        Ok(SearchResult {
            chunk_id: row.get(0)?,
            note_id: row.get(1)?,
            note_path: row.get(2)?,
            note_title: row.get(3)?,
            heading_path: row.get(4)?,
            chunk_text: row.get(5)?,
            rank: row.get(6)?,
        })
    })?;

    let mut search_results = Vec::new();
    for result in results {
        search_results.push(result?);
    }

    Ok(search_results)
}
