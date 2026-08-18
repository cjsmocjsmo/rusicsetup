// SPDX-FileCopyrightText: 2024 Charlie J Smotherman <porthose.cjsmo.cjsmo@gmail.com
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::setup::rusic_utils;
use crate::setup::rusic_utils::RusicUtils;
// use rusqlite::{Connection, Result};
use crate::types;
use std::clone::Clone;
use std::env;

// only .jpg coverart is expected on upload; no format conversion is needed here.
pub fn process_music_images(x: String, index: i32, pageg: i32) -> Option<types::AlbumImage> {
    let media = x.clone();

    let foo2 = RusicUtils {
        apath: media.clone(),
    };
    let dims = RusicUtils::get_dims(&foo2);
    let artalb = RusicUtils::split_artist_album(&foo2);
    let artist1 = artalb.0;
    let album1 = artalb.1;

    if dims == (0, 0) {
        eprintln!("Skipping coverart with unreadable dimensions: {}", media);
        return None;
    }

    let newdims = crate::setup::rusic_utils::normalize_music_image(dims);
    let width_r = newdims.0.to_string();
    let height_r = newdims.1.to_string();
    let fsize_results = RusicUtils::get_file_size(&foo2).to_string();
    let full_path = media.clone();
    let Some(tpath) = create_music_thumbnail(&media, artist1.clone(), album1.clone()) else {
        return None;
    };
    let (thumb_path, http_thumb_path) = tpath;

    let album_image = types::AlbumImage {
        albumid: rusic_utils::get_md5(album1.clone()),
        width: width_r,
        height: height_r,
        filesize: fsize_results,
        fullpath: full_path,
        thumbpath: thumb_path,
        idx: index.to_string(),
        page: pageg.to_string(),
        httpthumbpath: http_thumb_path,
    };
    Some(album_image)
}

fn create_music_thumbnail(x: &String, art: String, alb: String) -> Option<(String, String)> {
    let rusic_music_metadata_path = env::var("RUSIC_THUMBS").expect("$RUSIC_THUMBS is not set");
    // thumbs dir may not exist yet on a fresh install; create it so save() below doesn't silently fail
    if let Err(err) = std::fs::create_dir_all(&rusic_music_metadata_path) {
        eprintln!(
            "Unable to create thumbs directory {}: {}",
            rusic_music_metadata_path, err
        );
        return None;
    }
    let new_fname = "/".to_string() + art.as_str() + "_-_" + alb.as_str() + ".jpg";
    let ofname = rusic_music_metadata_path + &new_fname;
    let out_fname = ofname.replace(" ", "_");
    let server_addr = env::var("RUSIC_HTTP_ADDR").expect("$RUSIC_SERVER_ADDR is not set");
    let server_port = env::var("RUSIC_PORT").expect("$RUSIC_SERVER_PORT is not set");
    let http_path_1 = server_addr + &server_port + "/thumbs" + &new_fname;
    let http_path = http_path_1.replace(" ", "_");

    let img = match image::open(x) {
        Ok(img) => img,
        Err(err) => {
            eprintln!("Unable to open coverart image {}: {}", x, err);
            return None;
        }
    };
    let thumbnail = img.resize(200, 200, image::imageops::FilterType::Lanczos3);
    if let Err(err) = thumbnail.save(&out_fname) {
        eprintln!("Unable to save thumbnail {}: {}", out_fname, err);
        return None;
    }

    Some((out_fname, http_path))
}

// fn write_music_img_to_file(miinfo: types::MusicImageInfo, index: i32) {
//     let mii = serde_json::to_string(&miinfo).unwrap();
//     let rusic_music_metadata_path = env::var("RUSIC_NFOS").expect("$RUSIC_NFOS is not set");
//     let outpath = format!(
//         "{}/Music_Image_Meta_{}.json",
//         rusic_music_metadata_path.as_str(),
//         &index
//     );
//     std::fs::write(outpath, mii.clone()).unwrap();
// }

