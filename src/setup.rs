// SPDX-FileCopyrightText: 2024 Charlie J Smotherman <porthose.cjsmo.cjsmo@gmail.com
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::rusicdb;
use std::env;
use std::fs;
use std::collections::{HashMap, HashSet};
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};
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

fn health_check_interval() -> Option<Duration> {
    let secs = env::var("RUSIC_HEALTHCHECK_SECS")
        .ok()
        .and_then(|v| v.trim().parse::<u64>().ok())
        .unwrap_or(30);

    if secs == 0 {
        None
    } else {
        Some(Duration::from_secs(secs))
    }
}

fn maybe_print_health_check(
    stage: &str,
    processed: usize,
    total: usize,
    started_at: Instant,
    last_report: &mut Instant,
    interval: Option<Duration>,
) {
    let Some(interval) = interval else {
        return;
    };

    if last_report.elapsed() < interval {
        return;
    }

    println!(
        "Health check [{}]: processed {}/{} items in {}s",
        stage,
        processed,
        total,
        started_at.elapsed().as_secs()
    );
    *last_report = Instant::now();
}

fn env_var_truthy(name: &str) -> bool {
    env::var(name)
        .ok()
        .map(|v| {
            matches!(
                v.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
        .unwrap_or(false)
}

fn parent_dir_string(path: &str) -> String {
    Path::new(path)
        .parent()
        .unwrap_or(Path::new("."))
        .to_string_lossy()
        .to_string()
}

fn write_missing_coverart_report(missing_dirs: &[String]) -> String {
    let report_path = env::var("RUSIC_MISSING_COVERART_REPORT_PATH")
        .unwrap_or_else(|_| "missing_coverart_dirs.txt".to_string());

    let mut report_lines = Vec::new();
    report_lines.push(format!("Missing coverart directories: {}", missing_dirs.len()));
    report_lines.push(String::new());

    for dir in missing_dirs {
        report_lines.push(dir.clone());
    }

    let payload = report_lines.join("\n");
    if let Err(err) = fs::write(&report_path, payload) {
        eprintln!(
            "Failed to write missing coverart report {}: {}",
            report_path, err
        );
    }

    report_path
}

fn print_directory_list(header: &str, dirs: &[String]) {
    println!("{}", header);
    if dirs.is_empty() {
        println!("- none");
        return;
    }

    for dir in dirs {
        println!("- {}", dir);
    }
}

fn print_directory_count_list(header: &str, dirs: &[(String, usize)]) {
    println!("{}", header);
    if dirs.is_empty() {
        println!("- none");
        return;
    }

    for (dir, count) in dirs {
        println!("- {} ({} images)", dir, count);
    }
}

pub fn setup() -> String {
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

    let audio_count = media_lists.0.len();
    let img_count = media_lists.1.len();

    let audio_dirs: HashSet<String> = media_lists
        .0
        .iter()
        .map(|media| parent_dir_string(media))
        .collect();

    let image_dirs: HashSet<String> = media_lists
        .1
        .iter()
        .map(|media| parent_dir_string(media))
        .collect();

    let mut missing_coverart_dirs: Vec<String> =
        audio_dirs.difference(&image_dirs).cloned().collect();
    missing_coverart_dirs.sort();

    let covered_audio_dir_count = audio_dirs.intersection(&image_dirs).count();
    let mut orphan_coverart_dirs: Vec<String> = image_dirs.difference(&audio_dirs).cloned().collect();
    orphan_coverart_dirs.sort();
    let orphan_coverart_dir_count = orphan_coverart_dirs.len();

    let mut image_file_count_by_dir: HashMap<String, usize> = HashMap::new();
    for img_path in &media_lists.1 {
        let dir = parent_dir_string(img_path);
        *image_file_count_by_dir.entry(dir).or_insert(0) += 1;
    }

    let mut multi_coverart_dirs: Vec<(String, usize)> = image_file_count_by_dir
        .iter()
        .filter(|(_, count)| **count > 1)
        .map(|(dir, count)| (dir.clone(), *count))
        .collect();
    multi_coverart_dirs.sort_by(|a, b| a.0.cmp(&b.0));

    let dirs_with_multiple_coverart = multi_coverart_dirs.len();

    println!("Found {} audio directories", audio_dirs.len());
    println!("Found {} coverart image files", img_count);
    println!("Found {} directories with coverart", covered_audio_dir_count);
    println!(
        "There are {} directories without coverart images",
        missing_coverart_dirs.len()
    );
    println!(
        "There are {} directories with multiple coverart images",
        dirs_with_multiple_coverart
    );
    println!(
        "There are {} coverart directories without audio files",
        orphan_coverart_dir_count
    );

    let report_path = write_missing_coverart_report(&missing_coverart_dirs);
    println!("Missing coverart report path: {}", report_path);

    print_directory_list(
        "Directories without coverart images:",
        &missing_coverart_dirs,
    );
    print_directory_count_list(
        "Directories with multiple coverart images:",
        &multi_coverart_dirs,
    );
    print_directory_list(
        "Coverart directories without audio files:",
        &orphan_coverart_dirs,
    );

    if env_var_truthy("RUSIC_REPORT_ONLY") {
        println!("RUSIC_REPORT_ONLY enabled: scan/report completed, skipping DB and image processing");
        return "report-only".to_string();
    }

    let _create_tables = rusicdb::db_tables::create_tables();

    println!("{}", audio_count);
    println!("{}", img_count);

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
    let total_songs = alist.len();
    let mut index = 0;
    let mut page = 1;
    let mut page_count = 0;
    let started_at = Instant::now();
    let mut last_report = started_at;
    let health_check_interval = health_check_interval();

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
                    "INSERT OR IGNORE INTO music (
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
                    "INSERT OR IGNORE INTO startswith (
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
                    "INSERT OR IGNORE INTO artartid (
                            rusicid,
                            artist,
                            artistid
                        )
                        VALUES (?1, ?2, ?3)",
                )
                .expect("unable to prepare artartid insert");

            let mut albalbid_stmt = tx
                .prepare(
                    "INSERT OR IGNORE INTO albalbid (
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
                maybe_print_health_check(
                    "audio scan",
                    index,
                    total_songs,
                    started_at,
                    &mut last_report,
                    health_check_interval,
                );

                let Some((mfi, first_letter_info)) =
                    crate::setup::rusic_process_music::process_audio_file(
                        a.clone(),
                        index.to_string(),
                        page.to_string(),
                    )
                else {
                    continue;
                };

                let inserted_music_rows = music_stmt
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
                    .unwrap_or_else(|err| {
                        panic!(
                            "unable to insert music row for {} ({}) with rusicid {}: {}",
                            mfi.fullpath, mfi.song, mfi.rusicid, err
                        )
                    });

                if inserted_music_rows == 0 {
                    eprintln!(
                        "Skipping duplicate music row for {} with rusicid {}",
                        mfi.fullpath, mfi.rusicid
                    );
                    continue;
                }

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

    let total_images = alist.len();
    let mut index: i32 = 0;
    let mut page: i32 = 1;
    let mut page_count: i32 = 0;
    let started_at = Instant::now();
    let mut last_report = started_at;
    let health_check_interval = health_check_interval();

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

        maybe_print_health_check(
            "image scan",
            index as usize,
            total_images,
            started_at,
            &mut last_report,
            health_check_interval,
        );

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
