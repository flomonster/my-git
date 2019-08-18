use crate::objects::Hash;
use crate::objects::Object;
use crate::{refs, utils};
use clap::ArgMatches;
use colored::Colorize;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::path::PathBuf;

fn display(repo_path: &PathBuf, branches: &HashMap<String, Hash>) {
    let current_branch = match refs::current_branch(&repo_path) {
        Some((branch, _)) => branch,
        _ => String::new(),
    };

    for (branch, _) in branches {
        if current_branch == *branch {
            println!("* {}", branch.green());
        } else {
            println!("  {}", branch);
        }
    }
}

pub fn create_branch(
    repo_path: &PathBuf,
    branch: &String,
    branches: &HashMap<String, Hash>,
) -> Result<(), Box<dyn Error>> {
    // Check the branch name validity
    let re = Regex::new(r"^[+\w\])(&!@$%'`]+(\.?/?[+\-\w\])(&!@$%'`]+)*$").unwrap();
    if branch == "HEAD" || !re.is_match(branch) {
        return Err(Box::new(ErrorBranch::InvalidName(branch.to_string())));
    }
    // Check non existance of the branch
    if branches.iter().any(|(b, _)| b == branch) {
        return Err(Box::new(ErrorBranch::AlreadyExists(branch.to_string())));
    }
    let head = refs::get_head(&repo_path);
    match head {
        Some(head) => {
            refs::update(
                &repo_path,
                &format!("refs/heads/{}", branch).to_string(),
                &head.hash().to_string(),
                false,
            )?;
            Ok(())
        }
        None => Err(Box::new(ErrorBranch::NoCommitYet)),
    }
}
pub fn delete_branch(
    repo_path: &PathBuf,
    branch: &String,
    branches: &HashMap<String, Hash>,
) -> Result<(), Box<dyn Error>> {
    if !branches.iter().any(|(b, _)| b == branch) {
        return Err(Box::new(ErrorBranch::NoBranchFound(branch.clone())));
    }
    let current_branch = match refs::current_branch(&repo_path) {
        Some((branch, _)) => branch,
        _ => return Err(Box::new(ErrorBranch::NoCommitYet)),
    };
    if current_branch == *branch {
        return Err(Box::new(ErrorBranch::DeleteCurrentBranch(current_branch)));
    }
    let ref_path = repo_path.join(format!("refs/heads/{}", branch));
    refs::remove_ref(&ref_path)?;

    Ok(())
}

pub fn run(args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let repo_path = utils::find_repo()?;
    let branches = refs::branches(&repo_path);

    match (args.is_present("BRANCHNAME"), args.is_present("delete")) {
        (true, true) => {
            let branch = args.value_of("BRANCHNAME").unwrap();
            delete_branch(&repo_path, &branch.to_string(), &branches)?
        }
        (false, false) => display(&repo_path, &branches),
        (true, false) => {
            let branch = args.value_of("BRANCHNAME").unwrap();
            create_branch(&repo_path, &branch.to_string(), &branches)?
        }
        _ => return Err(Box::new(ErrorBranch::BranchNameRequired)),
    }

    Ok(())
}

#[derive(Debug)]
enum ErrorBranch {
    InvalidName(String),
    AlreadyExists(String),
    NoCommitYet,
    BranchNameRequired,
    NoBranchFound(String),
    DeleteCurrentBranch(String),
}

impl fmt::Display for ErrorBranch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorBranch::InvalidName(branch) => {
                write!(f, "fatal: '{}' is not a valid branch name.", branch)
            }
            ErrorBranch::AlreadyExists(branch) => {
                write!(f, "fatal: A branch named '{}' already exists.", branch)
            }
            ErrorBranch::NoCommitYet => write!(f, "fatal: can't create a branch without commit"),
            ErrorBranch::BranchNameRequired => write!(f, "fatal: branch name required"),
            ErrorBranch::NoBranchFound(branch) => {
                write!(f, "error: branch '{}' not found.", branch)
            }
            ErrorBranch::DeleteCurrentBranch(branch) => write!(
                f,
                "error: cannot delete branch '{}'. You should checkout first",
                branch
            ),
        }
    }
}

impl Error for ErrorBranch {}
