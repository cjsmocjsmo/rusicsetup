use crate::rusicdb::db_main::open_conn;
use rusqlite::Result;

pub fn get_mylikes_songs() -> Vec<String> {
    let conn = open_conn().expect("unable to open db file");
    let mut stmt = conn
        .prepare(
            "SELECT ps.song_rusicid
               FROM playlist_songs ps
               JOIN playlists p ON p.id = ps.playlist_id
              WHERE p.name = 'mylikes'
              ORDER BY ps.position",
        )
        .unwrap();
    let rows = stmt.query_map((), |row| row.get(0)).unwrap();

    rows.filter_map(std::result::Result::ok).collect()
}

pub fn set_mylikes_songs(rusicid: String, songs: Vec<String>) -> Result<()> {
    let conn = open_conn()?;

    conn.execute(
        "INSERT INTO playlists (rusicid, name) VALUES (?1, 'mylikes')
         ON CONFLICT(name) DO NOTHING",
        (&rusicid,),
    )?;

    let playlist_id: i64 = conn.query_row(
        "SELECT id FROM playlists WHERE name = 'mylikes'",
        (),
        |row| row.get(0),
    )?;

    let tx = conn.unchecked_transaction()?;
    tx.execute(
        "DELETE FROM playlist_songs WHERE playlist_id = ?1",
        (&playlist_id,),
    )?;
    for (position, song_rusicid) in songs.iter().enumerate() {
        tx.execute(
            "INSERT INTO playlist_songs (playlist_id, song_rusicid, position) VALUES (?1, ?2, ?3)",
            (&playlist_id, song_rusicid, &(position as i64)),
        )?;
    }
    tx.commit()
}

