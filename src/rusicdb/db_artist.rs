use crate::rusicdb::db_main::open_conn;
use crate::types;
use rusqlite::Result;

pub fn upsert_artist(artist: &types::Artist) -> Result<()> {
    let conn = open_conn()?;

    conn.execute(
        "INSERT INTO artists (
                artistid,
                name,
                first_letter
            )
            VALUES (?1, ?2, ?3)
            ON CONFLICT(artistid) DO NOTHING",
        (&artist.artistid, &artist.name, &artist.first_letter),
    )?;

    Ok(())
}
