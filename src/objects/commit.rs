use crate::objects::Hash;
use crate::objects::Object;
use chrono::offset::Local;
use chrono::DateTime;
use std::str::FromStr;

/// This object represents a version. It contains the root of the tree and
/// metadata about the author and committer.
pub struct Commit {
    tree: Hash,
    parents: Vec<Hash>,
    committer: (User, chrono::DateTime<Local>),
    author: (User, chrono::DateTime<Local>),
    message: String,
}

impl Commit {
    pub fn new(
        tree: Hash,
        parents: Vec<Hash>,
        user: User,
        date: DateTime<Local>,
        message: String,
    ) -> Commit {
        Commit {
            tree,
            parents,
            committer: (user.clone(), date),
            author: (user, date),
            message,
        }
    }
}

impl Object for Commit {
    fn dump(&self) -> Vec<u8> {
        let mut data = vec![];

        // Tree
        data.append(&mut format!("tree {}\n", self.tree.to_string()).into_bytes());

        // Parents
        for parent in self.parents.iter() {
            data.append(&mut format!("parent {}\n", parent.to_string()).into_bytes());
        }

        // Author
        let mut timezone = self.author.1.offset().to_string();
        timezone.remove(3);
        data.append(
            &mut format!(
                "author {} <{}> {} {}\n",
                self.author.0.name,
                self.author.0.email,
                self.author.1.timestamp(),
                timezone
            )
            .into_bytes(),
        );

        // Commiter
        let mut timezone = self.committer.1.offset().to_string();
        timezone.remove(3);
        data.append(
            &mut format!(
                "committer {} <{}> {} {}\n",
                self.committer.0.name,
                self.committer.0.email,
                self.committer.1.timestamp(),
                timezone
            )
            .into_bytes(),
        );

        // Message
        data.append(&mut format!("\n{}", self.message).into_bytes());

        // Add header
        let header = format!("commit {}\0", data.len());
        let mut res = vec![];
        res.reserve(data.len() + header.len());
        res.append(&mut header.into_bytes());
        res.append(&mut data);
        res
    }

    fn from(data: Vec<u8>) -> Box<Commit> {
        Box::new(Commit::new(
            Hash::from_str("e3095e3fb2e3cbc0dea81d961650feda7f6448f7").unwrap(),
            vec![Hash::from_str("f8ebe55b90a19ab7e5dea5ec51390948109623e5").unwrap()],
            User::new(
                String::from("Florian Amsallem"),
                String::from("florian.amsallem@epita.fr"),
            ),
            Local::now(),
            String::from("second: commit\n"),
        ))
    }
}

pub struct User {
    pub name: String,
    pub email: String,
}

impl User {
    pub fn new(name: String, email: String) -> User {
        User { name, email }
    }
}

impl Clone for User {
    fn clone(&self) -> User {
        User {
            name: self.name.clone(),
            email: self.email.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn commit_dump() {
        let mut commit = Commit::new(
            Hash::from_str("e3095e3fb2e3cbc0dea81d961650feda7f6448f7").unwrap(),
            vec![Hash::from_str("f8ebe55b90a19ab7e5dea5ec51390948109623e5").unwrap()],
            User::new(
                String::from("Florian Amsallem"),
                String::from("florian.amsallem@epita.fr"),
            ),
            Local::now(),
            String::from("second: commit\n"),
        );
        let dump = commit.dump();
        assert_eq!(dump.len(), 262);
    }
}
