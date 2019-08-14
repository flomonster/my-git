use crate::cmd::config::{Config, ConfigError};
use crate::index::Index;
use crate::objects::{Commit, Object, Tree};
use crate::refs;
use crate::utils;
use clap::ArgMatches;
use std::error::Error;
use std::fmt;

pub fn run(args: &ArgMatches) -> Result<(), Box<dyn Error>> {
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

    // Get head commit (parent)
    let mut parent = vec![];
    if let Some(commit) = refs::get_head(&repo_path) {
        parent.push(commit);
    }

    // Nothing to commit
    if (parent.is_empty() && tree.entries.is_empty())
        || (!parent.is_empty() && tree.hash() == parent[0].hash())
    {
        return Err(Box::new(NothingToCommit {}));
    }

    // Create commit object
    let message = String::from(args.value_of("msg").unwrap());
    let commit = Commit::create(&tree, parent, user_name.unwrap(), user_email, message);

    // Save commit object
    commit.save(&repo_path);

    // Update HEAD
    refs::update(
        &repo_path,
        &String::from("HEAD"),
        &commit.hash().to_string(),
    )
    .expect("fatal: error while updating HEAD ref");

    Ok(())
}

#[derive(Debug)]
struct NothingToCommit {}
impl fmt::Display for NothingToCommit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "nothing to commit")
    }
}
impl Error for NothingToCommit {}
