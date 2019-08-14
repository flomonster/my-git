use crate::index::Index;
use crate::utils;
use clap::ArgMatches;
use glob::glob;
use std::error::Error;
use std::fmt;
use std::path::PathBuf;

pub fn run(args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let root = utils::find_root()?;
    let repo_path = utils::find_repo()?;

    let mut index = Index::load(&repo_path);
    let ignored = utils::ignored(&root)?;
    let mut fails = vec![];
    let force = args.is_present("force");

    for spec in args.values_of("PATHSPEC").unwrap() {
        if glob(spec)?.count() == 0 {
            index.remove(&PathBuf::from(spec), &root)?;
        } else {
            for entry in glob(spec)? {
                if let Ok(file) = entry {
                    fails.append(&mut index.add(
                        &PathBuf::from(&file),
                        &repo_path,
                        &root,
                        force,
                        &ignored,
                    )?);
                    index.remove(&PathBuf::from(&file), &root)?;
                }
            }
        }
    }
    index.save(&repo_path);

    if !fails.is_empty() {
        return Err(Box::new(FaileAddIgnored::new(fails)));
    }

    Ok(())
}

#[derive(Debug)]
struct FaileAddIgnored {
    ignored: Vec<PathBuf>,
}

impl FaileAddIgnored {
    pub fn new(ignored: Vec<PathBuf>) -> Self {
        FaileAddIgnored { ignored }
    }
}

impl fmt::Display for FaileAddIgnored {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ignored = String::new();
        for ignore in self.ignored.iter() {
            ignored.push_str(ignore.to_str().unwrap());
            ignored.push('\n');
        }
        write!(
            f,
            "The following paths are ignored by your .my_gitignore file:\n{}\
             Use -f if you really want to add them.",
            ignored
        )
    }
}
impl Error for FaileAddIgnored {}
