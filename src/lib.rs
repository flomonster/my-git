//! # My Git
//!
//! `my_git` is a simple implementation of the versionning tool git.
use clap::App;
use std::error::Error;

mod add;
mod config;
mod init;
mod objects;

pub mod index;
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
        ("commit", Some(_matches)) => Ok(()),
        ("config", Some(matches)) => config::run(matches),
        (_, None) => {
            app.print_help()?;
            println!();
            Ok(())
        }
        _ => panic!("The used subcommand is not handle"),
    }
}
