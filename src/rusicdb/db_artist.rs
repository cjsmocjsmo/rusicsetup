use rusqlite::{Connection, Result};
use std::env;


pub fn write_art_artid_to_db(rusid: String, art: String, artid: String) -> Result<()> {
    let db_path = env::var("RUSIC_DB_PATH").expect("RUSIC_DB_PATH not set");
    let conn = Connection::open(db_path).unwrap();

    conn.execute(
        "INSERT INTO artartid (
                rusicid,
                artist,
                artistid
            )
            VALUES (?1, ?2, ?3)",
        (
            &rusid,
            &art,
            &artid,
        ),
    )?;

    Ok(())
}