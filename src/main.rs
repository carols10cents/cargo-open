#[macro_use]
extern crate clap;
extern crate cargo;

use clap::{Arg, App, AppSettings, SubCommand};
use cargo::core::SourceId;
use cargo::util::{hex, CargoResult};

use std::env;
use std::path::{Path, PathBuf};

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

    // Ok to use unwrap here because clap will handle argument errors
    let crate_name = matches.subcommand_matches("open").unwrap().value_of("CRATE").unwrap();

    match cargo_open(crate_name) {
        Ok(()) => {},
        Err(why) => panic!("{}", why),
    }
}

fn cargo_open(crate_name: &str) -> CargoResult<()> {
    let lock_path = "Cargo.lock";
    let lock_path = Path::new(&lock_path);
    let lock_path_buf = absolutize(lock_path.to_path_buf());
    let lock_path = lock_path_buf.as_path();

    let proj_dir = lock_path.parent().unwrap(); // TODO: check for None
    let src_id = SourceId::for_path(&proj_dir).unwrap();
    println!("src_id = {:?}", src_id);

    let resolved = cargo::ops::load_lockfile(&lock_path, &src_id).unwrap().expect("Lock file not found.");


    let pkgid = resolved.query(crate_name).ok().unwrap();


    println!("pkgid = {:?}", pkgid);
    println!("name = {}, version = {}", pkgid.name(), pkgid.version());

    let hash = hex::short_hash(pkgid.source_id());
    println!("hash = {}", hash);

    let ident = pkgid.source_id().url().host().unwrap().to_string();
    println!("ident = {}", ident);

    let part = format!("{}-{}", ident, hash);
    println!("part = {}", part);

    let dest = format!("{}-{}", pkgid.name(), pkgid.version());

    let cwd = env::current_dir().ok().unwrap();

    let cargo_home = env::var_os("CARGO_HOME").map(|home| {
        cwd.join(home)
    });
    let user_home = env::home_dir().map(|p| p.join(".cargo")).unwrap();
    let home_path = cargo_home.unwrap_or(user_home);


    let open_path = home_path.join("registry").join("src").join(&part).join(&dest);

    println!("open_path = {:?}", open_path);
    Ok(())

}

fn absolutize(pb: PathBuf) -> PathBuf {
    if pb.as_path().is_absolute() {
        pb
    } else {
        std::env::current_dir().unwrap().join(&pb.as_path()).clone()
    }
}
