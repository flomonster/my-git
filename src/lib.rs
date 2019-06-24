//! # My Git
//!
//! `my_git` is a simple implementation of the versionning tool git.
use clap::App;

/// This dispatch the main subcommand
///
/// # Arguments
///
/// * `app` - The program option app.
///
/// # Panics
///
/// When a valid subcommand is used but not handled.
pub fn run(app: &mut App) {
    let matches = app.clone().get_matches();

    match matches.subcommand() {
        ("init", Some(_matches)) => println!("Init was matched"),
        ("add", Some(_matches)) => println!("Add was matched"),
        ("commit", Some(_matches)) => println!("Commit was matched"),
        (_, None) => app.print_help().expect("An error occur printing the help"),
        _ => panic!("The used subcommand is not handle"),
    };
}
