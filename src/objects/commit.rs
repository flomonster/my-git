use crate::objects::Hash;
use crate::objects::{Object, Tree};
use chrono::offset::{FixedOffset, Local, TimeZone};
use chrono::DateTime;
use colored::Colorize;
use std::fmt;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::str::FromStr;

/// This object represents a version. It contains the root of the tree and
/// metadata about the author and committer.
pub struct Commit {
    pub tree: Hash,
    pub parents: Vec<Hash>,
    pub committer: (User, DateTime<FixedOffset>),
    pub author: (User, DateTime<FixedOffset>),
    pub message: String,
}

impl Commit {
    pub fn new(
        tree: Hash,
        parents: Vec<Hash>,
        user: User,
        date: DateTime<FixedOffset>,
        message: String,
    ) -> Self {
        Commit {
            tree,
            parents,
            committer: (user.clone(), date),
            author: (user, date),
            message,
        }
    }

    pub fn create(
        tree: &Tree,
        parents: Vec<Self>,
        user_name: String,
        user_email: String,
        message: String,
    ) -> Self {
        let parents: Vec<Hash> = parents.iter().map(|commit| commit.hash()).collect();
        let user = User::new(user_name, user_email);
        let date = DateTime::<FixedOffset>::from(Local::now());
        Self::new(tree.hash(), parents, user, date, message)
    }

    fn default() -> Commit {
        Commit::new(
            Hash::from_str("0000000000000000000000000000000000000000").unwrap(),
            vec![],
            User::new(String::from(""), String::from("")),
            FixedOffset::east(0).timestamp_millis(0),
            String::from(""),
        )
    }

    fn parse_user_date(data: &str) -> (User, DateTime<FixedOffset>) {
        let mut splitted = data.split_whitespace();
        let email = splitted.find(|&e| e.starts_with("<")).unwrap();
        let date = splitted.collect::<Vec<&str>>().join(" ");
        let date = FixedOffset::east(0)
            .datetime_from_str(date.as_str(), "%s %z")
            .unwrap();
        let name = data
            .split_whitespace()
            .take_while(|&e| !e.starts_with("<"))
            .collect::<Vec<&str>>()
            .join(" ");

        (
            User::new(name, String::from(&email[1..email.len() - 1])),
            date,
        )
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

    fn from(mut reader: BufReader<fs::File>) -> Box<Commit> {
        let mut buff = vec![];
        reader.read_until(0, &mut buff).unwrap();
        assert!(std::str::from_utf8(&buff).unwrap().starts_with("commit "));

        let mut res = Commit::default();
        let mut buff = String::new();
        while let Ok(_) = reader.read_line(&mut buff) {
            if buff == "\n" {
                break;
            }
            if buff.starts_with("tree ") {
                let buff: Vec<&str> = buff.split(' ').collect();
                res.tree = Hash::from_str(&buff[1][..40]).unwrap();
            } else if buff.starts_with("parent ") {
                let buff: Vec<&str> = buff.split(' ').collect();
                res.parents.push(Hash::from_str(&buff[1][..40]).unwrap());
            } else if buff.starts_with("author ") {
                (res.author) = Commit::parse_user_date(&buff[7..]);
            } else if buff.starts_with("committer ") {
                (res.committer) = Commit::parse_user_date(&buff[10..]);
            } else {
                panic!("Unexpected content in commit object");
            }

            buff.clear();
        }
        let mut buff = vec![];
        reader.read_to_end(&mut buff).unwrap();
        res.message = std::str::from_utf8(&buff).unwrap().to_string();
        Box::new(res)
    }
}

impl fmt::Display for Commit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: Handle refs/branches
        let commit = format!("commit {}", self.hash());
        let (user, date) = &self.author;
        let date = date.format("%a %b %e %T %Y");
        write!(
            f,
            "{}\nAuthor: {} <{}>\nDate:   {}\n\n    {}\n",
            commit.yellow(),
            user.name,
            user.email,
            date,
            self.message
        )
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
        let commit = Commit::new(
            Hash::from_str("e3095e3fb2e3cbc0dea81d961650feda7f6448f7").unwrap(),
            vec![Hash::from_str("f8ebe55b90a19ab7e5dea5ec51390948109623e5").unwrap()],
            User::new(
                String::from("Florian Amsallem"),
                String::from("florian.amsallem@epita.fr"),
            ),
            FixedOffset::east(7200).timestamp(1561665499, 0),
            String::from("second: commit\n"),
        );
        let dump = commit.dump();
        assert_eq!(dump.len(), 262);
    }
}
