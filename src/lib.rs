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

// Re-Export of clap
mod clap {
    pub use _clap::*;
}

const INTERNAL_ERROR_MSG: &'static str = "Fatal internal error. Please consider filing a bug \
                                          report at https://github.com/clap-rs/clap_derive/issues";

#[macro_use]
mod macros;
mod shells;
