use crate::objects::{Commit, Hash, Object};
use std::fs;
use std::io::Error;
use std::path::PathBuf;
use std::str::FromStr;

/// Dereference a ref (not guaranteed value existence)
fn deref(repo_path: &PathBuf, ref_: &String) -> Result<String, Error> {
    let path = repo_path.join(ref_);

    // End of the ref
    if !path.exists() {
        return Ok(ref_.clone());
    }

    let mut content = fs::read_to_string(path)?;
    // Remove trailing newline
    content.pop();
    if content.starts_with("ref: ") {
        deref(repo_path, &content[5..].to_string())
    } else {
        Ok(ref_.clone())
    }
}

/// This function resolve a reference giving the hash of the object behind it.
pub fn resolve(repo_path: &PathBuf, ref_: &String) -> Result<Hash, Error> {
    let ref_ = deref(repo_path, ref_)?;
    let path = repo_path.join(ref_);
    let mut content = fs::read_to_string(path)?;
    content.pop();
    Ok(Hash::from_str(&content[..]).unwrap())
}

/// This function return the HEAD commit
pub fn get_head(repo_path: &PathBuf) -> Option<Commit> {
    match resolve(repo_path, &String::from("HEAD")) {
        Ok(hash) => Some(*Commit::load(repo_path, hash)),
        _ => None,
    }
}

/// This function update/create the object stored in a ref safely.
/// It derefence the ref before update the value of it.
pub fn update(repo_path: &PathBuf, ref_: &String, value: &String) -> Result<(), Error> {
    let ref_ = deref(repo_path, ref_)?;
    let path = repo_path.join(ref_);
    fs::write(path, format!("{}\n", value))?;
    Ok(())
}
