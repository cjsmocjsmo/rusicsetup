use rusqlite::{Connection, Result};
use std::env;

pub fn create_tables() {
    let _csfat = create_songs_for_album_table().expect("Unable to create songs for album table");
    let _cafat =
        create_albums_for_artist_table().expect("Unable to create albums for artist table");
    let _cmit = create_music_images_table().expect("Unable to create music images table");
    let _cmt = create_music_table().expect("Unable to create music table");
    let _caswt = create_startswith_table().expect("Unable to create artiststartswith table");
    let _caait = create_artartid_table().expect("Unable to create artartid table");
    let _caalbit = create_albalbid_table().expect("Unable to create albalbid table");
    let _caartc = create_artist_count().expect("Unable to create artist count table");
    let _caalbc = create_album_count().expect("Unable to create album count table");
    let _casongc = create_song_count().expect("Unable to create song count table");
    let _cpl = create_playlist().expect("Unable to create playlist table");
    let _cstats = create_stats().expect("Unable to create stats table");
}

pub fn create_songs_for_album_table() -> Result<()> {
    let db_path = env::var("RUSIC_DB_PATH").expect("RUSIC_DB_PATH not set");
    let conn = Connection::open(db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS songsforalbum (
            id INTEGER PRIMARY KEY,
            page INTEGER NOT NULL,
            albumid TEXT NOT NULL,
            songs TEXT NOT NULL
        )",
        (),
    )?;

    Ok(())
}

pub fn create_albums_for_artist_table() -> Result<()> {
    let db_path = env::var("RUSIC_DB_PATH").expect("RUSIC_DB_PATH not set");
    let conn = Connection::open(db_path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS albumsforartist (
            id INTEGER PRIMARY KEY,
            page INTEGER NOT NULL,
            artistid TEXT NOT NULL,
            albums TEXT NOT NULL
        )",
        (),
    )?;

    Ok(())
}

pub fn create_music_images_table() -> Result<()> {
    let db_path = env::var("RUSIC_DB_PATH").expect("RUSIC_DB_PATH not set");
    let conn = Connection::open(db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS music_images (
            id INTEGER PRIMARY KEY,
            rusicid TEXT NOT NULL UNIQUE,
            width TEXT NOT NULL,
            height TEXT NOT NULL,
            artist TEXT NOT NULL,
            artistid TEXT NOT NULL,
            album TEXT NOT NULL,
            albumid TEXT NOT NULL,
            filesize TEXT NOT NULL,
            fullpath TEXT NOT NULL,
            thumbpath TEXT NOT NULL,
            idx TEXT NOT NULL,
            page TEXT NOT NULL,
            httpthumbpath TEXT NOT NULL
        )",
        (),
    )?;

    Ok(())
}

pub fn create_music_table() -> Result<()> {
    let db_path = env::var("RUSIC_DB_PATH").expect("RUSIC_DB_PATH not set");
    let conn = Connection::open(db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS music (
            id INTEGER PRIMARY KEY,
            rusicid TEXT NOT NULL UNIQUE,
            imgurl TEXT NOT NULL,
            playpath TEXT NOT NULL,
            artist TEXT NOT NULL,
            artistid TEXT NOT NULL,
            album TEXT NOT NULL,
            albumid TEXT NOT NULL,
            song TEXT NOT NULL,
            fullpath TEXT NOT NULL,
            extension TEXT NOT NULL,
            idx TEXT NOT NULL,
            page TEXT NOT NULL,
            fsizeresults TEXT NOT NULL
        )",
        (),
    )?;

    Ok(())
}

pub fn create_startswith_table() -> Result<()> {
    let db_path = env::var("RUSIC_DB_PATH").expect("RUSIC_DB_PATH not set");
    let conn = Connection::open(db_path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS startswith (
        id INTEGER PRIMARY KEY,
        rusicid TEXT NOT NULL UNIQUE,
        artist TEXT NOT NULL,
        artistid TEXT NOT NULL,
        album TEXT NOT NULL,
        albumid TEXT NOT NULL,
        song TEXT NOT NULL,
        artist_first_letter TEXT NOT NULL,
        album_first_letter TEXT NOT NULL,
        song_first_letter TEXT NOT NULL
    )",
        (),
    )?;

    Ok(())
}

pub fn create_artartid_table() -> Result<()> {
    let db_path = env::var("RUSIC_DB_PATH").expect("RUSIC_DB_PATH not set");
    let conn = Connection::open(db_path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS artartid (
        id INTEGER PRIMARY KEY,
        rusicid TEXT NOT NULL UNIQUE,
        artist TEXT NOT NULL,
        artistid TEXT NOT NULL
    )",
        (),
    )?;

    Ok(())
}

pub fn create_albalbid_table() -> Result<()> {
    let db_path = env::var("RUSIC_DB_PATH").expect("RUSIC_DB_PATH not set");
    let conn = Connection::open(db_path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS albalbid (
        id INTEGER PRIMARY KEY,
        rusicid TEXT NOT NULL UNIQUE,
        imageurl TEXT NOT NULL,
        albumid TEXT NOT NULL
    )",
        (),
    )?;

    Ok(())
}

pub fn create_artist_count() -> Result<()> {
    let db_path = env::var("RUSIC_DB_PATH").expect("RUSIC_DB_PATH not set");
    let conn = Connection::open(db_path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS artistcount (
        id INTEGER PRIMARY KEY,
        alpha TEXT NOT NULL,
        count INTEGER NOT NULL
    )",
        (),
    )?;

    Ok(())
}

pub fn create_album_count() -> Result<()> {
    let db_path = env::var("RUSIC_DB_PATH").expect("RUSIC_DB_PATH not set");
    let conn = Connection::open(db_path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS albumcount (
        id INTEGER PRIMARY KEY,
        alpha TEXT NOT NULL,
        count INTEGER NOT NULL
    )",
        (),
    )?;

    Ok(())
}

pub fn create_song_count() -> Result<()> {
    let db_path = env::var("RUSIC_DB_PATH").expect("RUSIC_DB_PATH not set");
    let conn = Connection::open(db_path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS songcount (
        id INTEGER PRIMARY KEY,
        alpha TEXT NOT NULL,
        count INTEGER NOT NULL
    )",
        (),
    )?;

    Ok(())
}

pub fn create_playlist() -> Result<()> {
    let db_path = env::var("RUSIC_DB_PATH").expect("RUSIC_DB_PATH not set");
    let conn = Connection::open(db_path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS playlists (
        id INTEGER PRIMARY KEY,
        rusicid TEXT NOT NULL UNIQUE,
        name TEXT NOT NULL UNIQUE,
        songs TEXT NOT NULL,
        numsongs TEXT NOT NULL
    )",
        (),
    )?;

    Ok(())
}

pub fn create_stats() -> Result<()> {
    let db_path = env::var("RUSIC_DB_PATH").expect("RUSIC_DB_PATH not set");
    let conn = Connection::open(db_path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS stats (
        id INTEGER PRIMARY KEY,
        artistcount TEXT NOT NULL,
        albumcount TEXT NOT NULL,
        songcount TEXT NOT NULL,
        imagecount TEXT NOT NULL
    )",
        (),
    )?;

    Ok(())
}

// conn.execute(
//     "CREATE TABLE IF NOT EXISTS artistids (
//         id INTEGER PRIMARY KEY,
//         artist TEXT NOT NULL,
//         artistid TEXT NOT NULL,
//         path TEXT NOT NULL
//     )",
//     (),
// )?;

// conn.execute(
//     "CREATE TABLE IF NOT EXISTS albumids (
//         id INTEGER PRIMARY KEY,
//         album TEXT NOT NULL,
//         albumid TEXT NOT NULL,
//         path TEXT NOT NULL
//     )",
//     (),
// )?;
