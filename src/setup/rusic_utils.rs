// SPDX-FileCopyrightText: 2024 Charlie J Smotherman <porthose.cjsmo.cjsmo@gmail.com
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::rusicdb::db_main;
use crate::types;
use filesize::PathExt;
use image::{self};
use lofty::file::TaggedFileExt;
use lofty::prelude::Accessor;
use lofty::probe::Probe;
use md5::{Digest, Md5};
use rusqlite::Result;
use std::env;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct RusicUtils {
    pub apath: String,
}

impl RusicUtils {
    pub fn split_base_dir_filename(&self) -> (String, String) {
        let path = Path::new(&self.apath);
        let dir_path = path.parent().unwrap();
        let filename = path.file_name().unwrap();

        (
            dir_path.to_string_lossy().to_string(),
            filename.to_string_lossy().to_string(),
        )
    }

    pub fn split_artist_album(&self) -> (String, String) {
        let path = Path::new(&self.apath);
        let basedir = path.parent().unwrap();
        let basedirpath = Path::new(&basedir);
        let album = basedirpath.file_name().unwrap();
        let basedirpath2 = basedirpath.parent().unwrap();
        let bdp3 = Path::new(&basedirpath2);
        let artist = bdp3.file_name().unwrap();
        let album_string = album.to_string_lossy().to_string();
        let artist_string = artist.to_string_lossy().to_string();

        let album_final = album_string.replace("_", " ");
        let artist_final = artist_string.replace("_", " ");

        (artist_final, album_final)
    }

    pub fn get_tag_info(&self) -> Result<(String, String, String, String), std::io::Error> {
        let tagged_file = Probe::open(&self.apath)
            .map_err(|e| move_to_needs_work(&self.apath, e.to_string()))?
            .read()
            .map_err(|e| move_to_needs_work(&self.apath, e.to_string()))?;

        let tag = tagged_file
            .primary_tag()
            .or_else(|| tagged_file.first_tag())
            .ok_or_else(|| move_to_needs_work(&self.apath, "No metadata tag found".to_string()))?;

        let artist = tag
            .artist()
            .ok_or_else(|| move_to_needs_work(&self.apath, "Missing artist tag".to_string()))?;
        let album = tag
            .album()
            .ok_or_else(|| move_to_needs_work(&self.apath, "Missing album tag".to_string()))?;
        let song = tag
            .title()
            .ok_or_else(|| move_to_needs_work(&self.apath, "Missing title tag".to_string()))?;
        let track = tag.track().unwrap_or(0);

        Ok((
            artist.to_string(),
            album.to_string(),
            song.to_string(),
            track.to_string(),
        ))
    }

    pub fn split_ext(&self) -> String {
        let path = Path::new(&self.apath);
        let boo_results = path.extension();
        let boo = match boo_results {
            Some(b) => b.to_string_lossy().to_string(),
            None => "split_ext did not work".to_string(),
        };
        let ext = ".".to_string() + boo.as_str();

        ext
    }

    pub fn get_file_size(&self) -> String {
        let path = Path::new(&self.apath);
        match path.size_on_disk() {
            Ok(size) => size.to_string(),
            Err(err) => {
                eprintln!("Unable to read file size for {}: {}", self.apath, err);
                "0".to_string()
            }
        }
    }

    pub fn get_dims(&self) -> (u32, u32) {
        let dims = get_image_dims(&self.apath);

        dims
    }
    pub fn artist_starts_with(&self) -> String {
        match self.get_tag_info() {
            Ok(tag) => tag.0.chars().next().unwrap_or('_').to_string(),
            Err(err) => {
                eprintln!("Unable to read artist tag for {}: {}", self.apath, err);
                "_".to_string()
            }
        }
    }

    pub fn album_starts_with(&self) -> String {
        match self.get_tag_info() {
            Ok(tag) => tag.1.chars().next().unwrap_or('_').to_string(),
            Err(err) => {
                eprintln!("Unable to read album tag for {}: {}", self.apath, err);
                "_".to_string()
            }
        }
    }

    pub fn song_starts_with(&self) -> String {
        match self.get_tag_info() {
            Ok(tag) => tag.2.chars().next().unwrap_or('_').to_string(),
            Err(err) => {
                eprintln!("Unable to read song tag for {}: {}", self.apath, err);
                "_".to_string()
            }
        }
    }

    pub fn create_audio_play_path(&self) -> String {
        let assend = self
            .apath
            .split_once("/Music/")
            .map(|(_, music_path)| format!("Music/{}", music_path))
            .unwrap_or_else(|| self.apath.trim_start_matches('/').to_string());

        let myhttpd = env::var("RUSIC_HTTP_ADDR").unwrap();
        let myport = env::var("RUSIC_PORT").unwrap();

        let playpath = myhttpd + &myport + "/" + assend.as_str();

        playpath
    }
}

pub fn get_md5(z: String) -> String {
    let mut hasher2 = Md5::new();
    hasher2.update(&z);
    let a_id = hasher2.finalize();
    let foo = format!("{:x}", a_id);

    foo
}

pub fn calc_playtime(path: &str) -> Result<String, std::io::Error> {
    let output = Command::new("ffprobe")
        .args([
            "-v",
            "error",
            "-show_entries",
            "format=duration",
            "-of",
            "csv=p=0",
        ])
        .arg(path)
        .output()
        .map_err(|err| {
            std::io::Error::new(
                err.kind(),
                format!("Unable to run ffprobe for {}: {}", path, err),
            )
        })?;

    if !output.status.success() {
        let message = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("ffprobe failed for {}: {}", path, message),
        ));
    }

    let playtime = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let duration = playtime.parse::<f64>();
    if playtime.is_empty()
        || duration
            .as_ref()
            .map(|value| !value.is_finite() || *value < 0.0)
            .unwrap_or(true)
    {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("ffprobe returned an invalid duration for {}", path),
        ));
    }

    Ok(playtime)
}

fn get_image_dims(x: &String) -> (u32, u32) {
    let dims_rs = image::image_dimensions(&x);
    let dims = match dims_rs {
        Ok(d) => d,
        Err(_) => (0, 0),
    };

    dims
}

pub fn normalize_music_image(dims: (u32, u32)) -> (u32, u32) {
    let largest: u32;

    if dims.0 == dims.1 {
        largest = dims.0;
    } else if dims.0 > dims.1 {
        largest = dims.0;
    } else {
        largest = dims.1;
    }

    let resizetup: (u32, u32);
    if largest < 100 {
        resizetup = (100, 100);
    } else if largest < 200 {
        resizetup = (200, 200);
    } else if largest < 300 {
        resizetup = (300, 300);
    } else {
        resizetup = (300, 300);
    }

    resizetup
}

pub fn gen_db_check_file() {
    let db_check_file_path = env::var("RUSIC_DB_CHECK_FILE_PATH").unwrap();
    let mut file = File::create(db_check_file_path).unwrap();
    file.write_all(b"1").unwrap();
}

pub fn is_db_check_file_present() -> bool {
    let db_check_file_path = env::var("RUSIC_DB_CHECK_FILE_PATH").unwrap();
    let path = Path::new(&db_check_file_path);

    path.exists()
}

pub fn gen_first_letter_db(media: String) -> Result<()> {
    let rus = RusicUtils {
        apath: media.clone(),
    };
    let tags = match rus.get_tag_info() {
        Ok(tags) => tags,
        Err(err) => {
            eprintln!("Skipping first-letter generation for {}: {}", media, err);
            return Ok(());
        }
    };

    let first_letter_info = types::FirstLetterInfo {
        rusicid: get_md5(media.clone()),
        artist: tags.0.clone(),
        album: tags.1.clone(),
        song: tags.2.clone(),
        artistid: get_md5(tags.0.clone()),
        albumid: get_md5(tags.1.clone()),
        artist_first_letter: rus.artist_starts_with(),
        album_first_letter: rus.album_starts_with(),
        song_first_letter: rus.song_starts_with(),
    };
    db_main::post_first_letter(first_letter_info)?;

    Ok(())
}

pub fn convert_bytes(mut bytes: usize) -> String {
    let units = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
    let mut i = 0;
    while bytes >= 1024 {
        bytes /= 1024;
        i += 1;
    }
    return format!("{:.2} {}", bytes, units[i]);
}

pub fn media_total_size(media_lists: Vec<String>) -> String {
    let mut total_size = Vec::new();
    for media in media_lists {
        let rus = RusicUtils {
            apath: media.clone(),
        };
        let fsize = rus.get_file_size();
        let fusize: usize = fsize.parse().unwrap_or(0);
        total_size.push(fusize);
    }
    let sum = total_size.iter().sum::<usize>();
    let humansum = convert_bytes(sum);

    humansum.to_string()
}

fn move_to_needs_work(apath: &str, message: String) -> std::io::Error {
    let target_dir = Path::new("/home/pi/needs_work");
    log_tag_issue(apath, &message);

    let move_result = (|| -> Result<(), std::io::Error> {
        if !target_dir.exists() {
            fs::create_dir_all(target_dir)?;
        }
        fs::rename(apath, target_dir.join(Path::new(apath).file_name().unwrap()))?;
        Ok(())
    })();

    if move_result.is_err() {
        log_tag_issue(
            apath,
            "Failed to move file to /home/pi/needs_work after tag error",
        );
        return std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("{}; additionally failed to move file to /home/pi/needs_work", message),
        );
    }

    std::io::Error::new(std::io::ErrorKind::Other, message)
}

fn log_tag_issue(apath: &str, message: &str) {
    let log_path = env::var("RUSIC_TAG_ISSUES_LOG")
        .unwrap_or_else(|_| "/home/pi/needs_work/tag_issues.log".to_string());
    let log_file_path = Path::new(&log_path);

    if let Some(parent) = log_file_path.parent() {
        if let Err(err) = fs::create_dir_all(parent) {
            eprintln!(
                "Unable to create tag issue log directory {}: {}",
                parent.display(),
                err
            );
            return;
        }
    }

    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let line = format!("{} | {} | {}\n", ts, apath, message);
    match OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file_path)
    {
        Ok(mut file) => {
            if let Err(err) = file.write_all(line.as_bytes()) {
                eprintln!("Unable to write tag issue log {}: {}", log_path, err);
            }
        }
        Err(err) => {
            eprintln!("Unable to open tag issue log {}: {}", log_path, err);
        }
    }
}

pub fn artist_album_count_by_alpha() {
    let mut alphabet = Vec::new();
    alphabet.push("A");
    alphabet.push("B");
    alphabet.push("C");
    alphabet.push("D");
    alphabet.push("E");
    alphabet.push("F");
    alphabet.push("G");
    alphabet.push("H");
    alphabet.push("I");
    alphabet.push("J");
    alphabet.push("K");
    alphabet.push("L");
    alphabet.push("M");
    alphabet.push("N");
    alphabet.push("O");
    alphabet.push("P");
    alphabet.push("Q");
    alphabet.push("R");
    alphabet.push("S");
    alphabet.push("T");
    alphabet.push("U");
    alphabet.push("V");
    alphabet.push("W");
    alphabet.push("X");
    alphabet.push("Y");
    alphabet.push("Z");

    for letter in alphabet.clone() {
        let _artist_alpha_count = db_main::post_artist_count_by_alpha(letter.to_string());
    }
    for letter2 in alphabet.clone() {
        let _album_alpha_count = db_main::post_album_count_by_alpha(letter2.to_string());
    }
    for letter3 in alphabet.clone() {
        let _song_alpha_count = db_main::post_song_count_by_alpha(letter3.to_string());
    }
}
