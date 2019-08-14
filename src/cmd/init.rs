use clap::ArgMatches;
use std::env;
use std::error::Error;
use std::fs;
use std::fs::{DirBuilder, File};

/// This funciton initialize the git repository. It returns an error if
/// something went wrong like a lake of rights
///
/// # Arguments
///
/// * `args` - The subcommand arguments
pub fn run(args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    // Compute the path of the git folder
    let repo_path = match args.value_of("directory") {
        Some(path) => {
            let mut p = std::path::PathBuf::new();
            p.push(path);
            p
        }
        None => env::current_dir()?,
    };
    let git_path = repo_path.join(".my_git");
    let reinitialized = git_path.exists();

    // Build dirs
    let mut dir_builder = DirBuilder::new();
    dir_builder
        .recursive(true)
        .create(git_path.join("objects/info"))?;
    dir_builder.create(git_path.join("refs/heads"))?;
    dir_builder.create(git_path.join("refs/tags"))?;

    if !git_path.join("HEAD").is_file() {
        fs::write(git_path.join("HEAD"), "ref: refs/heads/master\n")?;
    }
    if !git_path.join("index").is_file() {
        File::create(git_path.join("index"))?;
    }

    if !args.is_present("quiet") {
        match reinitialized {
            true => println!(
                "Reinitialized existing My-Git repository in {}",
                git_path.to_str().unwrap()
            ),
            false => println!(
                "Initialized empty My-Git repository in {}",
                git_path.to_str().unwrap()
            ),
        }
    }

    Ok(())
}
