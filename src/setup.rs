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

    let _rmt = run_music_threads(media_lists.0.clone());

    let human_total_size = rusic_utils::media_total_size(media_lists.0.clone());

    let _rmit = run_music_img_threads(media_lists.1.clone());

    let _gen_db_check_file = rusic_utils::gen_db_check_file();

    let stats_result = rusicdb::db_main::compute_stats(img_count as i64);
    let _ = match stats_result {
        Ok(stats) => rusicdb::db_main::post_stats_to_db(stats),
        Err(_) => Ok(()),
    };

    println!("\n\nFound {:?} USB devices", usb_drives.len());
    println!("Processed {} audio files", media_lists.0.clone().len());
    println!("Processed {} Jpg files", media_lists.1.clone().len());
    println!("Audio size on disk {}", human_total_size);
    "fuck".to_string()
}

fn run_music_threads(alist: Vec<String>) -> bool {
    let total_songs = alist.len();
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

    // Create thread pool for parallel audio file tag reading
    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel::<(usize, Option<(types::Artist, types::Album, types::Song)>)>();

    // Send all files to thread pool with their indices
    for (file_index, audio_file) in alist.into_iter().enumerate() {
        let tx = tx.clone();
        pool.execute(move || {
            let result = crate::setup::rusic_process_music::process_audio_file(
                audio_file.clone(),
                (file_index + 1).to_string(),
                "0".to_string(), // placeholder, will calculate after sorting
            );
            tx.send((file_index, result)).expect("Could not send data");
        });
    }

    drop(tx);

    // Collect results from channel and rebuild in order
    let mut results: Vec<(usize, Option<(types::Artist, types::Album, types::Song)>)> =
        rx.iter().collect();
    results.sort_by_key(|r| r.0);

    // Now write to database in batches with correct page numbers
    let mut index = 0;
    let mut page = 1;
    let mut page_count = 0;
    let mut batch_items = Vec::new();

    for (_file_index, result_opt) in results {
        index += 1;

        if page_count < offset as usize {
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

        if let Some((artist, album, mut song)) = result_opt {
            // Update song index and page now that we know the correct values
            song.idx = index.to_string();
            song.page = page.to_string();
            batch_items.push((artist, album, song));

            if batch_items.len() >= batch_size {
                write_songs_batch_to_db(&conn, &batch_items);
                batch_items.clear();
            }
        }
    }

    // Write any remaining items
    if !batch_items.is_empty() {
        write_songs_batch_to_db(&conn, &batch_items);
    }

    true
}

fn write_songs_batch_to_db(
    conn: &Connection,
    batch: &[(types::Artist, types::Album, types::Song)],
) {
    if batch.is_empty() {
        return;
    }

    let tx = conn
        .unchecked_transaction()
        .expect("unable to start sqlite transaction");
    {
        let mut artist_stmt = tx
            .prepare(
                "INSERT INTO artists (artistid, name, first_letter)
                    VALUES (?1, ?2, ?3)
                    ON CONFLICT(artistid) DO NOTHING",
            )
            .expect("unable to prepare artists insert");

        let mut album_stmt = tx
            .prepare(
                "INSERT INTO albums (albumid, artistid, name, first_letter)
                    VALUES (?1, ?2, ?3, ?4)
                    ON CONFLICT(albumid) DO NOTHING",
            )
            .expect("unable to prepare albums insert");

        let mut song_stmt = tx
            .prepare(
                "INSERT OR IGNORE INTO songs (
                        rusicid,
                        albumid,
                        title,
                        imgurl,
                        playpath,
                        fullpath,
                        extension,
                        idx,
                        page,
                        fsizeresults,
                        first_letter,
                        duration
                    )
                    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
            )
            .expect("unable to prepare songs insert");

        for (artist, album, song) in batch {
            artist_stmt
                .execute((&artist.artistid, &artist.name, &artist.first_letter))
                .expect("unable to insert artist row");

            album_stmt
                .execute((
                    &album.albumid,
                    &album.artistid,
                    &album.name,
                    &album.first_letter,
                ))
                .expect("unable to insert album row");

            let inserted_song_rows = song_stmt
                .execute((
                    &song.rusicid,
                    &song.albumid,
                    &song.title,
                    &song.imgurl,
                    &song.playpath,
                    &song.fullpath,
                    &song.extension,
                    &song.idx,
                    &song.page,
                    &song.fsizeresults,
                    &song.first_letter,
                    &song.duration,
                ))
                .unwrap_or_else(|err| {
                    panic!(
                        "unable to insert song row for {} ({}) with rusicid {}: {}",
                        song.fullpath, song.title, song.rusicid, err
                    )
                });

            if inserted_song_rows == 0 {
                eprintln!(
                    "Skipping duplicate song row for {} with rusicid {}",
                    song.fullpath, song.rusicid
                );
            }
        }
    }

    tx.commit().expect("unable to commit sqlite transaction");
}

fn run_music_img_threads(alist: Vec<String>) -> bool {
    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel::<Option<types::AlbumImage>>();

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
            let mut album_image_stmt = tx
                .prepare(
                    "INSERT OR IGNORE INTO album_images (
                            albumid,
                            width,
                            height,
                            filesize,
                            fullpath,
                            thumbpath,
                            idx,
                            page,
                            httpthumbpath
                        )
                        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                )
                .expect("unable to prepare album_images insert");

            for _ in 0..img_batch_size {
                let Some(img_info) = img_infos.next() else {
                    break;
                };
                wrote_rows = true;

                album_image_stmt
                    .execute((
                        &img_info.albumid,
                        &img_info.width,
                        &img_info.height,
                        &img_info.filesize,
                        &img_info.fullpath,
                        &img_info.thumbpath,
                        &img_info.idx,
                        &img_info.page,
                        &img_info.httpthumbpath,
                    ))
                    .expect("unable to insert album_images row");
            }
        }

        tx.commit().expect("unable to commit sqlite transaction");
        if !wrote_rows {
            break;
        }
    }

    true
}
