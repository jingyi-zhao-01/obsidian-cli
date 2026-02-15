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

/// Result for orphan and dead-end analysis
#[derive(Debug, Clone)]
pub struct DiagnoseResult {
    pub note_id: i64,
    pub note_path: String,
    pub note_title: String,
    pub incoming_count: i64,
    pub outgoing_count: i64,
}

/// Result for broken link diagnosis
#[derive(Debug, Clone)]
pub struct BrokenLinkResult {
    pub src_path: String,
    pub src_title: String,
    pub raw_link: String,
    pub target: String,
    pub status: String,          // "unresolved" or "ambiguous"
    pub candidates: Vec<String>, // list of candidate note paths
}

/// Get all broken links (unresolved and ambiguous)
/// Unresolved: links where dst_note_id is NULL (target doesn't exist)
/// Ambiguous: links where dst_text could match multiple notes
pub fn diagnose_broken_links(conn: &Connection) -> Result<Vec<BrokenLinkResult>> {
    let mut results = Vec::new();

    // First, get unresolved links (dst_note_id IS NULL)
    let mut stmt = conn.prepare(
        "SELECT 
            src.path as src_path,
            src.title as src_title,
            l.alias as raw_link,
            l.dst_text as target
         FROM links l
         JOIN notes src ON l.src_note_id = src.id
         WHERE l.dst_note_id IS NULL
         ORDER BY src.path, l.dst_text",
    )?;

    let unresolved_iter = stmt.query_map([], |row| {
        Ok(BrokenLinkResult {
            src_path: row.get(0)?,
            src_title: row.get(1)?,
            raw_link: row.get::<_, Option<String>>(2)?.unwrap_or_default(),
            target: row.get(3)?,
            status: "unresolved".to_string(),
            candidates: Vec::new(),
        })
    })?;

    for result in unresolved_iter {
        results.push(result?);
    }

    // Now find ambiguous links - links where dst_text could match multiple notes
    // This happens when the link target matches multiple notes (e.g., same basename in different folders)
    let mut stmt = conn.prepare(
        "SELECT 
            l.id as link_id,
            src.path as src_path,
            src.title as src_title,
            l.alias as raw_link,
            l.dst_text as target
         FROM links l
         JOIN notes src ON l.src_note_id = src.id
         WHERE l.dst_note_id IS NOT NULL",
    )?;

    let links_iter = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, Option<String>>(3)?.unwrap_or_default(),
            row.get::<_, String>(4)?,
        ))
    })?;

    // For each resolved link, check if the target is ambiguous
    let mut stmt_candidates =
        conn.prepare("SELECT path FROM notes WHERE path = ?1 OR title = ?1 LIMIT 2")?;

    for link_result in links_iter {
        let (_link_id, src_path, src_title, raw_link, target) = link_result?;

        // Check how many notes match this target
        let candidates: Vec<String> = stmt_candidates
            .query_map([&target], |row| row.get(0))?
            .filter_map(|r| r.ok())
            .collect();

        // If there are multiple matches, mark as ambiguous
        if candidates.len() > 1 {
            results.push(BrokenLinkResult {
                src_path,
                src_title,
                raw_link,
                target,
                status: "ambiguous".to_string(),
                candidates,
            });
        }
    }

    Ok(results)
}

/// Get orphan notes (no incoming AND no outgoing links)
/// Optionally exclude templates and daily notes
pub fn get_orphans(
    conn: &Connection,
    exclude_templates: bool,
    exclude_daily: bool,
) -> Result<Vec<DiagnoseResult>> {
    let mut query = String::from(
        "SELECT 
            n.id,
            n.path,
            n.title,
            (SELECT COUNT(*) FROM links l WHERE l.dst_note_id = n.id) as incoming_count,
            (SELECT COUNT(*) FROM links l WHERE l.src_note_id = n.id) as outgoing_count
         FROM notes n
         WHERE (SELECT COUNT(*) FROM links l WHERE l.dst_note_id = n.id) = 0
         AND (SELECT COUNT(*) FROM links l WHERE l.src_note_id = n.id) = 0",
    );

    if exclude_templates {
        query.push_str(" AND n.path NOT LIKE 'templates/%' AND n.path NOT LIKE '%/templates/%' AND n.path NOT LIKE '%/template%'");
    }

    if exclude_daily {
        query.push_str(" AND n.path NOT LIKE 'daily/%' AND n.path NOT LIKE '%/daily/%' AND n.title NOT LIKE '%Daily%'");
    }

    query.push_str(" ORDER BY n.path");

    let mut stmt = conn.prepare(&query)?;
    let results = stmt.query_map([], |row| {
        Ok(DiagnoseResult {
            note_id: row.get(0)?,
            note_path: row.get(1)?,
            note_title: row.get(2)?,
            incoming_count: row.get(3)?,
            outgoing_count: row.get(4)?,
        })
    })?;

    let mut orphans = Vec::new();
    for result in results {
        orphans.push(result?);
    }

    Ok(orphans)
}

/// Get dead-end notes (has incoming but no outgoing links)
/// Optionally exclude templates and daily notes
pub fn get_dead_ends(
    conn: &Connection,
    exclude_templates: bool,
    exclude_daily: bool,
) -> Result<Vec<DiagnoseResult>> {
    let mut query = String::from(
        "SELECT 
            n.id,
            n.path,
            n.title,
            (SELECT COUNT(*) FROM links l WHERE l.dst_note_id = n.id) as incoming_count,
            (SELECT COUNT(*) FROM links l WHERE l.src_note_id = n.id) as outgoing_count
         FROM notes n
         WHERE (SELECT COUNT(*) FROM links l WHERE l.dst_note_id = n.id) > 0
         AND (SELECT COUNT(*) FROM links l WHERE l.src_note_id = n.id) = 0",
    );

    if exclude_templates {
        query.push_str(" AND n.path NOT LIKE 'templates/%' AND n.path NOT LIKE '%/templates/%' AND n.path NOT LIKE '%/template%'");
    }

    if exclude_daily {
        query.push_str(" AND n.path NOT LIKE 'daily/%' AND n.path NOT LIKE '%/daily/%' AND n.title NOT LIKE '%Daily%'");
    }

    query.push_str(" ORDER BY n.path");

    let mut stmt = conn.prepare(&query)?;
    let results = stmt.query_map([], |row| {
        Ok(DiagnoseResult {
            note_id: row.get(0)?,
            note_path: row.get(1)?,
            note_title: row.get(2)?,
            incoming_count: row.get(3)?,
            outgoing_count: row.get(4)?,
        })
    })?;

    let mut dead_ends = Vec::new();
    for result in results {
        dead_ends.push(result?);
    }

    Ok(dead_ends)
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

    // Tests for diagnose_broken_links
    #[test]
    fn test_broken_link_result_creation() {
        let result = BrokenLinkResult {
            src_path: "source.md".to_string(),
            src_title: "Source".to_string(),
            raw_link: "[[target|alias]]".to_string(),
            target: "target".to_string(),
            status: "unresolved".to_string(),
            candidates: vec![],
        };

        assert_eq!(result.status, "unresolved");
        assert!(result.candidates.is_empty());
    }

    #[test]
    fn test_broken_link_result_ambiguous() {
        let result = BrokenLinkResult {
            src_path: "source.md".to_string(),
            src_title: "Source".to_string(),
            raw_link: "[[duplicate]]".to_string(),
            target: "duplicate".to_string(),
            status: "ambiguous".to_string(),
            candidates: vec![
                "folder1/duplicate.md".to_string(),
                "folder2/duplicate.md".to_string(),
            ],
        };

        assert_eq!(result.status, "ambiguous");
        assert_eq!(result.candidates.len(), 2);
    }

    #[test]
    fn test_diagnose_broken_links_unresolved() {
        let conn = Connection::open_in_memory().unwrap();
        setup_test_db(&conn);

        // Insert an unresolved link from test1 to nonexistent
        conn.execute(
            "INSERT INTO links (src_note_id, dst_note_id, dst_text, is_embed) VALUES (1, NULL, 'nonexistent.md', 0)",
            [],
        ).unwrap();

        let broken = diagnose_broken_links(&conn).unwrap();
        assert_eq!(broken.len(), 1);
        assert_eq!(broken[0].status, "unresolved");
        assert_eq!(broken[0].target, "nonexistent.md");
    }

    #[test]
    fn test_diagnose_broken_links_resolved() {
        let conn = Connection::open_in_memory().unwrap();
        setup_test_db(&conn);

        // Insert a resolved link from test1 to test2
        conn.execute(
            "INSERT INTO links (src_note_id, dst_note_id, dst_text, is_embed) VALUES (1, 2, 'test2.md', 0)",
            [],
        ).unwrap();

        let broken = diagnose_broken_links(&conn).unwrap();
        assert!(broken.is_empty());
    }

    #[test]
    fn test_diagnose_broken_links_empty() {
        let conn = Connection::open_in_memory().unwrap();
        setup_test_db(&conn);

        // No links at all
        let broken = diagnose_broken_links(&conn).unwrap();
        assert!(broken.is_empty());
    }

    // Tests for DiagnoseResult
    #[test]
    fn test_diagnose_result_creation() {
        let result = DiagnoseResult {
            note_id: 1,
            note_path: "test.md".to_string(),
            note_title: "Test".to_string(),
            incoming_count: 5,
            outgoing_count: 3,
        };

        assert_eq!(result.note_id, 1);
        assert_eq!(result.incoming_count, 5);
        assert_eq!(result.outgoing_count, 3);
    }

    // Tests for get_orphans
    #[test]
    fn test_get_orphans_with_orphans() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute(
            "CREATE TABLE notes (id INTEGER PRIMARY KEY, path TEXT, title TEXT)",
            [],
        )
        .unwrap();
        conn.execute(
            "CREATE TABLE links (id INTEGER PRIMARY KEY, src_note_id INTEGER, dst_note_id INTEGER, dst_text TEXT, is_embed INTEGER, alias TEXT, heading_ref TEXT, block_ref TEXT)",
            [],
        ).unwrap();

        // Insert notes - only test1 and test2 will have links
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
            "INSERT INTO notes (path, title) VALUES ('orphan.md', 'Orphan')",
            [],
        )
        .unwrap();

        // Add a link between test1 and test2
        conn.execute(
            "INSERT INTO links (src_note_id, dst_note_id, dst_text, is_embed) VALUES (1, 2, 'test2.md', 0)",
            [],
        ).unwrap();

        // orphan.md has no links - should be orphan
        let orphans = get_orphans(&conn, false, false).unwrap();
        assert_eq!(orphans.len(), 1);
        assert_eq!(orphans[0].note_path, "orphan.md");
    }

    #[test]
    fn test_get_orphans_no_orphans() {
        let conn = Connection::open_in_memory().unwrap();
        setup_test_db(&conn);

        // Add links to make all notes connected
        conn.execute(
            "INSERT INTO links (src_note_id, dst_note_id, dst_text, is_embed) VALUES (1, 2, 'test2.md', 0)",
            [],
        ).unwrap();
        conn.execute(
            "INSERT INTO links (src_note_id, dst_note_id, dst_text, is_embed) VALUES (2, 3, 'test3.md', 0)",
            [],
        ).unwrap();
        conn.execute(
            "INSERT INTO links (src_note_id, dst_note_id, dst_text, is_embed) VALUES (3, 1, 'test1.md', 0)",
            [],
        ).unwrap();

        let orphans = get_orphans(&conn, false, false).unwrap();
        assert!(orphans.is_empty());
    }

    #[test]
    fn test_get_orphans_exclude_templates() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute(
            "CREATE TABLE notes (id INTEGER PRIMARY KEY, path TEXT, title TEXT)",
            [],
        )
        .unwrap();
        conn.execute(
            "CREATE TABLE links (id INTEGER PRIMARY KEY, src_note_id INTEGER, dst_note_id INTEGER, dst_text TEXT, is_embed INTEGER, alias TEXT, heading_ref TEXT, block_ref TEXT)",
            [],
        ).unwrap();

        // Insert regular orphan
        conn.execute(
            "INSERT INTO notes (path, title) VALUES ('regular.md', 'Regular')",
            [],
        )
        .unwrap();
        // Insert template orphan
        conn.execute(
            "INSERT INTO notes (path, title) VALUES ('templates/template1.md', 'Template1')",
            [],
        )
        .unwrap();

        // Without exclude
        let orphans = get_orphans(&conn, false, false).unwrap();
        assert_eq!(orphans.len(), 2);

        // With exclude templates
        let orphans = get_orphans(&conn, true, false).unwrap();
        assert_eq!(orphans.len(), 1);
        assert_eq!(orphans[0].note_path, "regular.md");
    }

    #[test]
    fn test_get_orphans_exclude_daily() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute(
            "CREATE TABLE notes (id INTEGER PRIMARY KEY, path TEXT, title TEXT)",
            [],
        )
        .unwrap();
        conn.execute(
            "CREATE TABLE links (id INTEGER PRIMARY KEY, src_note_id INTEGER, dst_note_id INTEGER, dst_text TEXT, is_embed INTEGER, alias TEXT, heading_ref TEXT, block_ref TEXT)",
            [],
        ).unwrap();

        // Insert regular orphan
        conn.execute(
            "INSERT INTO notes (path, title) VALUES ('regular.md', 'Regular')",
            [],
        )
        .unwrap();
        // Insert daily orphan
        conn.execute(
            "INSERT INTO notes (path, title) VALUES ('daily/2024-01-01.md', 'Daily Notes')",
            [],
        )
        .unwrap();

        // Without exclude
        let orphans = get_orphans(&conn, false, false).unwrap();
        assert_eq!(orphans.len(), 2);

        // With exclude daily
        let orphans = get_orphans(&conn, false, true).unwrap();
        assert_eq!(orphans.len(), 1);
        assert_eq!(orphans[0].note_path, "regular.md");
    }

    // Tests for get_dead_ends
    #[test]
    fn test_get_dead_ends_with_dead_ends() {
        let conn = Connection::open_in_memory().unwrap();
        setup_test_db(&conn);

        // test3 has incoming from test1 but no outgoing
        conn.execute(
            "INSERT INTO links (src_note_id, dst_note_id, dst_text, is_embed) VALUES (1, 3, 'test3.md', 0)",
            [],
        ).unwrap();

        let dead_ends = get_dead_ends(&conn, false, false).unwrap();
        assert_eq!(dead_ends.len(), 1);
        assert_eq!(dead_ends[0].note_path, "test3.md");
    }

    #[test]
    fn test_get_dead_ends_no_dead_ends() {
        let conn = Connection::open_in_memory().unwrap();
        setup_test_db(&conn);

        // Create a fully connected graph
        conn.execute(
            "INSERT INTO links (src_note_id, dst_note_id, dst_text, is_embed) VALUES (1, 2, 'test2.md', 0)",
            [],
        ).unwrap();
        conn.execute(
            "INSERT INTO links (src_note_id, dst_note_id, dst_text, is_embed) VALUES (2, 3, 'test3.md', 0)",
            [],
        ).unwrap();
        conn.execute(
            "INSERT INTO links (src_note_id, dst_note_id, dst_text, is_embed) VALUES (3, 1, 'test1.md', 0)",
            [],
        ).unwrap();

        let dead_ends = get_dead_ends(&conn, false, false).unwrap();
        assert!(dead_ends.is_empty());
    }

    #[test]
    fn test_get_dead_ends_exclude_templates() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute(
            "CREATE TABLE notes (id INTEGER PRIMARY KEY, path TEXT, title TEXT)",
            [],
        )
        .unwrap();
        conn.execute(
            "CREATE TABLE links (id INTEGER PRIMARY KEY, src_note_id INTEGER, dst_note_id INTEGER, dst_text TEXT, is_embed INTEGER, alias TEXT, heading_ref TEXT, block_ref TEXT)",
            [],
        ).unwrap();

        // Insert regular dead-end (has incoming, no outgoing)
        conn.execute(
            "INSERT INTO notes (id, path, title) VALUES (1, 'source.md', 'Source')",
            [],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO notes (id, path, title) VALUES (2, 'regular.md', 'Regular')",
            [],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO notes (id, path, title) VALUES (3, 'templates/tmpl.md', 'Template')",
            [],
        )
        .unwrap();

        // Source links to both regular and template
        conn.execute(
            "INSERT INTO links (src_note_id, dst_note_id, dst_text, is_embed) VALUES (1, 2, 'regular.md', 0)",
            [],
        ).unwrap();
        conn.execute(
            "INSERT INTO links (src_note_id, dst_note_id, dst_text, is_embed) VALUES (1, 3, 'tmpl.md', 0)",
            [],
        ).unwrap();

        // Without exclude - both regular and template are dead-ends
        let dead_ends = get_dead_ends(&conn, false, false).unwrap();
        assert_eq!(dead_ends.len(), 2);

        // With exclude templates - only regular should be returned
        let dead_ends = get_dead_ends(&conn, true, false).unwrap();
        assert_eq!(dead_ends.len(), 1);
        assert_eq!(dead_ends[0].note_path, "regular.md");
    }

    #[test]
    fn test_get_dead_ends_exclude_daily() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute(
            "CREATE TABLE notes (id INTEGER PRIMARY KEY, path TEXT, title TEXT)",
            [],
        )
        .unwrap();
        conn.execute(
            "CREATE TABLE links (id INTEGER PRIMARY KEY, src_note_id INTEGER, dst_note_id INTEGER, dst_text TEXT, is_embed INTEGER, alias TEXT, heading_ref TEXT, block_ref TEXT)",
            [],
        ).unwrap();

        // Insert notes
        conn.execute(
            "INSERT INTO notes (id, path, title) VALUES (1, 'source.md', 'Source')",
            [],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO notes (id, path, title) VALUES (2, 'regular.md', 'Regular')",
            [],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO notes (id, path, title) VALUES (3, 'daily/2024-01-01.md', 'Daily Notes')",
            [],
        )
        .unwrap();

        // Source links to both
        conn.execute(
            "INSERT INTO links (src_note_id, dst_note_id, dst_text, is_embed) VALUES (1, 2, 'regular.md', 0)",
            [],
        ).unwrap();
        conn.execute(
            "INSERT INTO links (src_note_id, dst_note_id, dst_text, is_embed) VALUES (1, 3, '2024-01-01.md', 0)",
            [],
        ).unwrap();

        // Without exclude
        let dead_ends = get_dead_ends(&conn, false, false).unwrap();
        assert_eq!(dead_ends.len(), 2);

        // With exclude daily
        let dead_ends = get_dead_ends(&conn, false, true).unwrap();
        assert_eq!(dead_ends.len(), 1);
        assert_eq!(dead_ends[0].note_path, "regular.md");
    }

    // Edge case tests
    #[test]
    fn test_get_orphans_empty_database() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute(
            "CREATE TABLE notes (id INTEGER PRIMARY KEY, path TEXT, title TEXT)",
            [],
        )
        .unwrap();
        conn.execute(
            "CREATE TABLE links (id INTEGER PRIMARY KEY, src_note_id INTEGER, dst_note_id INTEGER, dst_text TEXT, is_embed INTEGER, alias TEXT, heading_ref TEXT, block_ref TEXT)",
            [],
        ).unwrap();

        let orphans = get_orphans(&conn, false, false).unwrap();
        assert!(orphans.is_empty());
    }

    #[test]
    fn test_get_dead_ends_empty_database() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute(
            "CREATE TABLE notes (id INTEGER PRIMARY KEY, path TEXT, title TEXT)",
            [],
        )
        .unwrap();
        conn.execute(
            "CREATE TABLE links (id INTEGER PRIMARY KEY, src_note_id INTEGER, dst_note_id INTEGER, dst_text TEXT, is_embed INTEGER, alias TEXT, heading_ref TEXT, block_ref TEXT)",
            [],
        ).unwrap();

        let dead_ends = get_dead_ends(&conn, false, false).unwrap();
        assert!(dead_ends.is_empty());
    }

    #[test]
    fn test_diagnose_broken_links_multiple_unresolved() {
        let conn = Connection::open_in_memory().unwrap();
        setup_test_db(&conn);

        // Insert multiple unresolved links
        conn.execute(
            "INSERT INTO links (src_note_id, dst_note_id, dst_text, is_embed) VALUES (1, NULL, 'nonexistent1.md', 0)",
            [],
        ).unwrap();
        conn.execute(
            "INSERT INTO links (src_note_id, dst_note_id, dst_text, is_embed) VALUES (1, NULL, 'nonexistent2.md', 0)",
            [],
        ).unwrap();
        conn.execute(
            "INSERT INTO links (src_note_id, dst_note_id, dst_text, is_embed) VALUES (2, NULL, 'nonexistent3.md', 0)",
            [],
        ).unwrap();

        let broken = diagnose_broken_links(&conn).unwrap();
        assert_eq!(broken.len(), 3);

        // All should be unresolved
        for link in &broken {
            assert_eq!(link.status, "unresolved");
        }
    }
}
