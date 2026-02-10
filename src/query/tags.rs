use rusqlite::{Connection, Result};

#[derive(Debug, Clone)]
pub struct TagResult {
    pub note_id: i64,
    pub note_path: String,
    pub note_title: String,
    pub tags: Vec<String>,
}

fn get_tags_for_note(conn: &Connection, note_id: i64) -> Result<Vec<String>> {
    let mut tag_stmt = conn.prepare("SELECT tag FROM tags WHERE note_id = ?1 ORDER BY tag")?;
    let tags = tag_stmt.query_map([note_id], |row| row.get::<_, String>(0))?;

    let mut tag_list = Vec::new();
    for tag_result in tags {
        tag_list.push(tag_result?);
    }

    Ok(tag_list)
}

fn collect_string_column(
    rows: rusqlite::MappedRows<impl FnMut(&rusqlite::Row<'_>) -> Result<String>>,
) -> Result<Vec<String>> {
    let mut values = Vec::new();
    for row in rows {
        values.push(row?);
    }
    Ok(values)
}

fn collect_tag_results(
    conn: &Connection,
    rows: impl Iterator<Item = Result<(i64, String, String)>>,
) -> Result<Vec<TagResult>> {
    let mut notes = Vec::new();
    for row in rows {
        let (note_id, note_path, note_title) = row?;
        let tag_list = get_tags_for_note(conn, note_id)?;
        notes.push(TagResult {
            note_id,
            note_path,
            note_title,
            tags: tag_list,
        });
    }
    Ok(notes)
}

/// List all unique tags in the vault
pub fn list_tags(conn: &Connection) -> Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT DISTINCT tag FROM tags ORDER BY tag")?;

    let results = stmt.query_map([], |row| row.get::<_, String>(0))?;
    collect_string_column(results)
}

/// Get all notes that have a specific tag
pub fn get_notes_by_tag(conn: &Connection, tag: &str) -> Result<Vec<TagResult>> {
    let mut stmt = conn.prepare(
        "SELECT DISTINCT
            n.id,
            n.path,
            n.title
         FROM notes n
         JOIN tags t ON n.id = t.note_id
         WHERE t.tag = ?1
         ORDER BY n.path",
    )?;

    let note_rows = stmt.query_map([tag], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
        ))
    })?;
    collect_tag_results(conn, note_rows)
}

/// Get all notes that have ALL of the specified tags (AND intersection)
pub fn get_notes_by_tags_and(conn: &Connection, tags: &[&str]) -> Result<Vec<TagResult>> {
    if tags.is_empty() {
        return Ok(Vec::new());
    }

    let placeholders = tags.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT DISTINCT
            n.id,
            n.path,
            n.title
         FROM notes n
         WHERE n.id IN (
             SELECT note_id FROM tags WHERE tag IN ({})
             GROUP BY note_id HAVING COUNT(DISTINCT tag) = {}
         )
         ORDER BY n.path",
        placeholders,
        tags.len()
    );

    let mut stmt = conn.prepare(&query)?;

    let params: Vec<&dyn rusqlite::ToSql> =
        tags.iter().map(|t| t as &dyn rusqlite::ToSql).collect();
    let rows = stmt.query_map(params.as_slice(), |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
        ))
    })?;
    collect_tag_results(conn, rows)
}

/// Get all notes that have ANY of the specified tags (OR union)
pub fn get_notes_by_tags_or(conn: &Connection, tags: &[&str]) -> Result<Vec<TagResult>> {
    if tags.is_empty() {
        return Ok(Vec::new());
    }

    let placeholders = tags.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT DISTINCT
            n.id,
            n.path,
            n.title
         FROM notes n
         WHERE n.id IN (
             SELECT note_id FROM tags WHERE tag IN ({})
         )
         ORDER BY n.path",
        placeholders
    );

    let mut stmt = conn.prepare(&query)?;

    let params: Vec<&dyn rusqlite::ToSql> =
        tags.iter().map(|t| t as &dyn rusqlite::ToSql).collect();
    let rows = stmt.query_map(params.as_slice(), |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
        ))
    })?;
    collect_tag_results(conn, rows)
}
