// SPDX-FileCopyrightText: 2024 Charlie J Smotherman <porthose.cjsmo.cjsmo@gmail.com
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::env;
use std::sync::mpsc::channel;
use threadpool::ThreadPool;
use crate::rusicdb;
// use crate::server::fragments;
use crate::types;
use std::path::Path;

pub mod rusic_album;
pub mod rusic_artist;
pub mod rusic_process_music;
pub mod rusic_process_music_images;
pub mod rusic_utils;
pub mod rusic_walk_dirs;

pub fn setup() -> String {
    let _create_tables = rusicdb::db_tables::create_tables();

    let usb_drive_count = rusic_walk_dirs::scan_for_usb_devices();
    let mut usb_drives: Vec<String> = Vec::new();
    let mut media_lists = (Vec::new(), Vec::new());
    if usb_drive_count > 1 {
        usb_drives = rusic_walk_dirs::scan_usb_devices();
        media_lists = rusic_walk_dirs::walk_usb_drives(usb_drives.clone());
    }

    
    let media_lists2= rusic_walk_dirs::walk_home_dir();
    media_lists.0.extend(media_lists2.0);
    media_lists.1.extend(media_lists2.1);

    let mp3_count = media_lists.0.clone().len();
    let mut dirlist = Vec::new();
    for media in media_lists.0.iter() {
        let path = Path::new(media);
        let dir = path.parent().unwrap_or(Path::new("."));
        if !dirlist.contains(&dir) {
            dirlist.push(dir);
        }
    }

    let img_count = media_lists.1.clone().len();
    if dirlist.len() != img_count {
        let diff = img_count - dirlist.len();
        let com1 = format!("Found {} directories", dirlist.len());
        let com2 = format!("Found {} coverart images", img_count);
        println!("{}", com1);
        println!("{}", com2);
        println!("\nThere are {} directories without coverart images\n", diff);
    }

    println!("{}", mp3_count.clone());
    println!("{}", img_count.clone());

    //NEED ARTIST COUNT FOR ALPHA
    //NEED ALBUM COUNT FOR ALPHA

    let _rmt = run_music_threads(media_lists.0.clone());

    let _gen_artist_count_by_alpha = rusic_utils::artist_album_count_by_alpha();

    let human_total_size = rusic_utils::mp3_total_size(media_lists.0.clone());

    let _rmit = run_music_img_threads(media_lists.1.clone());

    let arids = rusic_artist::unique_artistids();
    let aalbs = rusic_artist::albumids_for_artistid(arids.clone());
    let _insert_aalbs = rusic_artist::write_albums_for_artist_to_db(aalbs.clone()).unwrap();

    let alids = rusic_album::unique_albumids();
    let sids = rusic_album::songids_for_albumid(alids.clone());
    let insert_sids_result = rusicdb::db_main::post_songs_for_album_to_db(sids.clone());
    let _ = match insert_sids_result {
        Ok(_) => String::from("Exit 0 insert_sids"),
        Err(_) => String::from("Exit 1 insert_sids"),
    };
    let _gen_db_check_file = rusic_utils::gen_db_check_file();

    let stats = types::Stats {
        artistcount: "0".to_string(),
        albumcount: "0".to_string(),
        songcount: mp3_count.to_string(),
        imagecount: img_count.to_string(),
    };
    let insert_stats_results = rusicdb::db_main::post_stats_to_db(stats.clone());
    let _ = match insert_stats_results {
        Ok(_) => String::from("Exit 0 insert_stats"),
        Err(_) => String::from("Exit 1 insert_stats"),
    };
    
    println!("\n\nFound {:?} USB devices", usb_drives.len());
    println!("Processed {} Mp3 files", media_lists.0.clone().len());
    println!("Processed {} Jpg files", media_lists.1.clone().len());
    println!("Mp3 size on disk {}", human_total_size);
    "fuck".to_string()
}

fn run_music_threads(alist: Vec<String>) -> bool {
    let mut index = 0;
    let mut page = 1;
    let mut page_count = 0;

    let ofs = env::var("RUSIC_PAGINATION").unwrap();
    let offset: u32 = ofs.trim().parse().expect("offset conversion failed");

    for a in alist {
        index = index + 1;
        if page_count < offset {
            page_count = page_count + 1;
            page = page;
        } else {
            page_count = 1;
            page = page + 1;
        }
        println!("{}", index.clone());

        let _mfi = crate::setup::rusic_process_music::process_mp3s(
            a.clone(),
            index.to_string(),
            page.to_string(),
        );
        // let foo = format!("mfi: {:?}", &mfi);
        // println!("this is foo {}", foo);
    }

    true
}

fn run_music_img_threads(alist: Vec<String>) -> bool {
    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel();

    let mut index = 0;
    let mut page = 1;
    let mut page_count = 0;

    // let ofs = env::var("RUSIC_PAGINATION").unwrap();
    // let offset: u32 = ofs.trim().parse().expect("offset conversion failed");

    for i in alist {
        index = index + 1;
        if page_count < 6 {
            page_count = page_count + 1;
            page = page;
        } else {
            page_count = 1;
            page = page + 1;
        }

        println!("{}", index.clone());

        if i.contains("Music") {
            let tx = tx.clone();
            pool.execute(move || {
                let img_info =
                    rusic_process_music_images::process_music_images(i.clone(), index, page);
                tx.send(img_info).expect("Could not send data");
            });
        }
    }

    drop(tx);
    for t in rx.iter() {
        // Insert this into db
        let _ifo = t;
        // println!("Processed Music img {:?} files", ifo);
    }

    true
}
