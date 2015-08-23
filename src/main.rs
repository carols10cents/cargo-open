#[macro_use]
extern crate clap;

use clap::{Arg, App, AppSettings, SubCommand};

fn main() {
    let matches = App::new("cargo-open")
        .about("A third-party cargo extension to allow you to open a dependent crate in your $EDITOR")
        .version(&crate_version!()[..])
        // We have to lie about our binary name since this will be a third party
        // subcommand for cargo, this trick learned from cargo-outdated
        .bin_name("cargo")
        // We use a subcommand because parsed after `cargo` is sent to the third party plugin
        // which will be interpreted as a subcommand/positional arg by clap
        .subcommand(SubCommand::with_name("open")
            .about("A third-party cargo extension to allow you to open a dependent crate in your $EDITOR")
            .arg(Arg::with_name("CRATE")
              .help("The name of the crate you would like to open")
              .required(true)
              .index(1)))
        .settings(&[AppSettings::SubcommandRequired])
        .get_matches();

    println!("Using crate name: {}", matches.subcommand_matches("open").unwrap().value_of("CRATE").unwrap());
}
