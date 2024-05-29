use crate::rusicdb::db_main;
use crate::types;
use filesize::PathExt;
use id3::{Tag, TagLike};
use image::{self};
use md5::{Digest, Md5};
use rusqlite::Result;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

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

        ( artist_final, album_final )
    }

    pub fn get_tag_info(&self) -> Result<(String, String, String, String), std::io::Error> {
        let tag = match Tag::read_from_path(&self.apath) {
            Ok(tag) => tag,
            Err(_) => {
                let target_dir = Path::new("/home/pi/needs_work");
                if !target_dir.exists() {
                    fs::create_dir_all(target_dir)?;
                }
                fs::rename(&self.apath, target_dir.join(Path::new(&self.apath).file_name().unwrap()))?;
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "No ID3 tag found"));
            }
        };
    
        let artist = tag.artist().expect(&self.apath);
        let album = tag.album().expect(&self.apath);
        let song = tag.title().expect(&self.apath);
        let track = tag.track().expect(&self.apath);
    
        Ok((artist.to_string(), album.to_string(), song.to_string(), track.to_string()))
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

        path.size_on_disk().unwrap().to_string()
    }

    pub fn get_dims(&self) -> (u32, u32) {
        let dims = get_image_dims(&self.apath);

        dims
    }
    pub fn artist_starts_with(&self) -> String {
        let tag = Tag::read_from_path(&self.apath).expect(&self.apath);
        let artist = tag.artist().expect(&self.apath);
        let first_letter = artist.chars().next().unwrap();

        first_letter.to_string()
    }

    pub fn album_starts_with(&self) -> String {
        let tag = Tag::read_from_path(&self.apath).expect(&self.apath);
        let album = tag.album().expect(&self.apath);
        let first_letter = album.chars().next().unwrap();

        first_letter.to_string()
    }

    pub fn song_starts_with(&self) -> String {
        let tag = Tag::read_from_path(&self.apath).expect(&self.apath);
        let song = tag.title().expect(&self.apath);
        let first_letter = song.chars().next().unwrap();

        first_letter.to_string()
    }

    pub fn create_mp3_play_path(&self) -> String {
        let psplit = self.apath.split("/").skip(3).collect::<Vec<&str>>();
        let assend = psplit.join("/");

        let myhttpd = env::var("RUSIC_HTTP_ADDR").unwrap();
        let myport = env::var("RUSIC_PORT").unwrap();

        let playpath = myhttpd + &myport + "/Music/" + assend.as_str();

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
    let tags = rus.get_tag_info().unwrap();

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
    let _insertfirsletter = db_main::post_first_letter(first_letter_info).unwrap();

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

pub fn mp3_total_size(media_lists: Vec<String>) -> String {
    let mut mp3_total_size = Vec::new();
    for media in media_lists {
        let rus = RusicUtils {
            apath: media.clone(),
        };
        let fsize = rus.get_file_size();
        let fusize: usize = fsize.parse().unwrap();
        mp3_total_size.push(fusize);
    }
    let sum = mp3_total_size.iter().sum::<usize>();
    let humansum = convert_bytes(sum);

    humansum.to_string()
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
    };
    for letter2 in alphabet.clone() {
        let _album_alpha_count = db_main::post_album_count_by_alpha(letter2.to_string());
    };
    for letter3 in alphabet.clone() {
        let _song_alpha_count = db_main::post_song_count_by_alpha(letter3.to_string());
    };
}