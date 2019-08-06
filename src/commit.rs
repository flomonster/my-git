use crate::index::Index;
use crate::objects::Tree;
use crate::utils;
use clap::ArgMatches;
use std::error::Error;

pub fn run(_args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    // TODO: Check that files has been added

    // Create tree object
    let repo_path = utils::find_repo()?;
    let index = Index::load(&repo_path);
    let tree = Tree::from(&index);
    tree.save(&repo_path);

    // Create commit object
    Ok(())
}
