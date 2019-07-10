use crate::objects::Hash;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::str::FromStr;

pub struct Index {
    list: Vec<(String, Hash)>,
}

impl Index {
    /// This function load the current index from the repository path.
    pub fn load(repo_path: &PathBuf) -> Index {
        let index = File::open(repo_path.join("index")).expect("No index found in the repository");
        let mut index = BufReader::new(index);
        let mut list = vec![];
        loop {
            let mut line = String::new();
            match index.read_line(&mut line) {
                Ok(_) => {
                    let hash = line.split(' ').last().expect("No hash found in the entry");
                    let hash = Hash::from_str(hash).unwrap();
                    let path = line.split(' ');
                    let size = path.size_hint().0;
                    let path: String = path.take(size - 1).collect();
                    list.push((path, hash));
                }
                Err(_) => break,
            }
        }
        Index { list }
    }

    /// Save the current index to the repository
    pub fn save(&self, repo_path: &PathBuf) {
        let mut dump = String::new();
        for entry in self.list.iter() {
            dump.push_str(format!("{} {}", entry.0, entry.1).as_str());
        }
        fs::write(repo_path.join("index"), dump).expect("Index writing failed");
    }
}
