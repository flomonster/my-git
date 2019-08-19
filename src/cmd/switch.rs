use crate::cmd::branch;
use crate::objects::{Commit, Hash, Object};
use crate::{refs, utils};
use clap::ArgMatches;
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

pub fn switch_branch(
    repo_path: &PathBuf,
    branch: &String,
    branches: &HashMap<String, Hash>,
) -> Result<(), Box<dyn Error>> {
    // Find branch commit
    let commit = match branches.iter().find(|(b, _)| *b == branch) {
        Some((_, commit)) => Commit::load(repo_path, *commit),
        _ => return Err(Box::new(branch::ErrorBranch::NoBranchFound(branch.clone()))),
    };

    // TODO: Checkout to the commit

    // Update HEAD
    refs::update(
        repo_path,
        &String::from("HEAD"),
        &format!("ref: refs/heads/{}", branch),
        false,
    )?;
    Ok(())
}

pub fn run(args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let repo_path = utils::find_repo()?;
    let branch = args.value_of("BRANCH").unwrap().to_string();
    let branches = refs::branches(&repo_path);

    // Create branch
    if args.is_present("create") || args.is_present("force-create") {
        branch::create_branch(
            &repo_path,
            &branch,
            args.is_present("force-create"),
            &branches,
        )?;
    }

    switch_branch(&repo_path, &branch, &branches)?;

    Ok(())
}
