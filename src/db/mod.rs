use rusqlite::{Connection, OptionalExtension, Result, Transaction};
use std::path::Path;

mod operations;
mod schema;
mod stats;

pub use stats::DatabaseStats;

pub const SCHEMA_VERSION: i32 = 1;

#[derive(Debug, Clone)]
pub struct NoteMetadata {
    pub id: i64,
    pub mtime: i64,
    pub hash: String,
}

pub struct DatabaseTransaction<'a> {
    tx: Transaction<'a>,
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path)?;
        Ok(Database { conn })
    }

    pub fn transaction(&mut self) -> Result<DatabaseTransaction<'_>> {
        Ok(DatabaseTransaction {
            tx: self.conn.transaction()?,
        })
    }

    pub fn initialize(&self, force: bool) -> Result<()> {
        if force {
            schema::drop_tables(&self.conn)?;
        }

        // Create schema version table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS schema_version (
                version INTEGER PRIMARY KEY,
                applied_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        // Check current version
        let current_version: Option<i32> = self
            .conn
            .query_row(
                "SELECT version FROM schema_version ORDER BY version DESC LIMIT 1",
                [],
                |row| row.get(0),
            )
            .ok();

        if current_version.is_none() || force {
            schema::create_schema(&self.conn)?;
            self.conn.execute(
                "INSERT OR REPLACE INTO schema_version (version) VALUES (?1)",
                [SCHEMA_VERSION],
            )?;
        }

        Ok(())
    }

    pub fn get_version(&self) -> Result<Option<i32>> {
        self.conn
            .query_row(
                "SELECT version FROM schema_version ORDER BY version DESC LIMIT 1",
                [],
                |row| row.get(0),
            )
            .optional()
    }

    pub fn get_stats(&self) -> Result<DatabaseStats> {
        stats::get_stats(&self.conn)
    }

    pub fn insert_note(
        &self,
        path: &str,
        title: &str,
        mtime: u64,
        hash: &str,
        frontmatter_json: Option<&str>,
    ) -> Result<i64> {
        operations::insert_note(&self.conn, path, title, mtime, hash, frontmatter_json)
    }

    pub fn get_note_by_path(&self, path: &str) -> Result<Option<i64>> {
        operations::get_note_by_path(&self.conn, path)
    }

    pub fn get_note_metadata_by_path(&self, path: &str) -> Result<Option<NoteMetadata>> {
        operations::get_note_metadata_by_path(&self.conn, path)
    }

    pub fn insert_tag(&self, note_id: i64, tag: &str) -> Result<()> {
        operations::insert_tag(&self.conn, note_id, tag)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn insert_link(
        &self,
        src_note_id: i64,
        dst_text: &str,
        kind: &str,
        is_embed: bool,
        alias: Option<&str>,
        heading_ref: Option<&str>,
        block_ref: Option<&str>,
    ) -> Result<()> {
        operations::insert_link(
            &self.conn,
            src_note_id,
            dst_text,
            kind,
            is_embed,
            alias,
            heading_ref,
            block_ref,
        )
    }

    pub fn insert_chunk(&self, note_id: i64, heading_path: Option<&str>, text: &str) -> Result<()> {
        operations::insert_chunk(&self.conn, note_id, heading_path, text)
    }

    pub fn insert_chunk_with_offset(
        &self,
        note_id: i64,
        heading_path: Option<&str>,
        text: &str,
        byte_offset: i32,
        byte_length: i32,
    ) -> Result<()> {
        operations::insert_chunk_with_offset(
            &self.conn,
            note_id,
            heading_path,
            text,
            byte_offset,
            byte_length,
        )
    }

    pub fn clear_note_data(&self, note_id: i64) -> Result<()> {
        operations::clear_note_data(&self.conn, note_id)
    }

    /// Execute a query function with access to the database connection
    pub fn conn(&self) -> DatabaseQueryExecutor<'_> {
        DatabaseQueryExecutor { conn: &self.conn }
    }
}

impl DatabaseTransaction<'_> {
    pub fn insert_note(
        &self,
        path: &str,
        title: &str,
        mtime: u64,
        hash: &str,
        frontmatter_json: Option<&str>,
    ) -> Result<i64> {
        operations::insert_note(&self.tx, path, title, mtime, hash, frontmatter_json)
    }

    pub fn get_note_metadata_by_path(&self, path: &str) -> Result<Option<NoteMetadata>> {
        operations::get_note_metadata_by_path(&self.tx, path)
    }

    pub fn insert_tag(&self, note_id: i64, tag: &str) -> Result<()> {
        operations::insert_tag(&self.tx, note_id, tag)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn insert_link(
        &self,
        src_note_id: i64,
        dst_text: &str,
        kind: &str,
        is_embed: bool,
        alias: Option<&str>,
        heading_ref: Option<&str>,
        block_ref: Option<&str>,
    ) -> Result<()> {
        operations::insert_link(
            &self.tx,
            src_note_id,
            dst_text,
            kind,
            is_embed,
            alias,
            heading_ref,
            block_ref,
        )
    }

    pub fn insert_chunk_with_offset(
        &self,
        note_id: i64,
        heading_path: Option<&str>,
        text: &str,
        byte_offset: i32,
        byte_length: i32,
    ) -> Result<()> {
        operations::insert_chunk_with_offset(
            &self.tx,
            note_id,
            heading_path,
            text,
            byte_offset,
            byte_length,
        )
    }

    pub fn clear_note_data(&self, note_id: i64) -> Result<()> {
        operations::clear_note_data(&self.tx, note_id)
    }

    pub fn commit(self) -> Result<()> {
        self.tx.commit()
    }
}

pub struct DatabaseQueryExecutor<'a> {
    conn: &'a Connection,
}

impl DatabaseQueryExecutor<'_> {
    pub fn execute_query<T, F>(&self, f: F) -> Result<T>
    where
        F: FnOnce(&Connection) -> Result<T>,
    {
        f(self.conn)
    }
}
