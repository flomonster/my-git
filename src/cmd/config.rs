extern crate yaml_rust;
use crate::utils;
use clap::ArgMatches;
use dirs::home_dir;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fs;
use std::path::PathBuf;
use yaml_rust::yaml::Hash;
use yaml_rust::{Yaml, YamlEmitter, YamlLoader};

pub fn run(args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    // Set the path to the local config file
    if let Some(value) = args.value_of("VALUE") {
        let key = args.value_of("NAME").unwrap();
        if args.is_present("global") {
            let mut config = Config::load_global()?;
            config.set(&String::from(key), &String::from(value))?;
            config.dump_global()?;
        } else {
            let mut config = Config::load_local()?;
            config.set(&String::from(key), &String::from(value))?;
            config.dump_local()?;
        }
    } else {
        let config = if args.is_present("global") {
            Config::load_global()?
        } else {
            Config::load()?
        };
        if let Some(val) = config.get(&String::from(args.value_of("NAME").unwrap()))? {
            println!("{}", val);
        }
    }
    Ok(())
}

pub struct Config {
    pub user: User,
}

impl Config {
    fn new() -> Config {
        Config { user: User::new() }
    }

    pub fn load() -> Result<Config, Box<dyn Error>> {
        let mut res = Config::new();

        // Apply global config
        let path = home_dir()
            .expect("fatal: $HOME not set")
            .join(".my_gitconfig");
        if !path.exists() {
            fs::File::create(&path)?;
        }
        res.apply_file(&path)?;

        // Apply local config
        if let Ok(path) = utils::find_repo() {
            let path = path.join("config");
            if !path.exists() {
                fs::File::create(&path)?;
            }
            res.apply_file(&path)?;
        }
        Ok(res)
    }

    pub fn load_local() -> Result<Config, Box<dyn Error>> {
        let path = utils::find_repo()?.join("config");
        if !path.exists() {
            fs::File::create(&path)?;
        }
        let mut res = Config::new();
        res.apply_file(&path)?;
        Ok(res)
    }

    pub fn load_global() -> Result<Config, Box<dyn Error>> {
        let path = home_dir()
            .expect("fatal: $HOME not set")
            .join(".my_gitconfig");
        if !path.exists() {
            fs::File::create(&path)?;
        }
        let mut res = Config::new();
        res.apply_file(&path)?;
        Ok(res)
    }

    fn apply_file(&mut self, path: &PathBuf) -> Result<(), Box<dyn Error>> {
        let config = fs::read_to_string(path)?;
        let config = YamlLoader::load_from_str(config.as_str())?;
        self.apply_config(&Yaml::Array(config));

        Ok(())
    }

    fn apply_config(&mut self, config: &Yaml) {
        match config {
            Yaml::Hash(hash) => {
                for (key, val) in hash.iter() {
                    if let Yaml::String(key) = key {
                        match key.as_str() {
                            "user" => self.user.apply_config(val),
                            _ => (),
                        }
                    }
                }
            }
            Yaml::Array(arr) => {
                for e in arr.iter() {
                    self.apply_config(e)
                }
            }
            _ => (),
        }
    }
    fn set(&mut self, key: &String, value: &String) -> Result<(), ConfigError> {
        let mut key = key.split(".");
        match key.next() {
            Some("user") => match key.next() {
                Some("name") => self.user.name = Some(value.clone()),
                Some("email") => self.user.email = Some(value.clone()),
                Some(key) => {
                    return Err(ConfigError::InvalidKey(
                        String::from("user"),
                        String::from(key),
                    ))
                }
                None => return Err(ConfigError::EmptyKey(String::from("user"))),
            },
            key => {
                return Err(ConfigError::InvalidKey(
                    String::from("config"),
                    String::from(key.unwrap()),
                ))
            }
        };
        Ok(())
    }

    fn get(&self, key: &String) -> Result<Option<String>, ConfigError> {
        let mut key = key.split(".");
        match key.next() {
            Some("user") => match key.next() {
                Some("name") => Ok(self.user.name.clone()),
                Some("email") => Ok(self.user.email.clone()),
                Some(key) => Err(ConfigError::InvalidKey(
                    String::from("user"),
                    String::from(key),
                )),
                None => Err(ConfigError::EmptyKey(String::from("user"))),
            },
            key => Err(ConfigError::InvalidKey(
                String::from("config"),
                String::from(key.unwrap()),
            )),
        }
    }

    pub fn dump_local(&self) -> Result<(), Box<dyn Error>> {
        let path = utils::find_repo()?.join("config");
        self.dump(path)?;
        Ok(())
    }

    pub fn dump_global(&self) -> Result<(), Box<dyn Error>> {
        let path = home_dir()
            .expect("fatal: $HOME not set")
            .join(".my_gitconfig");
        self.dump(path)?;
        Ok(())
    }

    pub fn dump(&self, path: PathBuf) -> Result<(), Box<dyn Error>> {
        let mut dump = Hash::new();

        // User
        let mut user = Hash::new();
        if let Some(name) = &self.user.name {
            user.insert(Yaml::from_str("name"), Yaml::from_str(name.as_str()));
        }
        if let Some(email) = &self.user.email {
            user.insert(Yaml::from_str("email"), Yaml::from_str(email.as_str()));
        }
        dump.insert(Yaml::from_str("user"), Yaml::Hash(user));

        // Dump
        let mut dump_str = String::new();
        let mut emitter = YamlEmitter::new(&mut dump_str);
        emitter.dump(&Yaml::Hash(dump)).unwrap();
        fs::write(path, dump_str)?;
        Ok(())
    }
}

pub struct User {
    pub name: Option<String>,
    pub email: Option<String>,
}

impl User {
    pub fn new() -> User {
        User {
            name: None,
            email: None,
        }
    }

    pub fn apply_config(&mut self, config: &Yaml) {
        if let Yaml::Hash(config) = config {
            for (key, val) in config.iter() {
                if let Yaml::String(val) = val {
                    match key {
                        Yaml::String(key) if key == "name" => self.name = Some(val.clone()),
                        Yaml::String(key) if key == "email" => self.email = Some(val.clone()),
                        _ => (),
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum ConfigError {
    MissingAuthor(String),
    InvalidKey(String, String),
    EmptyKey(String),
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigError::MissingAuthor(email) => write!(
                f,
                "*** Please tell me who you are.\n\nRun\n\n  \
                 git config --global user.email \"you@example.com\"\n  \
                 git config --global user.name \"Your Name\"\n\n\
                 to set your account's default identity.\n\
                 Omit --global to set the identity only in this repository.\n\n\
                 fatal: empty ident name (for <{}>) not allowed",
                email
            ),
            ConfigError::InvalidKey(section, key) => {
                write!(f, "error: {} does not contain a section: {}", section, key)
            }
            ConfigError::EmptyKey(section) => {
                write!(f, "error: you must specify a section for {}", section)
            }
        }
    }
}

impl Error for ConfigError {}
