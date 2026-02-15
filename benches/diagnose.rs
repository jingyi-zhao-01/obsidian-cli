//! Benchmark for diagnose commands (orphans, dead-ends, broken links)
//!
//! Run with: cargo bench

use criterion::{criterion_group, criterion_main, Criterion};
use obsidian_cli_inspector::query::{diagnose_broken_links, get_dead_ends, get_orphans};
use rusqlite::Connection;

fn setup_large_db(conn: &Connection, num_notes: usize, num_links: usize) {
    // Create tables
    conn.execute(
        "CREATE TABLE IF NOT EXISTS notes (
            id INTEGER PRIMARY KEY,
            path TEXT,
            title TEXT,
            mtime INTEGER,
            hash TEXT,
            frontmatter_json TEXT,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )
    .unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS links (
            id INTEGER PRIMARY KEY,
            src_note_id INTEGER,
            dst_note_id INTEGER,
            dst_text TEXT,
            kind TEXT,
            is_embed INTEGER DEFAULT 0,
            alias TEXT,
            heading_ref TEXT,
            block_ref TEXT
        )",
        [],
    )
    .unwrap();

    // Insert notes
    for i in 0..num_notes {
        conn.execute(
            "INSERT INTO notes (path, title, mtime, hash) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![
                format!("note_{}.md", i),
                format!("Note {}", i),
                i as i64,
                format!("hash_{}", i)
            ],
        )
        .unwrap();
    }

    // Insert links (some valid, some broken)
    for i in 0..num_links {
        let src = i % num_notes;
        let dst = (i + 1) % num_notes;

        if i % 10 == 0 {
            // 10% broken links (unresolved)
            conn.execute(
                "INSERT INTO links (src_note_id, dst_note_id, dst_text, kind, is_embed) VALUES (?1, NULL, ?2, 'wikilink', 0)",
                rusqlite::params![src + 1, format!("nonexistent_{}.md", i)],
            )
            .unwrap();
        } else {
            // Valid links
            conn.execute(
                "INSERT INTO links (src_note_id, dst_note_id, dst_text, kind, is_embed) VALUES (?1, ?2, ?3, 'wikilink', 0)",
                rusqlite::params![src + 1, dst + 1, format!("note_{}.md", dst)],
            )
            .unwrap();
        }
    }

    // Create some orphans (notes with no links)
    let orphan_count = num_notes / 20; // 5% orphans
    for i in 0..orphan_count {
        conn.execute(
            "INSERT INTO notes (path, title, mtime, hash) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![
                format!("orphan_{}.md", i),
                format!("Orphan {}", i),
                i as i64,
                format!("orphan_hash_{}", i)
            ],
        )
        .unwrap();
    }

    // Create some dead-ends (notes with incoming but no outgoing)
    let dead_end_count = num_notes / 10; // 10% dead-ends
    for i in 0..dead_end_count {
        let dst = num_notes + orphan_count + i;
        conn.execute(
            "INSERT INTO notes (path, title, mtime, hash) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![
                format!("dead_end_{}.md", i),
                format!("Dead End {}", i),
                i as i64,
                format!("dead_hash_{}", i)
            ],
        )
        .unwrap();
        // Link to this dead-end
        conn.execute(
            "INSERT INTO links (src_note_id, dst_note_id, dst_text, kind, is_embed) VALUES (1, ?1, ?2, 'wikilink', 0)",
            rusqlite::params![dst + 1, format!("dead_end_{}.md", i)],
        )
        .unwrap();
    }
}

fn bench_get_orphans(c: &mut Criterion) {
    let mut group = c.benchmark_group("diagnose_orphans");

    // Small vault: 100 notes, 500 links
    group.bench_function("small_vault_100_notes", |b| {
        b.iter(|| {
            let conn = Connection::open_in_memory().unwrap();
            setup_large_db(&conn, 100, 500);
            get_orphans(&conn, false, false).unwrap()
        });
    });

    // Medium vault: 500 notes, 2500 links
    group.bench_function("medium_vault_500_notes", |b| {
        b.iter(|| {
            let conn = Connection::open_in_memory().unwrap();
            setup_large_db(&conn, 500, 2500);
            get_orphans(&conn, false, false).unwrap()
        });
    });

    // Large vault: 1000 notes, 5000 links
    group.bench_function("large_vault_1000_notes", |b| {
        b.iter(|| {
            let conn = Connection::open_in_memory().unwrap();
            setup_large_db(&conn, 1000, 5000);
            get_orphans(&conn, false, false).unwrap()
        });
    });

    // With exclusions
    group.bench_function("with_exclusions", |b| {
        b.iter(|| {
            let conn = Connection::open_in_memory().unwrap();
            setup_large_db(&conn, 500, 2500);
            get_orphans(&conn, true, true).unwrap()
        });
    });

    group.finish();
}

fn bench_get_dead_ends(c: &mut Criterion) {
    let mut group = c.benchmark_group("diagnose_dead_ends");

    // Small vault
    group.bench_function("small_vault_100_notes", |b| {
        b.iter(|| {
            let conn = Connection::open_in_memory().unwrap();
            setup_large_db(&conn, 100, 500);
            get_dead_ends(&conn, false, false).unwrap()
        });
    });

    // Medium vault
    group.bench_function("medium_vault_500_notes", |b| {
        b.iter(|| {
            let conn = Connection::open_in_memory().unwrap();
            setup_large_db(&conn, 500, 2500);
            get_dead_ends(&conn, false, false).unwrap()
        });
    });

    // Large vault
    group.bench_function("large_vault_1000_notes", |b| {
        b.iter(|| {
            let conn = Connection::open_in_memory().unwrap();
            setup_large_db(&conn, 1000, 5000);
            get_dead_ends(&conn, false, false).unwrap()
        });
    });

    group.finish();
}

fn bench_diagnose_broken_links(c: &mut Criterion) {
    let mut group = c.benchmark_group("diagnose_broken_links");

    // Small vault
    group.bench_function("small_vault_100_notes", |b| {
        b.iter(|| {
            let conn = Connection::open_in_memory().unwrap();
            setup_large_db(&conn, 100, 500);
            diagnose_broken_links(&conn).unwrap()
        });
    });

    // Medium vault
    group.bench_function("medium_vault_500_notes", |b| {
        b.iter(|| {
            let conn = Connection::open_in_memory().unwrap();
            setup_large_db(&conn, 500, 2500);
            diagnose_broken_links(&conn).unwrap()
        });
    });

    // Large vault
    group.bench_function("large_vault_1000_notes", |b| {
        b.iter(|| {
            let conn = Connection::open_in_memory().unwrap();
            setup_large_db(&conn, 1000, 5000);
            diagnose_broken_links(&conn).unwrap()
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_get_orphans,
    bench_get_dead_ends,
    bench_diagnose_broken_links
);
criterion_main!(benches);
