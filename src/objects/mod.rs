pub use blob::Blob;
pub use commit::Commit;
use sha1::{Digest, Sha1};
pub use tree::Tree;

mod blob;
mod commit;
mod tree;

type Hash = Digest;

/// This represents Git object as blob, tree and commit
pub trait Object {
    /// This function dump an object to his raw data
    fn dump(&self) -> Vec<u8>;

    /// This function allow object to be hashed
    fn hash(&self) -> Hash {
        Sha1::from(self.dump()).digest()
    }
}

#[cfg(test)]
mod tests {
    use super::Blob;
    use super::Tree;
    use super::*;
    use chrono::offset::TimeZone;
    use chrono::Local;
    use commit::User;
    use std::str::FromStr;

    #[test]
    fn blob_hash() {
        let blob = Blob::new(String::from("Hey").into_bytes());
        assert_eq!(
            blob.hash().to_string(),
            "63cd04a52f5c8cb95686081b000223e968ed74f4"
        );
    }

    #[test]
    fn tree_hash_simple() {
        let mut tree = Tree::new();
        tree.add_file(
            String::from("lol"),
            Hash::from_str("63cd04a52f5c8cb95686081b000223e968ed74f4").unwrap(),
        );
        assert_eq!(
            tree.hash().to_string(),
            "1953c52d154c2ae716190669a376235df8ac1cce"
        );
    }
    #[test]
    fn tree_hash_symlink() {
        let mut tree = Tree::new();
        tree.add_symlink(
            String::from("lol_link"),
            Hash::from_str("21c7de8be9398f4b356ffe7d75838fa166b4d5a6").unwrap(),
        );
        assert_eq!(
            tree.hash().to_string(),
            "828ed76b504d419d56d72df04c1bbb477ea69109"
        );
    }

    #[test]
    fn tree_hash_multiple() {
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
        assert_eq!(
            tree.hash().to_string(),
            "c9d0390d36023a52e95ca89ea06bbb2be7ab58ec"
        );
    }

    #[test]
    fn commit_dump() {
        let mut commit = Commit::new(
            Hash::from_str("07f9cb6648d474785a4e08afe408633b1cf04d50").unwrap(),
            vec![Hash::from_str("bed08c07a4fb5d3be29024eac3b7efd7d8729e46").unwrap()],
            User::new(
                String::from("Florian Amsallem"),
                String::from("florian.amsallem@epita.fr"),
            ),
            Local.timestamp(1561665499, 0),
            String::from("second: commit\n"),
        );
        println!("{}", std::str::from_utf8(&commit.dump()[..]).unwrap());
        assert_eq!(
            commit.hash().to_string(),
            "3f07efedb395e8e29412149b5d596f163af24ad4"
        );
    }
}
