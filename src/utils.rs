use std::env;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

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
