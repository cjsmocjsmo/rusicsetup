// SPDX-FileCopyrightText: 2024 Charlie J Smotherman <porthose.cjsmo.cjsmo@gmail.com
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::setup::rusic_utils;
use crate::setup::rusic_utils::RusicUtils;
use crate::types;
use std::clone::Clone;
use std::env;

pub fn process_audio_file(
    x: String,
    index: String,
    page: String,
) -> Option<(types::Artist, types::Album, types::Song)> {
    let fu = RusicUtils { apath: x.clone() };
    let rusic_id = rusic_utils::get_md5(x.clone());
    let art_alb = RusicUtils::split_artist_album(&fu);
    let tag = match RusicUtils::get_tag_info(&fu) {
        Ok(tag) => tag,
        Err(err) => {
            eprintln!("Skipping audio file {}: {}", x, err);
            return None;
        }
    };
    let tag_artist = tag.0.clone();
    let tag_album = tag.1.clone();
    let tag_song = tag.2.clone();
    let artist_id = rusic_utils::get_md5(tag.0.clone());
    let album_id = rusic_utils::get_md5(tag.1.clone());
    let img_url = create_thumb_path(art_alb.0.clone(), art_alb.1.clone());
    let play_path = RusicUtils::create_audio_play_path(&fu);

    let artist = types::Artist {
        artistid: artist_id.clone(),
        name: tag_artist.clone(),
        first_letter: tag_artist.chars().next().unwrap_or('_').to_string(),
    };

    let album = types::Album {
        albumid: album_id.clone(),
        artistid: artist_id,
        name: tag_album.clone(),
        first_letter: tag_album.chars().next().unwrap_or('_').to_string(),
    };

    let song = types::Song {
        rusicid: rusic_id,
        albumid: album_id,
        title: tag_song.clone(),
        imgurl: img_url,
        playpath: play_path,
        fullpath: x.clone(),
        extension: RusicUtils::split_ext(&fu),
        idx: index,
        page,
        fsizeresults: RusicUtils::get_file_size(&fu).to_string(),
        first_letter: tag_song.chars().next().unwrap_or('_').to_string(),
    };

    Some((artist, album, song))
}

// fn write_music_nfos_to_file(mfo: types::Song, index: String) {
//     let mus_info = serde_json::to_string(&mfo).unwrap();
//     let rusic_music_metadata_path = env::var("RUSIC_NFOS").expect("$RUSIC_NFOS is not set");
//     let a = format!("{}/", rusic_music_metadata_path.as_str());
//     let b = format!("Music_Meta_{}.json", index.to_string());
//     let outpath = a + &b;
//     std::fs::write(outpath, mus_info).unwrap();
// }

fn create_thumb_path(art: String, alb: String) -> String {
    let myhttpd = env::var("RUSIC_HTTP_ADDR").expect("$RUSIC_HTTP_ADDR is not set");
    let myport = env::var("RUSIC_PORT").expect("$RUSIC_PORT is not set");
    let npath = myhttpd + &myport + "/thumbs" + &art + "_-_" + &alb + ".jpg";
    let newpath = npath.replace(" ", "_");
    newpath
}
