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

pub fn post_song_to_db(song: &types::Song) -> Result<usize> {
    let conn = open_conn()?;

    conn.execute(
        "INSERT OR IGNORE INTO songs (
                rusicid,
                albumid,
                title,
                imgurl,
                playpath,
                fullpath,
                extension,
                idx,
                page,
                fsizeresults,
                first_letter
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        (
            &song.rusicid,
            &song.albumid,
            &song.title,
            &song.imgurl,
            &song.playpath,
            &song.fullpath,
            &song.extension,
            &song.idx,
            &song.page,
            &song.fsizeresults,
            &song.first_letter,
        ),
    )
}

pub fn post_album_image_to_db(img: &types::AlbumImage) -> Result<usize> {
    let conn = open_conn()?;

    conn.execute(
        "INSERT OR IGNORE INTO album_images (
                albumid,
                width,
                height,
                filesize,
                fullpath,
                thumbpath,
                idx,
                page,
                httpthumbpath
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        (
            &img.albumid,
            &img.width,
            &img.height,
            &img.filesize,
            &img.fullpath,
            &img.thumbpath,
            &img.idx,
            &img.page,
            &img.httpthumbpath,
        ),
    )
}

