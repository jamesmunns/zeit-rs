use clap::{App, SubCommand};
use pomodoro;

use std::io::prelude::*;

pub fn parse_cli() {
    let matches = App::new("zeit: A CLI time tracking tool")
        .author(crate_authors!())
        .version(crate_version!())
        .subcommand(SubCommand::with_name("init").about("Initialize Zeit Tracking"))
        .get_matches();

    match matches.subcommand_name() {
        Some("init") => {
            init();
        }
        _ => println!("Hallo, welt!"),
    }
}

fn init() {
    use std::env;
    use std::fs;
    use std::path::Path;
    use std::io::ErrorKind;

    println!("Initalizing Zeit...");

    let home = env::home_dir().expect("Home Directory Not Defined!");

    let working_dir = home.join(Path::new(".zeit"));

    println!("Creating working directory: {:?}", working_dir);

    match fs::create_dir(&working_dir) {
        Ok(_) => println!("Created successfully"),
        Err(err) => {
            match err.kind() {
                ErrorKind::AlreadyExists => {
                    println!("Working directory already exists.");
                }
                _ => {
                    panic!("Failed to create working dir! {:?}", err.kind());
                }
            }
        }
    }

    //
    // Create config file
    //
    let config_file = working_dir.join(Path::new("config.toml"));

    println!("Creating config file: {:?}", config_file);

    // TODO: Initialize with empty config?
    match fs::File::create(&config_file) {
        Ok(_) => println!("Created successfully"),
        Err(err) => {
            // NOTE: This doesn't make sense for file creation
            match err.kind() {
                ErrorKind::AlreadyExists => {
                    println!("Config file already exists.");
                }
                _ => {
                    panic!("Failed to create config file! {:?}", err.kind());
                }
            }
        }
    }

    //
    // Create history file
    //
    let history_file = working_dir.join(Path::new("history.toml"));

    println!("Creating history file: {:?}", history_file);

    match fs::File::create(&history_file) {
        Ok(mut hfile) => {
            let blank = pomodoro::Pomodoros::new()
                .to_string()
                .expect("How the hell did serializing an empty history object fail!?!");
            let _ = hfile.write_all(&blank.into_bytes()).expect("Failed to write to history file");
            println!("Created successfully");
        }
        Err(err) => {
            // NOTE: This doesn't make sense for file creation
            match err.kind() {
                ErrorKind::AlreadyExists => {
                    println!("History file already exists.");
                }
                _ => {
                    panic!("Failed to create history file! {:?}", err.kind());
                }
            }
        }
    }


}
