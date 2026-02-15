use rusqlite::{Connection, OptionalExtension, Result};

#[derive(Debug, Clone)]
pub struct LinkResult {
    pub note_id: i64,
    pub note_path: String,
    pub note_title: String,
    pub is_embed: bool,
    pub alias: Option<String>,
    pub heading_ref: Option<String>,
    pub block_ref: Option<String>,
}

/// Get all notes that link to a given note (backlinks)
pub fn get_backlinks(conn: &Connection, note_path: &str) -> Result<Vec<LinkResult>> {
    // First find the target note
    let target_note_id: Option<i64> = conn
        .query_row("SELECT id FROM notes WHERE path = ?1", [note_path], |row| {
            row.get(0)
        })
        .optional()?;

    if target_note_id.is_none() {
        return Ok(Vec::new());
    }

    let target_note_id = target_note_id.unwrap();

    // Get all links pointing to this note
    let mut stmt = conn.prepare(
        "SELECT 
            src.id,
            src.path,
            src.title,
            l.is_embed,
            l.alias,
            l.heading_ref,
            l.block_ref
         FROM links l
         JOIN notes src ON l.src_note_id = src.id
         WHERE l.dst_note_id = ?1
         ORDER BY src.path",
    )?;

    let results = stmt.query_map([target_note_id], |row| {
        Ok(LinkResult {
            note_id: row.get(0)?,
            note_path: row.get(1)?,
            note_title: row.get(2)?,
            is_embed: row.get::<_, i32>(3)? != 0,
            alias: row.get(4)?,
            heading_ref: row.get(5)?,
            block_ref: row.get(6)?,
        })
    })?;

    let mut backlinks = Vec::new();
    for result in results {
        backlinks.push(result?);
    }

    Ok(backlinks)
}

/// Get all notes that a given note links to (forward links)
pub fn get_forward_links(conn: &Connection, note_path: &str) -> Result<Vec<LinkResult>> {
    // First find the source note
    let src_note_id: Option<i64> = conn
        .query_row("SELECT id FROM notes WHERE path = ?1", [note_path], |row| {
            row.get(0)
        })
        .optional()?;

    if src_note_id.is_none() {
        return Ok(Vec::new());
    }

    let src_note_id = src_note_id.unwrap();

    // Get all links from this note
    let mut stmt = conn.prepare(
        "SELECT 
            COALESCE(dst.id, -1),
            COALESCE(dst.path, l.dst_text),
            COALESCE(dst.title, l.dst_text),
            l.is_embed,
            l.alias,
            l.heading_ref,
            l.block_ref
         FROM links l
         LEFT JOIN notes dst ON l.dst_note_id = dst.id
         WHERE l.src_note_id = ?1
         ORDER BY l.dst_text",
    )?;

    let results = stmt.query_map([src_note_id], |row| {
        Ok(LinkResult {
            note_id: row.get(0)?,
            note_path: row.get(1)?,
            note_title: row.get(2)?,
            is_embed: row.get::<_, i32>(3)? != 0,
            alias: row.get(4)?,
            heading_ref: row.get(5)?,
            block_ref: row.get(6)?,
        })
    })?;

    let mut forward_links = Vec::new();
    for result in results {
        forward_links.push(result?);
    }

    Ok(forward_links)
}

/// Get all unresolved links (links pointing to non-existent notes)
pub fn get_unresolved_links(conn: &Connection) -> Result<Vec<LinkResult>> {
    let mut stmt = conn.prepare(
        "SELECT 
            src.id,
            src.path,
            src.title,
            l.is_embed,
            l.alias,
            l.heading_ref,
            l.block_ref,
            l.dst_text
         FROM links l
         JOIN notes src ON l.src_note_id = src.id
         WHERE l.dst_note_id IS NULL
         ORDER BY l.dst_text, src.path",
    )?;

    let results = stmt.query_map([], |row| {
        Ok(LinkResult {
            note_id: row.get(0)?,
            note_path: row.get(1)?,
            note_title: row.get(2)?,
            is_embed: row.get::<_, i32>(3)? != 0,
            alias: row.get(4)?,
            heading_ref: row.get(5)?,
            block_ref: row.get(6)?,
        })
    })?;

    let mut unresolved = Vec::new();
    for result in results {
        unresolved.push(result?);
    }

    Ok(unresolved)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn setup_test_db(conn: &Connection) {
        conn.execute(
            "CREATE TABLE notes (id INTEGER PRIMARY KEY, path TEXT, title TEXT)",
            [],
        )
        .unwrap();
        conn.execute(
            "CREATE TABLE links (id INTEGER PRIMARY KEY, src_note_id INTEGER, dst_note_id INTEGER, dst_text TEXT, is_embed INTEGER, alias TEXT, heading_ref TEXT, block_ref TEXT)",
            [],
        ).unwrap();

        // Insert test notes
        conn.execute(
            "INSERT INTO notes (path, title) VALUES ('test1.md', 'Test 1')",
            [],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO notes (path, title) VALUES ('test2.md', 'Test 2')",
            [],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO notes (path, title) VALUES ('test3.md', 'Test 3')",
            [],
        )
        .unwrap();
    }

    #[test]
    fn test_link_result_creation() {
        let link = LinkResult {
            note_id: 1,
            note_path: "test.md".to_string(),
            note_title: "Test".to_string(),
            is_embed: false,
            alias: Some("alias".to_string()),
            heading_ref: Some("heading".to_string()),
            block_ref: Some("block".to_string()),
        };

        assert_eq!(link.note_id, 1);
        assert!(!link.is_embed);
        assert!(link.alias.is_some());
    }

    #[test]
    fn test_link_result_no_optionals() {
        let link = LinkResult {
            note_id: 1,
            note_path: "test.md".to_string(),
            note_title: "Test".to_string(),
            is_embed: true,
            alias: None,
            heading_ref: None,
            block_ref: None,
        };

        assert!(link.is_embed);
        assert!(link.alias.is_none());
    }

    #[test]
    fn test_get_backlinks_with_results() {
        let conn = Connection::open_in_memory().unwrap();
        setup_test_db(&conn);

        // Insert a link from test1 to test2
        conn.execute(
            "INSERT INTO links (src_note_id, dst_note_id, dst_text, is_embed) VALUES (1, 2, 'test2.md', 0)",
            [],
        ).unwrap();

        let backlinks = get_backlinks(&conn, "test2.md").unwrap();
        assert_eq!(backlinks.len(), 1);
        assert_eq!(backlinks[0].note_path, "test1.md");
    }

    #[test]
    fn test_get_backlinks_no_results() {
        let conn = Connection::open_in_memory().unwrap();
        setup_test_db(&conn);

        let backlinks = get_backlinks(&conn, "test2.md").unwrap();
        assert!(backlinks.is_empty());
    }

    #[test]
    fn test_get_forward_links_with_results() {
        let conn = Connection::open_in_memory().unwrap();
        setup_test_db(&conn);

        // Insert a link from test1 to test2
        conn.execute(
            "INSERT INTO links (src_note_id, dst_note_id, dst_text, is_embed) VALUES (1, 2, 'test2.md', 0)",
            [],
        ).unwrap();

        let forward_links = get_forward_links(&conn, "test1.md").unwrap();
        assert_eq!(forward_links.len(), 1);
    }

    #[test]
    fn test_get_forward_links_unresolved() {
        let conn = Connection::open_in_memory().unwrap();
        setup_test_db(&conn);

        // Insert an unresolved link from test1 to nonexistent
        conn.execute(
            "INSERT INTO links (src_note_id, dst_note_id, dst_text, is_embed) VALUES (1, NULL, 'nonexistent.md', 0)",
            [],
        ).unwrap();

        let forward_links = get_forward_links(&conn, "test1.md").unwrap();
        assert_eq!(forward_links.len(), 1);
        assert_eq!(forward_links[0].note_id, -1); // Unresolved
    }

    #[test]
    fn test_get_unresolved_links() {
        let conn = Connection::open_in_memory().unwrap();
        setup_test_db(&conn);

        // Insert an unresolved link
        conn.execute(
            "INSERT INTO links (src_note_id, dst_note_id, dst_text, is_embed) VALUES (1, NULL, 'nonexistent.md', 0)",
            [],
        ).unwrap();

        let unresolved = get_unresolved_links(&conn).unwrap();
        assert_eq!(unresolved.len(), 1);
    }
}
