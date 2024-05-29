use env_logger::{Builder, Target};
use std::time::Instant;
use clap::{Arg, Command};
use dotenv::dotenv;
// pub mod envvars;
pub mod setup;
pub mod rusicdb;
pub mod types;

fn main() -> std::io::Result<()> {
    let matches = Command::new("My Program")
        .version("1.0")
        .author("Porthose <porthose.cjsmo.cjsmo@gmail.com>")
        .about("Rusic Setup Script")
        .arg(Arg::new("")
            .short('d')
            .long("default")
            .value_name("DEFAULT")
            .help("Sets the program to use default settings"))

        .arg(Arg::new("musicpath")
            .short('m')
            .long("musicpath")
            .value_name("MUSICPATH")
            .help("Sets the program music path"))

        .arg(Arg::new("usbpath")
            .short('u')
            .long("usbpath")
            .value_name("USBPATH")
            .help("Sets the program usb path"))

        .arg(Arg::new("dbchkfile")
            .short('D')
            .long("dbchkfile")
            .value_name("DBCHKFILE")
            .help("Sets the program dbchkfile path"))

        .arg(Arg::new("dbpath")
            .short('w')
            .long("dbpath")
            .value_name("DBPATH")
            .help("Sets the program db path"))

        .arg(Arg::new("noart")
            .short('Q')
            .long("noart")
            .value_name("NOART")
            .help("Sets the program no_art_pic path"))

        .arg(Arg::new("pagination")
            .short('z')
            .long("pagination")
            .value_name("PAGINATION")
            .help("Sets the number of items to display per page"))

        .arg(Arg::new("rusicpath")
            .short('b')
            .long("rusicpath")
            .value_name("RUSICPATH")
            .help("Sets the Path to write files to"))

        .arg(Arg::new("thumbnails")
            .short('t')
            .long("thumbnails")
            .value_name("THUMBNAILS")
            .help("Sets the Path to write thumbnails to"))

        .arg(Arg::new("nfos")
            .short('n')
            .long("nfo")
            .value_name("NFOS")
            .help("Sets the Path to write nfos to"))

        .arg(Arg::new("raddress")
            .short('r')
            .long("raddr")
            .value_name("RADDRESS")
            .help("Sets the raw http address such as 192.168.0.97"))

        .arg(Arg::new("address")
            .short('a')
            .long("addr")
            .value_name("ADDRESS")
            .help("Sets the http address such as http://192.168.0.97"))

        .arg(Arg::new("port")
            .short('p')
            .long("port")
            .value_name("PORT")
            .help("Sets the port to use"))
        .get_matches();

    if let Some(musicpath) = matches.get_one::<String>("musicpath") {
        std::env::set_var("RUSIC_MUSIC", musicpath);
    }

    if let Some(usbpath) = matches.get_one::<String>("usbpath") {
        std::env::set_var("RUSIC_USB", usbpath);
    }

    if let Some(dbchkfile) = matches.get_one::<String>("dbchkfile") {
        std::env::set_var("RUSIC_DB_CHK_FILE_PATH", dbchkfile);
    }

    if let Some(dbpath) = matches.get_one::<String>("dbpath") {
        std::env::set_var("RUSIC_DB_PATH", dbpath);
    }

    if let Some(noart) = matches.get_one::<String>("noart") {
        std::env::set_var("RUSIC_NO_ART_PIC", noart);
    }

    if let Some(pagination) = matches.get_one::<String>("pagination") {
        std::env::set_var("RUSIC_PAGINATION", pagination);
    }

    if let Some(rusicpath) = matches.get_one::<String>("rusicpath") {
        std::env::set_var("RUSIC_PATH", rusicpath);
    }

    if let Some(thumbnails) = matches.get_one::<String>("thumbnails") {
        std::env::set_var("RUSIC_THUMBS", thumbnails);
    }

    if let Some(nfos) = matches.get_one::<String>("nfos") {
        std::env::set_var("RUSIC_NFOS", nfos);
    }

    if let Some(raddress) = matches.get_one::<String>("raddress") {
        std::env::set_var("RUSIC_RAW_HTTP", raddress);
    }

    if let Some(address) = matches.get_one::<String>("address") {
        std::env::set_var("RUSIC_HTTP_ADDR", address);
    }

    if let Some(port) = matches.get_one::<String>("port") {
        std::env::set_var("RUSIC_PORT", port);
    }

    if let Some(default) = matches.get_one::<String>("default") {
        println!("{}", default);
        dotenv().ok();
    }

    let start = Instant::now();
    Builder::new().target(Target::Stdout).init();

    log::info!("Rusic setup started");

    let _setup = setup::setup();

    let duration = start.elapsed();
    if duration.as_secs() < 60 {
        log::info!("Setup completed in: {} seconds", duration.as_secs());
        println!("Setup completed in: {} seconds", duration.as_secs());
    } else {
        let minutes = duration.as_secs() / 60;
        log::info!("Setup completed in: {} minutes", minutes);
        println!("Setup completed in: {} minutes", minutes);
    }

    Ok(())
}
