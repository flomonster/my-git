use crate::index::Index;
use crate::utils;
use clap::ArgMatches;
use glob::glob;
use std::error::Error;
use std::path::PathBuf;

pub fn run(args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let root = utils::find_root()?;
    let repo_path = utils::find_repo()?;

    let mut index = Index::load(&repo_path);

    for spec in args.values_of("PATHSPEC").unwrap() {
        if glob(spec)?.count() == 0 {
            index.remove(&PathBuf::from(spec), &root)?;
        } else {
            for entry in glob(spec)? {
                if let Ok(file) = entry {
                    index.add(&PathBuf::from(&file), &repo_path, &root)?;
                    index.remove(&PathBuf::from(&file), &root)?;
                }
            }
        }
    }
    index.save(&repo_path);

    Ok(())
}
