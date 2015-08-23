# `cargo open`

A third-party cargo extension to allow you to open a dependent crate in your $EDITOR. Heavily inspired by `bundle open`!

# Compiling

I'm assuming you already have Rust and cargo set up.

Clone this repository and go into the created directory:

    git clone https://github.com/carols10cents/cargo-open.git
    cd cargo-open

And compile a release version:

    cargo build --release

You should now have an executable in `[starting directory]/cargo-open/target/release/cargo-open`.

# Installing and Using

Compile the code as shown in the previous section, then put the `cargo-open` executable in your PATH.

My favorite way of doing this is I have a pre-existing directory in `~/bin` that contains little scripts of mine, that dir is added to my PATH in my `.bashrc` so that it's always available, and then I symlink the release version from where it exists to that directory:

    ln -s [starting directory]/cargo-open/target/release/cargo-open ~/bin/

Once you've done that, because of the way cargo is set up to use third party extensions, in any other Rust project of yours, you should be able to run:

    cargo open [some crate you're using]

and that crate will open in your desired editor.

`cargo open` determines your desired editor by first checking `$CARGO_EDITOR`, then `$VISUAL`, then `$EDITOR`. It will fail with a hopefully-helpful error message if none of these are set.

# Contributing

Pull requests, bug reports, and feature requests are all welcome! <3 <3 <3

If you'd like to work on your own version of the code, fork this repo and follow the Compiling steps above except with your fork.

One weird thing if you're running the binary directly instead of through the `cargo` plugin system is that clap doesn't think you're using a subcommand. If you try, you'll get:

    $ ./target/release/cargo-open whatever
    error: Found argument 'whatever', but cargo wasn't expecting any

    USAGE:
            cargo <SUBCOMMAND>

    For more information try --help

To get around this, either follow the Installation and Usage instructions above and always use `cargo open whatever` or re-specify `open` as the subcommand:

    ./target/release/cargo-open open whatever

# TODO

If you'd like to help with any of these and they're not clear, please ask! They're written mostly for my own benefit.

* Check for the 3 env vars in order
* Add tests
  * 3 env vars, no env vars
  * different kinds of crate installation -- *, version, github, local path, local override
  * crate not yet downloaded
  * dependencies of dependencies
  * multirust

# License

`cargo open` is primarily distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See LICENSE-APACHE and LICENSE-MIT for details.