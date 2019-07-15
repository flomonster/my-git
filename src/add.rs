use crate::index::Index;
use crate::utils;
use clap::ArgMatches;
use std::error::Error;
use std::path::PathBuf;

pub fn run(args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let root = utils::find_root()?;
    let repo_path = utils::find_repo()?;

    let mut index = Index::load(&repo_path);

    for file in args.values_of("PATH_SPEC").unwrap() {
        index.add(&PathBuf::from(file), &repo_path, &root)?;
    }
    index.save(&repo_path);

    Ok(())
}
