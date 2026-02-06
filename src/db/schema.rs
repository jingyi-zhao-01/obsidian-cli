use rusqlite::Connection;

pub fn create_schema(conn: &Connection) -> rusqlite::Result<()> {
    // Notes table
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
    )?;

    // Links table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS links (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            src_note_id INTEGER NOT NULL,
            dst_text TEXT NOT NULL,
            dst_note_id INTEGER,
            kind TEXT NOT NULL,
            is_embed INTEGER NOT NULL DEFAULT 0,
            alias TEXT,
            heading_ref TEXT,
            block_ref TEXT,
            FOREIGN KEY (src_note_id) REFERENCES notes(id) ON DELETE CASCADE,
            FOREIGN KEY (dst_note_id) REFERENCES notes(id) ON DELETE SET NULL
        )",
        [],
    )?;

    // Tags table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tags (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            note_id INTEGER NOT NULL,
            tag TEXT NOT NULL,
            FOREIGN KEY (note_id) REFERENCES notes(id) ON DELETE CASCADE,
            UNIQUE(note_id, tag)
        )",
        [],
    )?;

    // Chunks table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS chunks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            note_id INTEGER NOT NULL,
            heading_path TEXT,
            text TEXT NOT NULL,
            byte_offset INTEGER NOT NULL,
            byte_length INTEGER NOT NULL,
            FOREIGN KEY (note_id) REFERENCES notes(id) ON DELETE CASCADE
        )",
        [],
    )?;

    // FTS5 virtual table for full-text search
    conn.execute(
        "CREATE VIRTUAL TABLE IF NOT EXISTS fts_chunks USING fts5(
            note_id UNINDEXED,
            heading_path,
            text,
            content=chunks,
            content_rowid=id
        )",
        [],
    )?;

    // Triggers to keep FTS5 in sync
    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS chunks_ai AFTER INSERT ON chunks BEGIN
            INSERT INTO fts_chunks(rowid, note_id, heading_path, text)
            VALUES (new.id, new.note_id, new.heading_path, new.text);
        END",
        [],
    )?;

    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS chunks_ad AFTER DELETE ON chunks BEGIN
            DELETE FROM fts_chunks WHERE rowid = old.id;
        END",
        [],
    )?;

    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS chunks_au AFTER UPDATE ON chunks BEGIN
            DELETE FROM fts_chunks WHERE rowid = old.id;
            INSERT INTO fts_chunks(rowid, note_id, heading_path, text)
            VALUES (new.id, new.note_id, new.heading_path, new.text);
        END",
        [],
    )?;

    // Create indexes
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_notes_path ON notes(path)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_notes_mtime ON notes(mtime)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_links_src ON links(src_note_id)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_links_dst ON links(dst_note_id)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_links_dst_text ON links(dst_text)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_tags_note ON tags(note_id)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_tags_tag ON tags(tag)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_chunks_note ON chunks(note_id)",
        [],
    )?;

    Ok(())
}

pub fn drop_tables(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute("DROP TABLE IF EXISTS fts_chunks", [])?;
    conn.execute("DROP TABLE IF EXISTS chunks", [])?;
    conn.execute("DROP TABLE IF EXISTS tags", [])?;
    conn.execute("DROP TABLE IF EXISTS links", [])?;
    conn.execute("DROP TABLE IF EXISTS notes", [])?;
    conn.execute("DROP TABLE IF EXISTS schema_version", [])?;
    Ok(())
}
