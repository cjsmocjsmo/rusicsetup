
use rusqlite::{Connection, Result};
use std::env;

pub fn write_alb_albid_to_db(rusid: String, imageurl: String, albid: String) -> Result<()> {
    let db_path = env::var("RUSIC_DB_PATH").expect("RUSIC_DB_PATH not set");
    let conn = Connection::open(db_path).unwrap();

    conn.execute(
        "INSERT INTO albalbid (
                rusicid,
                imageurl,
                albumid
            )
            VALUES (?1, ?2, ?3)",
        (
            &rusid,
            &imageurl,
            &albid,
        ),
    )?;

    Ok(())
}