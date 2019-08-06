use crate::objects::{Commit, Hash, Object};
use std::env;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use std::str::FromStr;

/// This function return the path to the repository. If not in a my-git repository then return an
/// error.
pub fn find_repo() -> Result<PathBuf, Error> {
    let path = find_root()?;

    Ok(path.join(".my_git"))
}

/// This function return the path to root of the project repository. If not in an
/// my-git repository then return an error.
pub fn find_root() -> Result<PathBuf, Error> {
    let mut path = env::current_dir()?;

    while !path.join(".my_git").exists() {
        path = match path.parent() {
            Some(path) => path.to_path_buf(),
            None => {
                return Err(Error::new(
                    ErrorKind::NotFound,
                    "fatal: not a git repository (or any of the parent directories): .my_git",
                ))
            }
        }
    }

    Ok(path)
}

/// This function resolve a reference
pub fn ref_resolve(repo_path: &PathBuf, ref_: &String) -> Result<Hash, Error> {
    let path = repo_path.join(ref_);
    let mut content = fs::read_to_string(path)?;
    // Remove trailing newline
    content.pop();
    if content.starts_with("ref: ") {
        ref_resolve(repo_path, &content[5..].to_string())
    } else {
        Ok(Hash::from_str(&content[..]).unwrap())
    }
}

/// This function return the HEAD commit
pub fn get_head(repo_path: &PathBuf) -> Option<Commit> {
    match ref_resolve(repo_path, &String::from("HEAD")) {
        Ok(hash) => Some(*Commit::load(repo_path, hash)),
        _ => None,
    }
}
