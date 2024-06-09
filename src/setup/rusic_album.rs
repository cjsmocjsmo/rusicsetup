// SPDX-FileCopyrightText: 2024 Charlie J Smotherman <porthose.cjsmo.cjsmo@gmail.com
//
// SPDX-License-Identifier: GPL-3.0-or-later

use rusqlite::Connection;
use serde_json;
use std::env;
use crate::types;

pub fn unique_albumids() -> Vec<String> {
    let db_path = env::var("RUSIC_DB_PATH").expect("RUSIC_DB_PATH not set");
    let conn = Connection::open(db_path).expect("unable to open db file");
    let mut stmt = conn.prepare("SELECT DISTINCT albumid FROM music;").unwrap();
    let rows = stmt.query_map([], |row| row.get(0)).unwrap();
    let mut albumids: Vec<String> = Vec::new();
    for row in rows {
        albumids.push(row.unwrap());
    }

    // log::info!("albumids: {:?}", albumids.len());

    albumids
}

pub fn songids_for_albumid(xlist: Vec<String>) -> Vec<types::AlbumSongs> {
    let db_path = env::var("RUSIC_DB_PATH").expect("RUSIC_DB_PATH not set");
    let pagination_str = env::var("RUSIC_PAGINATION").expect("RUSIC_PAGINATION not set");
    let pagination: i32 = pagination_str.parse().unwrap();
    let mut idx = 1;
    let mut pge = 1;
    let mut albums_songs_vec = Vec::new();

    for x in xlist {
        idx += 1;
        if idx == pagination {
            pge += 1;
            idx = 1;
        }
        let conn = Connection::open(db_path.clone()).expect("unable to open db file");
        let mut stmt = conn
            .prepare("SELECT rusicid FROM music WHERE albumid = ?1")
            .unwrap();
        let mut rows = stmt.query(&[&x]).expect("Unable to query db");

        let mut songids: Vec<String> = Vec::new();
        while let Some(row) = rows.next().unwrap() {
            songids.push(row.get(0).unwrap());
        }
        let vstring = serde_json::to_string(&songids).unwrap();
        let albumsongs = types::AlbumSongs {
            page: pge,
            albumid: x,
            rusicids: vstring,
        };
        albums_songs_vec.push(albumsongs);
    }

    albums_songs_vec
}
