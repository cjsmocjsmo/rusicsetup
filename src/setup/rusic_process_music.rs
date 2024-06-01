use crate::rusicdb::db_album;
use crate::rusicdb::db_artist;
use crate::rusicdb::db_main;
use crate::setup::rusic_utils;
use crate::setup::rusic_utils::RusicUtils;
use crate::types;
use std::clone::Clone;
use std::env;

pub fn process_mp3s(x: String, index: String, page: String) -> types::MusicInfo {
    let fu = RusicUtils { apath: x.clone() };
    let rusic_id = rusic_utils::get_md5(x.clone());
    let art_alb = RusicUtils::split_artist_album(&fu);
    let tag = RusicUtils::get_tag_info(&fu).unwrap();
    let tag_artist = tag.0.clone();
    let tag_album = tag.1.clone();
    let artist_id = rusic_utils::get_md5(tag.0.clone());
    let album_id = rusic_utils::get_md5(tag.1.clone());
    let img_url = create_thumb_path(art_alb.0.clone(), art_alb.1.clone());
    let play_path = RusicUtils::create_mp3_play_path(&fu);

    let music_info = types::MusicInfo {
        rusicid: rusic_id.clone(),
        imgurl: img_url.clone(),
        playpath: play_path.clone(),
        artist: tag_artist.clone(),
        artistid: artist_id.clone(),
        album: tag_album.clone(),
        albumid: album_id.clone(),
        song: tag.2.clone(), // Extract the value from the Result
        fullpath: x.clone(),
        extension: RusicUtils::split_ext(&fu),
        idx: index.clone(),
        page: page.clone(),
        fsizeresults: RusicUtils::get_file_size(&fu).to_string(),
    };
    let _wm = db_main::post_music_to_db(music_info.clone()).unwrap();
    let _wnfo = write_music_nfos_to_file(music_info.clone(), index.clone());

    let _insert_first_letter = rusic_utils::gen_first_letter_db(x.clone()).unwrap();
    let _insert_art_artid =
        db_artist::write_art_artid_to_db(rusic_id.clone(), tag_artist.clone(), artist_id.clone())
            .unwrap();
    let _insert_alb_albid =
        db_album::write_alb_albid_to_db(rusic_id.clone(), img_url.clone(), album_id.clone())
            .unwrap();

    music_info.clone()
}

fn write_music_nfos_to_file(mfo: types::MusicInfo, index: String) {
    let mus_info = serde_json::to_string(&mfo).unwrap();
    let rusic_music_metadata_path = env::var("RUSIC_NFOS").expect("$RUSIC_NFOS is not set");
    let a = format!("{}/", rusic_music_metadata_path.as_str());
    let b = format!("Music_Meta_{}.json", index.to_string());
    let outpath = a + &b;
    std::fs::write(outpath, mus_info).unwrap();
}

fn create_thumb_path(art: String, alb: String) -> String {
    let myhttpd = env::var("RUSIC_HTTP_ADDR").expect("$RUSIC_HTTP_ADDR is not set");
    let myport = env::var("RUSIC_PORT").expect("$RUSIC_PORT is not set");
    let npath = myhttpd + &myport + "/thumbnails" + &art + "_-_" + &alb + ".jpg";
    let newpath = npath.replace(" ", "_");
    newpath
}
