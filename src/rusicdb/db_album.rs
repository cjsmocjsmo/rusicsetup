use crate::rusicdb::db_main::open_conn;
use crate::types;
use rusqlite::Result;

pub fn upsert_album(album: &types::Album) -> Result<()> {
    let conn = open_conn()?;

    conn.execute(
        "INSERT INTO albums (
                albumid,
                artistid,
                name,
                first_letter
            )
            VALUES (?1, ?2, ?3, ?4)
            ON CONFLICT(albumid) DO NOTHING",
        (&album.albumid, &album.artistid, &album.name, &album.first_letter),
    )?;

    Ok(())
}
