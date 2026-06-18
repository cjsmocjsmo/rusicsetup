// SPDX-FileCopyrightText: 2024 Charlie J Smotherman <porthose.cjsmo.cjsmo@gmail.com
//
// SPDX-License-Identifier: GPL-3.0-or-later


use env_logger::{Builder, Target};
use std::path::PathBuf;
use std::time::Instant;
// use clap::{Arg, Command};
use dotenv;
// pub mod envvars;
pub mod setup;
pub mod rusicdb;
pub mod types;

fn load_env_file() -> (Option<PathBuf>, Vec<PathBuf>) {
    let mut attempts: Vec<PathBuf> = Vec::new();

    if let Ok(cwd) = std::env::current_dir() {
        attempts.push(cwd.join(".env"));
    }

    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let env_path = exe_dir.join(".env");
            if !attempts.iter().any(|p| p == &env_path) {
                attempts.push(env_path);
            }
        }
    }

    for env_path in &attempts {
        eprintln!("Trying .env path: {}", env_path.display());
        if dotenv::from_path(env_path).is_ok() {
            eprintln!("Loaded .env from: {}", env_path.display());
            return (Some(env_path.clone()), attempts);
        }
    }

    (None, attempts)
}

fn main() -> std::io::Result<()> {

    let (_loaded_from, attempted_paths) = load_env_file();
    std::env::var("RUSIC_DB_PATH").unwrap_or_else(|_| {
        let attempted = attempted_paths
            .iter()
            .map(|p| format!("- {}", p.display()))
            .collect::<Vec<String>>()
            .join("\n");
        panic!(
            "RUSIC_DB_PATH not set. .env lookup attempts:\n{}",
            attempted
        )
    });

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
