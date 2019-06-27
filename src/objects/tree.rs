use crate::objects::Hash;
use crate::objects::Object;

/// This enum contains all the entries in a Tree.
pub enum TreeEntry {
    File(String, Hash),
    Executable(String, Hash),
    Symlink(String, Hash),
    Directory(String, Hash),
}

/// This object carry trees and blobs. It represents the files in the
/// repository.
pub struct Tree {
    entries: Vec<TreeEntry>,
}

impl Tree {
    pub fn new() -> Tree {
        Tree { entries: vec![] }
    }

    pub fn add(&mut self, entry: TreeEntry) -> &mut Tree {
        self.entries.push(entry);
        self
    }

    pub fn add_file(&mut self, path: String, hash: Hash) -> &mut Tree {
        self.entries.push(TreeEntry::File(path, hash));
        self
    }

    pub fn add_executable(&mut self, path: String, hash: Hash) -> &mut Tree {
        self.entries.push(TreeEntry::Executable(path, hash));
        self
    }

    pub fn add_symlink(&mut self, path: String, hash: Hash) -> &mut Tree {
        self.entries.push(TreeEntry::Symlink(path, hash));
        self
    }

    pub fn add_directory(&mut self, path: String, hash: Hash) -> &mut Tree {
        self.entries.push(TreeEntry::Directory(path, hash));
        self
    }
}

impl Object for Tree {
    fn dump(&self) -> Vec<u8> {
        // Compute data for each entry
        let mut data = vec![];
        for entry in self.entries.iter() {
            let (mode, path, hash) = match entry {
                TreeEntry::File(path, hash) => ("100644", path, hash),
                TreeEntry::Executable(path, hash) => ("100755", path, hash),
                TreeEntry::Symlink(path, hash) => ("120000", path, hash),
                TreeEntry::Directory(path, hash) => ("40000", path, hash),
            };
            let entry = format!("{} {}\0", mode, path);
            data.append(&mut entry.into_bytes());
            data.extend_from_slice(&hash.bytes());
        }

        // Add header
        let header = format!("tree {}\0", data.len());
        let mut res = vec![];
        res.reserve(data.len() + header.len());
        res.append(&mut header.into_bytes());
        res.append(&mut data);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn tree_dump() {
        let mut tree = Tree::new();
        tree.add_file(
            String::from("lol"),
            Hash::from_str("63cd04a52f5c8cb95686081b000223e968ed74f4").unwrap(),
        );
        let dump = tree.dump();
        assert_eq!(dump.len(), 39);
        assert_eq!(std::str::from_utf8(&dump[0..8]).unwrap(), "tree 31\0");
        assert_eq!(dump[7], 0);
        assert_eq!(dump[38], 0xf4);
    }

    #[test]
    fn tree_dump_multiple() {
        let mut tree = Tree::new();
        tree.add_directory(
            String::from("dir"),
            Hash::from_str("828ed76b504d419d56d72df04c1bbb477ea69109").unwrap(),
        )
        .add_file(
            String::from("lol"),
            Hash::from_str("63cd04a52f5c8cb95686081b000223e968ed74f4").unwrap(),
        )
        .add_executable(
            String::from("run.sh"),
            Hash::from_str("5198cfd733f87f38ddfb400964c38c8ea238ea17").unwrap(),
        );
        let dump = tree.dump();
        assert_eq!(dump.len(), 103);
        assert_eq!(std::str::from_utf8(&dump[0..8]).unwrap(), "tree 95\0");
    }
}
