// SPDX-FileCopyrightText: 2024 Charlie J Smotherman <porthose.cjsmo.cjsmo@gmail.com
//
// SPDX-License-Identifier: GPL-3.0-or-later


use env_logger::{Builder, Target};
use std::time::Instant;
// use clap::{Arg, Command};
use dotenv::dotenv;
// pub mod envvars;
pub mod setup;
pub mod rusicdb;
pub mod types;

fn main() -> std::io::Result<()> {

    dotenv().ok();
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
