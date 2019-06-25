#[macro_use]
extern crate clap;
use clap::App;
use std::process::exit;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let mut app = App::from_yaml(yaml);
    if let Err(e) = my_git::run(&mut app) {
        eprintln!("{}", e);
        exit(2);
    }
}
