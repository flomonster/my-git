use crate::{refs, utils};
use clap::ArgMatches;
use colored::Colorize;
use std::error::Error;
use std::path::PathBuf;

fn display(repo_path: &PathBuf) {
    let branches = refs::branches(&repo_path);
    let current_branch = match refs::current_branch(&repo_path) {
        Some((branch, _)) => branch,
        _ => String::new(),
    };

    for (branch, _) in branches {
        if current_branch == branch {
            println!("* {}", branch.green());
        } else {
            println!("  {}", branch);
        }
    }
}

pub fn run(_args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let repo_path = utils::find_repo()?;

    // TODO: Check branchname arg and delete option

    display(&repo_path);

    Ok(())
}
