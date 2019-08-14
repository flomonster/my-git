//! # My Git
//!
//! `my_git` is a simple implementation of the versionning tool git.
use clap::App;
use std::error::Error;

pub mod cmd;
pub mod objects;

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
        ("add", Some(matches)) => cmd::add::run(matches),
        ("branch", Some(matches)) => cmd::branch::run(matches),
        ("commit", Some(matches)) => cmd::commit::run(matches),
        ("config", Some(matches)) => cmd::config::run(matches),
        ("init", Some(matches)) => cmd::init::run(matches),
        ("log", Some(matches)) => cmd::log::run(matches),
        ("status", Some(matches)) => cmd::status::run(matches),
        (_, None) => {
            app.print_help()?;
            println!();
            Ok(())
        }
        _ => panic!("The used subcommand is not handle"),
    }
}
