use std::env;
use std::fs;
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

/// This function return relative the path to `dest`.
pub fn find_relative_path(dest: &PathBuf) -> PathBuf {
    let mut path = env::current_dir().unwrap();
    let dest = fs::canonicalize(dest).unwrap();
    let mut res = PathBuf::new();

    // Case same directory
    if dest == path {
        res.push(".");
        return res;
    }

    // Ascending directories
    while !dest.starts_with(&path) {
        res.push("..");
        path.pop();
    }

    // Descending tree
    while dest != path {
        let filename = dest
            .iter()
            .skip(path.iter().count())
            .next()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        res.push(&filename);
        path.push(&filename);
    }

    res
}
