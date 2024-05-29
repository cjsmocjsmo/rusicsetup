use crate::setup::rusic_utils;
use crate::setup::rusic_utils::RusicUtils;
// use rusqlite::{Connection, Result};
use std::clone::Clone;
use std::env;
use std::fs::remove_file;
use std::path::Path;
use webp::*;
use crate::types;
use crate::rusicdb::db_main;

//NEED TO PROCESS FOR CONVERT PNG GIF WEBP TO JPG
pub fn process_music_images(x: String, index: i32, pageg: i32) -> i32 {
    let mut needs_to_be_processed = Vec::new();
    if x.ends_with("webp") {
        println!(".webp found converting to jpg: {:?}", x);
        let new_fname = convert_webp_to_jpg(x.clone());
        needs_to_be_processed.push(new_fname);
    } else if x.ends_with("jpeg") {
        println!(".jpeg found please convert to jpg: {:?}", x);
        let new_fname = convert_jpeg_to_jpg(x.clone());
        needs_to_be_processed.push(new_fname);
    } else if x.ends_with("png") {
        println!(".png found please convert to jpg: {:?}", x);
        let new_fname = convert_png_to_jpg(x.clone());
        needs_to_be_processed.push(new_fname);
    } else if x.ends_with("gif") {
        println!(".gif found please convert to jpg: {:?}", x);
        let new_fname = convert_gif_to_jpg(x.clone());
        needs_to_be_processed.push(new_fname);
    } else {
        needs_to_be_processed.push(x.clone());
    }

    let media = needs_to_be_processed[0].clone();

    let foo2 = RusicUtils {
        apath: media.clone(),
    };
    let id = rusic_utils::get_md5(x.clone());
    let dims = RusicUtils::get_dims(&foo2);
    let artalb = RusicUtils::split_artist_album(&foo2);
    let artist1 = artalb.0;
    let album1 = artalb.1;

    if dims != (0, 0) {
        let newdims = crate::setup::rusic_utils::normalize_music_image(dims);
        let width_r = newdims.0.to_string();
        let height_r = newdims.1.to_string();
        let fsize_results = RusicUtils::get_file_size(&foo2).to_string();
        let full_path = &x.to_string();
        let tpath = create_music_thumbnail(&x, artist1.clone(), album1.clone());
        let thumb_path = tpath.0;
        let http_thumb_path = tpath.1;

        let music_img_info = types::MusicImageInfo {
            rusicid: id,
            width: width_r,
            height: height_r,
            artist: artist1.clone(),
            artistid: rusic_utils::get_md5(artist1.clone()),
            album: album1.clone(),
            albumid: rusic_utils::get_md5(album1.clone()),
            filesize: fsize_results,
            fullpath: full_path.to_string(),
            thumbpath: thumb_path,
            idx: index.to_string(),
            page: pageg.to_string(),
            httpthumbpath: http_thumb_path,
        };
        write_music_img_to_file(music_img_info.clone(), index);
        db_main::post_music_img_to_db(music_img_info.clone()).expect("music image db insertion failed")
    };

    index
}

fn create_music_thumbnail(x: &String, art: String, alb: String) -> (String, String) {
    let rusic_music_metadata_path = env::var("RUSIC_THUMBS").expect("$RUSIC_THUMBS is not set");
    let new_fname = "/".to_string() + art.as_str() + "_-_" + alb.as_str() + ".jpg";
    let ofname = rusic_music_metadata_path + &new_fname;
    let out_fname = ofname.replace(" ", "_");
    println!("out_fname: {:?}", out_fname);
    let server_addr = env::var("RUSIC_HTTP_ADDR").expect("$RUSIC_SERVER_ADDR is not set");
    let server_port = env::var("RUSIC_PORT").expect("$RUSIC_SERVER_PORT is not set");
    let http_path_1 = server_addr + &server_port + "/assets/" + &new_fname;
    let http_path = http_path_1.replace(" ", "_");
    let img = image::open(x).expect("ooooh fuck it didnt open");
    let thumbnail = img.resize(200, 200, image::imageops::FilterType::Lanczos3);
    thumbnail
        .save(out_fname.clone())
        .expect(&out_fname);

    (out_fname.to_string(), http_path)
}

fn write_music_img_to_file(miinfo: types::MusicImageInfo, index: i32) {
    let mii = serde_json::to_string(&miinfo).unwrap();
    let rusic_music_metadata_path = env::var("RUSIC_NFOS").expect("$RUSIC_NFOS is not set");
    let outpath = format!(
        "{}/Music_Image_Meta_{}.json",
        rusic_music_metadata_path.as_str(),
        &index
    );
    std::fs::write(outpath, mii.clone()).unwrap();
}

fn convert_webp_to_jpg(x: String) -> String {
    let img = image::open(x.clone()).unwrap();
    let encoder = Encoder::from_image(&img).unwrap();
    let output_buffer = encoder.encode(90f32);

    let path = Path::new(&x);

    let new_filename = path.with_extension("");
    let new_filename = new_filename.into_os_string().to_str().unwrap().to_owned() + ".jpg";

    std::fs::write(new_filename.clone(), &*output_buffer).unwrap();

    let rm_path = Path::new(&x);
    remove_file(rm_path).expect("unable to remove webp file");

    new_filename.clone()
}

fn convert_jpeg_to_jpg(x: String) -> String {
    let img = image::open(x.clone()).unwrap();
    let encoder = Encoder::from_image(&img).unwrap();
    let output_buffer = encoder.encode(90f32);

    let path = Path::new(&x);

    let new_filename = path.with_extension("");
    let new_filename = new_filename.into_os_string().to_str().unwrap().to_owned() + ".jpg";

    std::fs::write(new_filename.clone(), &*output_buffer).unwrap();

    let rm_path = Path::new(&x);
    remove_file(rm_path).expect("unable to remove webp file");

    new_filename.clone()
}

fn convert_png_to_jpg(x: String) -> String {
    let img = image::open(x.clone()).unwrap();
    let encoder = Encoder::from_image(&img).unwrap();
    let output_buffer = encoder.encode(90f32);

    let path = Path::new(&x);

    let new_filename = path.with_extension("");
    let new_filename = new_filename.into_os_string().to_str().unwrap().to_owned() + ".jpg";

    std::fs::write(new_filename.clone(), &*output_buffer).unwrap();
    let rm_path = Path::new(&x);
    remove_file(rm_path).expect("unable to remove webp file");

    new_filename.clone()
}

fn convert_gif_to_jpg(x: String) -> String {
    let img = image::open(x.clone()).unwrap();
    let encoder = Encoder::from_image(&img).unwrap();
    let output_buffer = encoder.encode(90f32);

    let path = Path::new(&x);

    let new_filename = path.with_extension("");
    let new_filename = new_filename.into_os_string().to_str().unwrap().to_owned() + ".jpg";

    std::fs::write(new_filename.clone(), &*output_buffer).unwrap();
    // let rm_path = Path::new(&x);
    // remove_file(rm_path).expect("unable to remove webp file");

    new_filename.clone()
}
