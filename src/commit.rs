use crate::config::{Config, ConfigError};
use crate::index::Index;
use crate::objects::Tree;
use crate::utils;
use clap::ArgMatches;
use std::error::Error;

pub fn run(_args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    // TODO: Check that files has been added

    // Load config
    let config = Config::load()?;
    let user_name = config.user.name;
    let user_email = config.user.email;

    // Return an error in case of an empty configuration
    match &user_name {
        Some(name) if name != &String::new() => (),
        _ => return Err(Box::new(ConfigError::MissingAuthor(user_email))),
    }

    // Create tree object
    let repo_path = utils::find_repo()?;
    let index = Index::load(&repo_path);
    let tree = Tree::from(&index);
    tree.save(&repo_path);

    // Create commit object
    Ok(())
}
