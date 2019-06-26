//! # My Git
//!
//! `my_git` is a simple implementation of the versionning tool git.
use clap::App;
use std::error::Error;

mod init;
mod objects;

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
        ("add", Some(_matches)) => Ok(()),
        ("commit", Some(_matches)) => Ok(()),
        (_, None) => match app.print_help() {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        },
        _ => panic!("The used subcommand is not handle"),
    }
}
