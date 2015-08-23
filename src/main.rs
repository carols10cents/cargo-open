#[macro_use]
extern crate clap;

use clap::{Arg, App};

fn main() {
    let matches = App::new("cargo-open")
                         .about("A third-party cargo extension to allow you to open a dependent crate in your $EDITOR")
                         .version(&crate_version!()[..])
                         .arg(Arg::with_name("CRATE")
                              .help("The name of the crate you would like to open")
                              .required(true)
                              .index(1))
                         .get_matches();
    println!("Using crate name: {}", matches.value_of("CRATE").unwrap());
}
