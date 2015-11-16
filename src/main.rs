#[macro_use]
extern crate clap;
extern crate cargo;

use clap::{Arg, App, AppSettings, SubCommand};
use cargo::core::SourceId;
use cargo::util::{hex, human, CargoResult};

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::ffi::OsString;

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

    let crate_dir = match cargo_dir(crate_name) {
        Ok(path) => path,
        Err(why) => panic!("{}", why),
    };

    let editor = match cargo_editor() {
        Ok(editor) => editor,
        Err(why) => panic!("{}", why),
    };

    match Command::new(editor).arg(crate_dir).status() {
        Ok(_)    => return,
        Err(why) => panic!("{}", why),
    };
}

pub fn cargo_editor() -> CargoResult<OsString> {
    env::var_os("CARGO_EDITOR").or_else(||
        env::var_os("VISUAL").or_else(||
            env::var_os("EDITOR")
        )
    ).ok_or(human("Cannot find an editor. Please specify one of $CARGO_EDITOR, $VISUAL, or $EDITOR and try again."))
}

fn cargo_dir(crate_name: &str) -> CargoResult<PathBuf> {
    // Load the current project's dependencies from its Cargo.lock.
    let lock_path     = "Cargo.lock";
    let lock_path     = Path::new(&lock_path);
    let lock_path_buf = absolutize(lock_path.to_path_buf());
    let lock_path     = lock_path_buf.as_path();
    let proj_dir      = lock_path.parent().unwrap();
    let src_id        = SourceId::for_path(&proj_dir).unwrap();
    let resolved      = try!(cargo::ops::load_lockfile(&lock_path, &src_id)).unwrap();

    // Look up the crate we're interested in that the current project is using
    let pkgid = try!(resolved.query(crate_name));

    // Build registry_source_path the same way cargo's Config does as of
    // https://github.com/rust-lang/cargo/blob/176b5c17906cf43445888e83a4031e411f56e7dc/src/cargo/util/config.rs#L35-L80
    let cwd                  = try!(env::current_dir());
    let cargo_home           = env::var_os("CARGO_HOME").map(|home| cwd.join(home));
    let user_home            = env::home_dir().map(|p| p.join(".cargo")).unwrap();
    let home_path            = cargo_home.unwrap_or(user_home);
    let registry_source_path = home_path.join("registry").join("src");

    // Build src_path the same way cargo's RegistrySource does as of
    // https://github.com/rust-lang/cargo/blob/176b5c17906cf43445888e83a4031e411f56e7dc/src/cargo/sources/registry.rs#L232-L238
    let hash     = hex::short_hash(pkgid.source_id());
    let ident    = pkgid.source_id().url().host().unwrap().to_string();
    let part     = format!("{}-{}", ident, hash);
    let src_path = registry_source_path.join(&part);

    // Build the crate's unpacked destination directory the same way cargo's RegistrySource does as
    // of https://github.com/rust-lang/cargo/blob/176b5c17906cf43445888e83a4031e411f56e7dc/src/cargo/sources/registry.rs#L357-L358
    let dest = format!("{}-{}", pkgid.name(), pkgid.version());

    Ok(src_path.join(&dest))
}

fn absolutize(pb: PathBuf) -> PathBuf {
    if pb.as_path().is_absolute() {
        pb
    } else {
        std::env::current_dir().unwrap().join(&pb.as_path()).clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    fn setup() {
        // Reset all env vars to isolate each test
        env::remove_var("CARGO_EDITOR");
        env::remove_var("VISUAL");
        env::remove_var("EDITOR");
    }

    #[test]
    fn check_env_editor() {
        setup();
        let editor = "some_editor";
        env::set_var("EDITOR", editor);
        assert_eq!(editor, cargo_editor().unwrap().to_str().unwrap());
    }

    #[test]
    fn check_env_cargo_editor() {
        setup();
        let cargo_editor_val = "some_cargo_editor";
        env::set_var("CARGO_EDITOR", cargo_editor_val);
        assert_eq!(cargo_editor_val, cargo_editor().unwrap().to_str().unwrap());
    }

    #[test]
    fn check_env_visual() {
        setup();
        let visual = "some_visual";
        env::set_var("VISUAL", visual);
        assert_eq!(visual, cargo_editor().unwrap().to_str().unwrap());
    }

    #[test]
    fn prefer_cargo_editor_over_visual() {
        setup();
        let cargo_editor_val = "some_cargo_editor";
        let visual = "some_visual";
        env::set_var("CARGO_EDITOR", cargo_editor_val);
        env::set_var("VISUAL", visual);
        assert_eq!(cargo_editor_val, cargo_editor().unwrap().to_str().unwrap());
    }

    #[test]
    fn prefer_visual_over_editor() {
        setup();
        let visual = "some_visual";
        let editor = "some_editor";
        env::set_var("VISUAL", visual);
        env::set_var("EDITOR", editor);
        assert_eq!(visual, cargo_editor().unwrap().to_str().unwrap());
    }

    #[test]
    fn prefer_cargo_editor_over_editor() {
        setup();
        let cargo_editor_val = "some_cargo_editor";
        let editor = "some_editor";
        env::set_var("CARGO_EDITOR", cargo_editor_val);
        env::set_var("EDITOR", editor);
        assert_eq!(cargo_editor_val, cargo_editor().unwrap().to_str().unwrap());
    }

    #[test]
    fn prefer_cargo_editor_over_visual_and_editor() {
        setup();
        let cargo_editor_val = "some_cargo_editor";
        let visual = "some_visual";
        let editor = "some_editor";
        env::set_var("CARGO_EDITOR", cargo_editor_val);
        env::set_var("VISUAL", visual);
        env::set_var("EDITOR", editor);
        assert_eq!(cargo_editor_val, cargo_editor().unwrap().to_str().unwrap());
    }

    #[test]
    #[should_panic(expected = "Cannot find an editor. Please specify one of $CARGO_EDITOR, $VISUAL, or $EDITOR and try again.")]
    fn error_on_no_env_editor() {
        setup();
        cargo_editor().unwrap();
    }
}
