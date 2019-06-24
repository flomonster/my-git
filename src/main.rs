#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let mut app = App::from_yaml(yaml);
    my_git::run(&mut app);
}
