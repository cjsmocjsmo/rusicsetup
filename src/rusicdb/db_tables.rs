use crate::rusicdb::db_main::open_conn;
use rusqlite::Result;

pub fn create_tables() {
    let _cat = create_artists_table().expect("Unable to create artists table");
    let _calt = create_albums_table().expect("Unable to create albums table");
    let _cst = create_songs_table().expect("Unable to create songs table");
    let _cait = create_album_images_table().expect("Unable to create album_images table");
    let _cpl = create_playlists_table().expect("Unable to create playlists table");
    let _cpls = create_playlist_songs_table().expect("Unable to create playlist_songs table");
    let _cstats = create_stats_table().expect("Unable to create stats table");
}

pub fn create_artists_table() -> Result<()> {
    let conn = open_conn()?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS artists (
            artistid TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            first_letter TEXT NOT NULL
        )",
        (),
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_artists_first_letter ON artists(first_letter)",
        (),
    )?;

    Ok(())
}

pub fn create_albums_table() -> Result<()> {
    let conn = open_conn()?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS albums (
            albumid TEXT PRIMARY KEY,
            artistid TEXT NOT NULL REFERENCES artists(artistid) ON DELETE CASCADE,
            name TEXT NOT NULL,
            first_letter TEXT NOT NULL
        )",
        (),
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_albums_artistid ON albums(artistid)",
        (),
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_albums_first_letter ON albums(first_letter)",
        (),
    )?;

    Ok(())
}

pub fn create_songs_table() -> Result<()> {
    let conn = open_conn()?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS songs (
            rusicid TEXT PRIMARY KEY,
            albumid TEXT NOT NULL REFERENCES albums(albumid) ON DELETE CASCADE,
            title TEXT NOT NULL,
            imgurl TEXT NOT NULL,
            playpath TEXT NOT NULL,
            fullpath TEXT NOT NULL,
            extension TEXT NOT NULL,
            idx TEXT NOT NULL,
            page TEXT NOT NULL,
            fsizeresults TEXT NOT NULL,
            first_letter TEXT NOT NULL
        )",
        (),
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_songs_albumid ON songs(albumid)",
        (),
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_songs_first_letter ON songs(first_letter)",
        (),
    )?;

    Ok(())
}

pub fn create_album_images_table() -> Result<()> {
    let conn = open_conn()?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS album_images (
            id INTEGER PRIMARY KEY,
            albumid TEXT NOT NULL REFERENCES albums(albumid) ON DELETE CASCADE,
            width TEXT NOT NULL,
            height TEXT NOT NULL,
            filesize TEXT NOT NULL,
            fullpath TEXT NOT NULL UNIQUE,
            thumbpath TEXT NOT NULL,
            idx TEXT NOT NULL,
            page TEXT NOT NULL,
            httpthumbpath TEXT NOT NULL
        )",
        (),
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_album_images_albumid ON album_images(albumid)",
        (),
    )?;

    Ok(())
}

pub fn create_playlists_table() -> Result<()> {
    let conn = open_conn()?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS playlists (
            id INTEGER PRIMARY KEY,
            rusicid TEXT NOT NULL UNIQUE,
            name TEXT NOT NULL UNIQUE
        )",
        (),
    )?;

    Ok(())
}

pub fn create_playlist_songs_table() -> Result<()> {
    let conn = open_conn()?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS playlist_songs (
            playlist_id INTEGER NOT NULL REFERENCES playlists(id) ON DELETE CASCADE,
            song_rusicid TEXT NOT NULL REFERENCES songs(rusicid) ON DELETE CASCADE,
            position INTEGER NOT NULL,
            PRIMARY KEY (playlist_id, song_rusicid)
        )",
        (),
    )?;

    Ok(())
}

pub fn create_stats_table() -> Result<()> {
    let conn = open_conn()?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS stats (
            id INTEGER PRIMARY KEY,
            artistcount INTEGER NOT NULL,
            albumcount INTEGER NOT NULL,
            songcount INTEGER NOT NULL,
            imagecount INTEGER NOT NULL
        )",
        (),
    )?;

    Ok(())
}

