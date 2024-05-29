use std::env;
use crate::types;
use rusqlite::Connection;

pub fn get_mylikes_oldsongs() -> (String, String) {
    let db_path = env::var("RUSIC_DB_PATH").expect("RUSIC_DB_PATH not set");
    let conn = Connection::open(db_path.clone()).expect("unable to open db file");
    let mylikes = "mylikes".to_string();
    let mut stmt = conn
        .prepare("SELECT * FROM playlists WHERE name = ?1")
        .unwrap();
    let mut rows = stmt.query(&[&mylikes]).expect("Unable to query db");

    let mut oldsongs = String::new();
    let mut oldnumsongs = String::new();
    while let Some(row) = rows.next().unwrap() {
        let oldplinfo = types::PlayList {
            rusicid: row.get(1).unwrap(),
            name: row.get(2).unwrap(),
            songs: row.get(3).unwrap(),
            numsongs: row.get(4).unwrap(),
        };
        oldsongs = oldplinfo.songs;
        oldnumsongs = oldplinfo.numsongs;
    }

    (oldsongs, oldnumsongs)
}

pub fn update_mylikes(songs: String, numsongs: String, name: String) -> bool {
    let db_path = env::var("RUSIC_DB_PATH").expect("RUSIC_DB_PATH not set");
    let conn = Connection::open(db_path.clone()).expect("unable to open db file");

    let mut stmt = conn
        .prepare("UPDATE playlists SET songs = ?1, numsongs = ?2 WHERE name = ?3")
        .unwrap();
    let _rows = stmt
        .execute(&[&songs, &numsongs, &name])
        .expect("Unable to query db");

    true
}