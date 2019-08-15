use crate::objects::{Commit, Hash, Object};
use std::collections::HashMap;
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

/// This function return the list of branches and their associated commits hash
pub fn branches(repo_path: &PathBuf) -> HashMap<String, Hash> {
    let mut branches = HashMap::new();
    let path = repo_path.join("refs/heads");
    for file in fs::read_dir(path).expect("Can't read in heads directory") {
        let branch_name = file.unwrap().file_name().into_string().unwrap();
        let hash = resolve(
            repo_path,
            &format!("refs/heads/{}", branch_name).to_string(),
        );
        if let Ok(hash) = hash {
            branches.insert(branch_name, hash);
        }
    }
    branches
}

/// This function return the current branch name and his associated commits hash
pub fn current_branch(repo_path: &PathBuf) -> Option<(String, Hash)> {
    let path = repo_path.join("HEAD");
    let head = match get_head(repo_path) {
        Some(commit) => commit.hash(),
        None => return None,
    };
    match fs::read_to_string(path) {
        Ok(branch) => {
            let mut branch = branch[16..].to_string();
            branch.pop();
            Some((branch, head))
        }
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
