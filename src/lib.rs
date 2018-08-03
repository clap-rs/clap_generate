// Copyright ⓒ 2015-2018 Kevin B. Knapp
//
// `clap_generate` is distributed under the terms of both the MIT license and the Apache License
// (Version 2.0).
// See the [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) files in this repository
// for more information.

//! Generates shell completion scripts for [`clap`](https://github.com/kbknapp/clap-rs) based CLIs

#![crate_type = "lib"]
#![doc(html_root_url = "https://docs.rs/clap_generate/0.0.1")]
#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    unused_import_braces,
    unused_allocation
)]
#![cfg_attr(
    not(any(feature = "lints", feature = "nightly")),
    forbid(unstable_features)
)]
#![cfg_attr(feature = "lints", feature(plugin))]
#![cfg_attr(feature = "lints", plugin(clippy))]
#![cfg_attr(feature = "lints", deny(warnings))]
#![cfg_attr(feature = "lints", allow(cyclomatic_complexity))]
#![cfg_attr(feature = "lints", allow(doc_markdown))]
#![cfg_attr(feature = "lints", allow(explicit_iter_loop))]

extern crate clap as _clap;
extern crate man;

// Re-Export of clap
mod clap {
    pub use _clap::*;
}

const INTERNAL_ERROR_MSG: &'static str = "Fatal internal error. Please consider filing a bug \
                                          report at https://github.com/clap-rs/clap_derive/issues";

#[macro_use]
mod macros;
mod manual;
mod shells;

use shells::{ComplGen, Shell};

use std::ffi::OsString;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub use manual::gen_manuals;

/// Generate a completions file for a specified shell at compile time.
///
/// **NOTE:** to generate the file at compile time you must use a `build.rs` "Build Script"
///
/// # Examples
///
/// The following example generates a bash completion script via a `build.rs` script. In this
/// simple example, we'll demo a very small application with only a single subcommand and two
/// args. Real applications could be many multiple levels deep in subcommands, and have tens or
/// potentially hundreds of arguments.
///
/// First, it helps if we separate out our `App` definition into a separate file. Whether you
/// do this as a function, or bare App definition is a matter of personal preference.
///
/// ```
/// // src/cli.rs
///
/// use clap::{App, Arg, SubCommand};
///
/// pub fn build_cli() -> App<'static, 'static> {
///     App::new("compl")
///         .about("Tests completions")
///         .arg(Arg::with_name("file")
///             .help("some input file"))
///         .subcommand(SubCommand::with_name("test")
///             .about("tests things")
///             .arg(Arg::with_name("case")
///                 .long("case")
///                 .takes_value(true)
///                 .help("the case to test")))
/// }
/// ```
///
/// In our regular code, we can simply call this `build_cli()` function, then call
/// `get_matches()`, or any of the other normal methods directly after. For example:
///
/// ```ignore
/// // src/main.rs
///
/// mod cli;
///
/// fn main() {
///     let m = cli::build_cli().get_matches();
///
///     // normal logic continues...
/// }
/// ```
///
/// Next, we set up our `Cargo.toml` to use a `build.rs` build script.
///
/// ```toml
/// # Cargo.toml
/// build = "build.rs"
///
/// [build-dependencies]
/// clap = "2.23"
/// ```
///
/// Next, we place a `build.rs` in our project root.
///
/// ```ignore
/// extern crate clap;
///
/// use clap::Shell;
///
/// include!("src/cli.rs");
///
/// fn main() {
///     let outdir = match env::var_os("OUT_DIR") {
///         None => return,
///         Some(outdir) => outdir,
///     };
///     let mut app = build_cli();
///     app.gen_completions("myapp",      // We need to specify the bin name manually
///                         Shell::Bash,  // Then say which shell to build completions for
///                         outdir);      // Then say where write the completions to
/// }
/// ```
/// Now, once we compile there will be a `{bin_name}.bash` file in the directory.
/// Assuming we compiled with debug mode, it would be somewhere similar to
/// `<project>/target/debug/build/myapp-<hash>/out/myapp.bash`.
///
/// Fish shell completions will use the file format `{bin_name}.fish`
pub fn gen_completions<T: Into<OsString>, S: Into<String>>(
    app: &mut clap::App,
    bin_name: S,
    for_shell: Shell,
    out_dir: T,
) {
    use std::error::Error;

    let out_dir = PathBuf::from(out_dir.into());
    let name = &*app.bin_name.as_ref().unwrap().clone();
    let file_name = match for_shell {
        Shell::Bash => format!("{}.bash", name),
        Shell::Fish => format!("{}.fish", name),
        Shell::Zsh => format!("_{}", name),
        Shell::PowerShell => format!("_{}.ps1", name),
        Shell::Elvish => format!("{}.elv", name),
        _ => panic!("Unsupported shell type for completion generation"),
    };

    let mut file = match File::create(out_dir.join(file_name)) {
        Err(why) => panic!("couldn't create completion file: {}", why.description()),
        Ok(file) => file,
    };
    generate_completions_to(app, bin_name.into(), for_shell, &mut file)
}

/// Generate a completions file for a specified shell at runtime.  Until `cargo install` can
/// install extra files like a completion script, this may be used e.g. in a command that
/// outputs the contents of the completion script, to be redirected into a file by the user.
///
/// # Examples
///
/// Assuming a separate `cli.rs` like the [example above](./struct.App.html#method.gen_completions),
/// we can let users generate a completion script using a command:
///
/// ```ignore
/// // src/main.rs
///
/// mod cli;
/// use std::io;
///
/// fn main() {
///     let matches = cli::build_cli().get_matches();
///
///     if matches.is_present("generate-bash-completions") {
///         cli::build_cli().gen_completions_to("myapp", Shell::Bash, &mut io::stdout());
///     }
///
///     // normal logic continues...
/// }
///
/// ```
///
/// Usage:
///
/// ```shell
/// $ myapp generate-bash-completions > /usr/share/bash-completion/completions/myapp.bash
/// ```
pub fn generate_completions_to<W: Write, S: Into<String>>(
    app: &mut clap::App,
    bin_name: S,
    for_shell: Shell,
    buf: &mut W,
) {
    app.bin_name = Some(bin_name.into());
    if !app.is_set(clap::AppSettings::Propagated) {
        app._build(clap::Propagation::Full);
        app._build_bin_names();
    }

    ComplGen::new(app).generate(for_shell, buf)
}
