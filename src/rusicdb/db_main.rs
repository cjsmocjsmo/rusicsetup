use crate::types;
use rusqlite::{Connection, Result};
use std::env;

pub fn open_conn() -> Result<Connection> {
    let db_path = env::var("RUSIC_DB_PATH").expect("RUSIC_DB_PATH not set");
    let conn = Connection::open(db_path)?;
    conn.execute_batch("PRAGMA foreign_keys = ON;")?;
    Ok(conn)
}

pub fn post_playlist_to_db(pl: types::PlayList) -> Result<()> {
    let conn = open_conn()?;

    conn.execute(
        "INSERT INTO playlists (
                rusicid,
                name
            )
            VALUES (?1, ?2)",
        (&pl.rusicid, &pl.name),
    )?;

    Ok(())
}

pub fn post_stats_to_db(stats: types::Stats) -> Result<()> {
    let conn = open_conn()?;

    conn.execute(
        "INSERT INTO stats (
                artistcount,
                albumcount,
                songcount,
                imagecount
            )
            VALUES (?1, ?2, ?3, ?4)",
        (
            &stats.artistcount,
            &stats.albumcount,
            &stats.songcount,
            &stats.imagecount,
        ),
    )?;

    Ok(())
}

pub fn compute_stats(imagecount: i64) -> Result<types::Stats> {
    let conn = open_conn()?;
    let artistcount: i64 = conn.query_row("SELECT COUNT(*) FROM artists", (), |row| row.get(0))?;
    let albumcount: i64 = conn.query_row("SELECT COUNT(*) FROM albums", (), |row| row.get(0))?;
    let songcount: i64 = conn.query_row("SELECT COUNT(*) FROM songs", (), |row| row.get(0))?;

    Ok(types::Stats {
        artistcount,
        albumcount,
        songcount,
        imagecount,
    })
}


