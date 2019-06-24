use clap::ArgMatches;

/// This dispatch the subcommand used and
pub fn run(matches: ArgMatches) {
    match matches.subcommand() {
        ("init", Some(_matches)) => println!("Init was matched"),
        ("add", Some(_matches)) => println!("Add was matched"),
        ("commit", Some(_matches)) => println!("Commit was matched"),
        _ => panic!("No subcommand matched"),
    };
}
