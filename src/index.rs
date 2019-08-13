use crate::objects::Hash;
use crate::objects::{Blob, Object};
use crate::utils;
use glob::Pattern;
use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::{Error, ErrorKind};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Eq, PartialEq, Clone)]
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

    /// Return the type of an existing file
    pub fn get_file_type(path: &PathBuf) -> EntryType {
        let metadata = fs::metadata(&path).unwrap();
        if metadata.file_type().is_symlink() {
            EntryType::Symlink
        } else if metadata.permissions().mode() & 1 == 1 {
            EntryType::Executable
        } else {
            EntryType::File
        }
    }

    /// Add an existing file/directory to the index
    pub fn add(
        &mut self,
        path: &PathBuf,
        repo_path: &PathBuf,
        root: &PathBuf,
        force: bool,
        ignored: &Vec<Pattern>,
    ) -> Result<(Vec<PathBuf>), Box<Error>> {
        let file = &fs::canonicalize(&path)?;

        // Check file is inside the repository
        if !file.starts_with(root) {
            return Err(Box::new(Error::new(
                ErrorKind::InvalidInput,
                format!(
                    "fatal: {}: '{}' is outside repository",
                    file.to_str().unwrap(),
                    file.to_str().unwrap()
                ),
            )));
        }

        // Check ignored if not a force add
        if !force && utils::is_ignored(file, ignored)? && !self.contains(file)? {
            return Ok(vec![path.clone()]);
        }

        // Call recursively in case of dir
        if file.is_dir() {
            let mut failed = vec![];
            for file in fs::read_dir(file)? {
                failed.append(&mut self.add(&file?.path(), repo_path, root, force, ignored)?);
            }
            return Ok(failed);
        }

        // Compute and save blob
        let content = fs::read(file)?;
        let blob = Blob::new(content);
        blob.save(&repo_path);

        // Compute type
        let file_type = Self::get_file_type(&file);

        // Add to the index
        let file: PathBuf = file.iter().skip(root.iter().count()).collect();
        self.entries.insert(
            String::from(file.to_str().unwrap()),
            (file_type, blob.hash()),
        );
        Ok(vec![])
    }

    /// Remove file/directory from the index
    /// TODO: Handle globing
    pub fn remove(&mut self, file: &PathBuf, root: &PathBuf) -> Result<(), Error> {
        // Get absolute path of file
        let full_path = if file.is_absolute() {
            file.clone()
        } else {
            let mut full_path = env::current_dir().unwrap();
            for filename in file.iter() {
                if filename == "." {
                    continue;
                }
                if filename == ".." {
                    full_path.pop();
                } else {
                    full_path.push(filename);
                }
            }
            full_path
        };

        // Check file is inside the repository
        if !full_path.starts_with(root) {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!(
                    "fatal: {}: '{}' is outside repository",
                    file.to_str().unwrap(),
                    file.to_str().unwrap()
                ),
            ));
        }

        let index_path: PathBuf = full_path.iter().skip(root.iter().count()).collect();
        for (entry_path, _) in self.entries.clone().iter() {
            let path = PathBuf::from(entry_path);
            if path.starts_with(&index_path) {
                let path = root.join(path);
                if !path.exists() {
                    self.entries.remove(entry_path);
                }
            }
        }

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
