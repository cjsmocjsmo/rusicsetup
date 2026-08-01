// SPDX-FileCopyrightText: 2024 Charlie J Smotherman <porthose.cjsmo.cjsmo@gmail.com
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::rusicdb;
use std::env;
use std::sync::mpsc::channel;
use threadpool::ThreadPool;
// use crate::server::fragments;
use crate::types;
use rusqlite::Connection;
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

    let media_lists2 = rusic_walk_dirs::walk_home_dir();
    media_lists.0.extend(media_lists2.0);
    media_lists.1.extend(media_lists2.1);

    let audio_count = media_lists.0.clone().len();
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

    println!("{}", audio_count.clone());
    println!("{}", img_count.clone());

    //NEED ARTIST COUNT FOR ALPHA
    //NEED ALBUM COUNT FOR ALPHA

    let _rmt = run_music_threads(media_lists.0.clone());

    let _gen_artist_count_by_alpha = rusic_utils::artist_album_count_by_alpha();

    let human_total_size = rusic_utils::media_total_size(media_lists.0.clone());

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
        songcount: audio_count.to_string(),
        imagecount: img_count.to_string(),
    };
    let insert_stats_results = rusicdb::db_main::post_stats_to_db(stats.clone());
    let _ = match insert_stats_results {
        Ok(_) => String::from("Exit 0 insert_stats"),
        Err(_) => String::from("Exit 1 insert_stats"),
    };

    println!("\n\nFound {:?} USB devices", usb_drives.len());
    println!("Processed {} audio files", media_lists.0.clone().len());
    println!("Processed {} Jpg files", media_lists.1.clone().len());
    println!("Audio size on disk {}", human_total_size);
    "fuck".to_string()
}

fn run_music_threads(alist: Vec<String>) -> bool {
    let mut index = 0;
    let mut page = 1;
    let mut page_count = 0;

    let ofs = env::var("RUSIC_PAGINATION").unwrap();
    let offset: u32 = ofs.trim().parse().expect("offset conversion failed");
    let batch_size: usize = env::var("RUSIC_SQLITE_BATCH_SIZE")
        .ok()
        .and_then(|v| v.trim().parse::<usize>().ok())
        .filter(|v| *v > 0)
        .unwrap_or(1000);

    let db_path = env::var("RUSIC_DB_PATH").expect("RUSIC_DB_PATH not set");
    let conn = Connection::open(db_path).expect("unable to open db file");
    conn.execute_batch("PRAGMA journal_mode = WAL; PRAGMA synchronous = NORMAL;")
        .expect("unable to configure sqlite pragmas");
    let mut songs = alist.into_iter();

    loop {
        let tx = conn
            .unchecked_transaction()
            .expect("unable to start sqlite transaction");
        let mut wrote_rows = false;

        {
            let mut music_stmt = tx
                .prepare(
                    "INSERT INTO music (
                            rusicid,
                            imgurl,
                            playpath,
                            artist,
                            artistid,
                            album,
                            albumid,
                            song,
                            fullpath,
                            extension,
                            idx,
                            page,
                            fsizeresults
                        )
                        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
                )
                .expect("unable to prepare music insert");

            let mut startswith_stmt = tx
                .prepare(
                    "INSERT INTO startswith (
                            rusicid,
                            artist,
                            album,
                            artistid,
                            albumid,
                            song,
                            artist_first_letter,
                            album_first_letter,
                            song_first_letter
                        )
                        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                )
                .expect("unable to prepare startswith insert");

            let mut artartid_stmt = tx
                .prepare(
                    "INSERT INTO artartid (
                            rusicid,
                            artist,
                            artistid
                        )
                        VALUES (?1, ?2, ?3)",
                )
                .expect("unable to prepare artartid insert");

            let mut albalbid_stmt = tx
                .prepare(
                    "INSERT INTO albalbid (
                            rusicid,
                            imageurl,
                            albumid
                        )
                        VALUES (?1, ?2, ?3)",
                )
                .expect("unable to prepare albalbid insert");

            for _ in 0..batch_size {
                let Some(a) = songs.next() else {
                    break;
                };
                wrote_rows = true;

                index += 1;
                if page_count < offset {
                    page_count += 1;
                } else {
                    page_count = 1;
                    page += 1;
                }
                println!("{}", index);

                let Some((mfi, first_letter_info)) =
                    crate::setup::rusic_process_music::process_audio_file(
                        a.clone(),
                        index.to_string(),
                        page.to_string(),
                    )
                else {
                    continue;
                };

                music_stmt
                    .execute((
                        &mfi.rusicid,
                        &mfi.imgurl,
                        &mfi.playpath,
                        &mfi.artist,
                        &mfi.artistid,
                        &mfi.album,
                        &mfi.albumid,
                        &mfi.song,
                        &mfi.fullpath,
                        &mfi.extension,
                        &mfi.idx,
                        &mfi.page,
                        &mfi.fsizeresults,
                    ))
                    .expect("unable to insert music row");

                startswith_stmt
                    .execute((
                        &first_letter_info.rusicid,
                        &first_letter_info.artist,
                        &first_letter_info.album,
                        &first_letter_info.artistid,
                        &first_letter_info.albumid,
                        &first_letter_info.song,
                        &first_letter_info.artist_first_letter,
                        &first_letter_info.album_first_letter,
                        &first_letter_info.song_first_letter,
                    ))
                    .expect("unable to insert startswith row");

                artartid_stmt
                    .execute((&mfi.rusicid, &mfi.artist, &mfi.artistid))
                    .expect("unable to insert artartid row");

                albalbid_stmt
                    .execute((&mfi.rusicid, &mfi.imgurl, &mfi.albumid))
                    .expect("unable to insert albalbid row");
            }
        }

        tx.commit().expect("unable to commit sqlite transaction");
        if !wrote_rows {
            break;
        }
    }

    true
}

fn run_music_img_threads(alist: Vec<String>) -> bool {
    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel::<Option<types::MusicImageInfo>>();

    let mut index = 0;
    let mut page = 1;
    let mut page_count = 0;

    // let ofs = env::var("RUSIC_PAGINATION").unwrap();
    // let offset: u32 = ofs.trim().parse().expect("offset conversion failed");

    for i in alist {
        index = index + 1;
        if page_count < 6 {
            page_count = page_count + 1;
            // page = page;
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

    let img_batch_size: usize = env::var("RUSIC_IMG_SQLITE_BATCH_SIZE")
        .ok()
        .and_then(|v| v.trim().parse::<usize>().ok())
        .filter(|v| *v > 0)
        .or_else(|| {
            env::var("RUSIC_SQLITE_BATCH_SIZE")
                .ok()
                .and_then(|v| v.trim().parse::<usize>().ok())
                .filter(|v| *v > 0)
        })
        .unwrap_or(1000);

    let db_path = env::var("RUSIC_DB_PATH").expect("RUSIC_DB_PATH not set");
    let conn = Connection::open(db_path).expect("unable to open db file");
    conn.execute_batch("PRAGMA journal_mode = WAL; PRAGMA synchronous = NORMAL;")
        .expect("unable to configure sqlite pragmas");

    let mut img_infos = rx.into_iter().flatten();

    loop {
        let tx = conn
            .unchecked_transaction()
            .expect("unable to start sqlite transaction");
        let mut wrote_rows = false;

        {
            let mut music_img_stmt = tx
                .prepare(
                    "INSERT OR IGNORE INTO music_images (
                            rusicid,
                            width,
                            height,
                            artist,
                            artistid,
                            album,
                            albumid,
                            filesize,
                            fullpath,
                            thumbpath,
                            idx,
                            page,
                            httpthumbpath
                        )
                        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
                )
                .expect("unable to prepare music_images insert");

            for _ in 0..img_batch_size {
                let Some(img_info) = img_infos.next() else {
                    break;
                };
                wrote_rows = true;

                music_img_stmt
                    .execute((
                        &img_info.rusicid,
                        &img_info.width,
                        &img_info.height,
                        &img_info.artist,
                        &img_info.artistid,
                        &img_info.album,
                        &img_info.albumid,
                        &img_info.filesize,
                        &img_info.fullpath,
                        &img_info.thumbpath,
                        &img_info.idx,
                        &img_info.page,
                        &img_info.httpthumbpath,
                    ))
                    .expect("unable to insert music_images row");
            }
        }

        tx.commit().expect("unable to commit sqlite transaction");
        if !wrote_rows {
            break;
        }
    }

    true
}
