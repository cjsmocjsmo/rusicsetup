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
) -> Option<(types::MusicInfo, types::FirstLetterInfo)> {
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

    let music_info = types::MusicInfo {
        rusicid: rusic_id.clone(),
        imgurl: img_url.clone(),
        playpath: play_path.clone(),
        artist: tag_artist.clone(),
        artistid: artist_id.clone(),
        album: tag_album.clone(),
        albumid: album_id.clone(),
        song: tag_song.clone(),
        fullpath: x.clone(),
        extension: RusicUtils::split_ext(&fu),
        idx: index.clone(),
        page: page.clone(),
        fsizeresults: RusicUtils::get_file_size(&fu).to_string(),
    };

    let first_letter_info = types::FirstLetterInfo {
        rusicid: rusic_id,
        artist: tag_artist.clone(),
        album: tag_album.clone(),
        artistid: artist_id,
        albumid: album_id,
        song: tag_song.clone(),
        artist_first_letter: tag_artist.chars().next().unwrap_or('_').to_string(),
        album_first_letter: tag_album.chars().next().unwrap_or('_').to_string(),
        song_first_letter: tag_song.chars().next().unwrap_or('_').to_string(),
    };

    Some((music_info, first_letter_info))
}

// fn write_music_nfos_to_file(mfo: types::MusicInfo, index: String) {
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
