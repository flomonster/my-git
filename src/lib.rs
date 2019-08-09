//! # My Git
//!
//! `my_git` is a simple implementation of the versionning tool git.
use clap::App;
use std::error::Error;

mod add;
mod commit;
mod config;
mod init;
mod log;
mod objects;
mod status;

pub mod index;
pub mod refs;
pub mod utils;

/// This dispatch the main subcommand and return an error if something went
/// wrong
///
/// # Arguments
///
/// * `app` - The program option app.
///
/// # Panics
///
/// When a valid subcommand is used but not handled.
pub fn run(app: &mut App) -> Result<(), Box<dyn Error>> {
    let matches = app.clone().get_matches();

    match matches.subcommand() {
        ("init", Some(matches)) => init::run(matches),
        ("add", Some(matches)) => add::run(matches),
        ("commit", Some(matches)) => commit::run(matches),
        ("config", Some(matches)) => config::run(matches),
        ("status", Some(matches)) => status::run(matches),
        ("log", Some(matches)) => log::run(matches),
        (_, None) => {
            app.print_help()?;
            println!();
            Ok(())
        }
        _ => panic!("The used subcommand is not handle"),
    }
}
