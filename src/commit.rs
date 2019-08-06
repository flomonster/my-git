use crate::config::{Config, ConfigError};
use crate::index::Index;
use crate::objects::{Commit, Object, Tree};
use crate::utils;
use clap::ArgMatches;
use std::error::Error;

pub fn run(args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    // TODO: Check that files has been added

    // Load config
    let config = Config::load()?;
    let user_name = config.user.name;
    let user_email = match config.user.email {
        Some(email) => email,
        None => String::new(),
    };

    // Return an error in case of an empty configuration
    match &user_name {
        Some(name) if name != &String::new() => (),
        _ => return Err(Box::new(ConfigError::MissingAuthor(user_email))),
    }

    // Create tree object
    let repo_path = utils::find_repo()?;
    let index = Index::load(&repo_path);
    let tree = Tree::from(&index);

    // Save tree
    tree.save(&repo_path);

    // TODO: Get head commit (parent)
    let parent = vec![];

    // Create commit object
    let message = String::from(args.value_of("msg").unwrap());
    let commit = Commit::create(&tree, parent, user_name.unwrap(), user_email, message);

    // Save commit object
    commit.save(&repo_path);

    // Change HEAD

    Ok(())
}
