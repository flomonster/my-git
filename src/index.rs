use crate::objects::Hash;
use crate::objects::{Blob, Object};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use std::str::FromStr;

pub struct Index {
    entries: HashMap<String, Hash>,
}

impl Index {
    /// This function load the current index from the repository path.
    pub fn load(repo_path: &PathBuf) -> Index {
        let index = File::open(repo_path.join("index")).expect("No index found in the repository");
        let mut index = BufReader::new(index);
        let mut entries = HashMap::new();
        loop {
            let mut line = String::new();
            match index.read_line(&mut line) {
                Ok(0) => break,
                Ok(_) => {
                    let hash = &line.split(' ').last().expect("No hash found in the entry")[..40];
                    let hash = Hash::from_str(hash).unwrap();
                    let size = line.split(' ').count();
                    let path: String = line.split(' ').take(size - 1).collect();
                    entries.insert(path, hash);
                }
                Err(e) => panic!(e),
            }
        }
        Index { entries }
    }

    /// Save the current index to the repository
    pub fn save(&self, repo_path: &PathBuf) {
        let mut dump = String::new();
        for (path, hash) in self.entries.iter() {
            dump.push_str(format!("{} {}\n", path, hash).as_str());
        }
        fs::write(repo_path.join("index"), dump).expect("Index writing failed");
    }

    pub fn add(
        &mut self,
        file: &PathBuf,
        repo_path: &PathBuf,
        root: &PathBuf,
    ) -> Result<(), Error> {
        let file = &fs::canonicalize(file)?;
        if !file.is_file() {
            for file in fs::read_dir(file)? {
                self.add(&file?.path(), repo_path, root)?;
            }
            return Ok(());
        }
        let content = fs::read(file)?;
        let blob = Blob::new(content);
        blob.save(&repo_path);

        if !file.starts_with(root) {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!(
                    "fatal: {}: '{}' is outside repository",
                    file.to_str().unwrap(),
                    file.to_str().unwrap()
                ),
            ));
        }

        let file: PathBuf = file.iter().skip(root.iter().count()).collect();
        self.entries
            .insert(String::from(file.to_str().unwrap()), blob.hash());
        Ok(())
    }
}
