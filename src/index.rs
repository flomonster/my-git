use crate::objects::Hash;
use crate::objects::{Blob, Object};
use crate::utils;
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::{Error, ErrorKind};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::str::FromStr;

pub enum EntryType {
    File,
    Executable,
    Symlink,
}

impl fmt::Display for EntryType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EntryType::File => write!(f, "f"),
            EntryType::Executable => write!(f, "e"),
            EntryType::Symlink => write!(f, "s"),
        }
    }
}

impl From<&str> for EntryType {
    fn from(t: &str) -> Self {
        match t {
            "f" => EntryType::File,
            "e" => EntryType::Executable,
            "s" => EntryType::Symlink,
            t => panic!("Couldn't parse {} into an EntryType", t),
        }
    }
}

pub struct Index {
    pub entries: HashMap<String, (EntryType, Hash)>,
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
                    let entry_type = line.split(' ').skip(size - 2).next().unwrap();
                    let entry_type = EntryType::from(entry_type);
                    let path: Vec<&str> = line.split(' ').take(size - 2).collect();
                    let path = path.join(" ");
                    entries.insert(path, (entry_type, hash));
                }
                Err(e) => panic!(e),
            }
        }
        Index { entries }
    }

    /// Save the current index to the repository
    pub fn save(&self, repo_path: &PathBuf) {
        let mut dump = String::new();
        for (path, (entry_type, hash)) in self.entries.iter() {
            dump.push_str(format!("{} {} {}\n", path, entry_type, hash).as_str());
        }
        fs::write(repo_path.join("index"), dump).expect("Index writing failed");
    }

    /// Add a file to the index
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

        let metadata = fs::metadata(&file)?;
        let file: PathBuf = file.iter().skip(root.iter().count()).collect();
        let file_type = if metadata.file_type().is_symlink() {
            EntryType::Symlink
        } else if metadata.permissions().mode() & 1 == 1 {
            EntryType::Executable
        } else {
            EntryType::File
        };

        self.entries.insert(
            String::from(file.to_str().unwrap()),
            (file_type, blob.hash()),
        );
        Ok(())
    }

    /// Return whether the index contains or not the given path to file/directory
    /// NOTE: if path is the root directory this function always return true.
    pub fn contains(&self, path: &PathBuf) -> Result<bool, Box<Error>> {
        let root = utils::find_root()?;
        let real_path = fs::canonicalize(path).unwrap();

        // Special case when the index is empty it should return true
        if root == real_path {
            return Ok(true);
        }

        let index_path: PathBuf = real_path.iter().skip(root.iter().count()).collect();

        Ok(self
            .entries
            .iter()
            .any(|(entry_path, _)| PathBuf::from(entry_path).starts_with(&index_path)))
    }
}
