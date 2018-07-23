clap_generate
====

[![Crates.io](https://img.shields.io/crates/v/clap_generate.svg)](https://crates.io/crates/clap_generate) [![Crates.io](https://img.shields.io/crates/d/clap_generate.svg)](https://crates.io/crates/clap_generate) [![license](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/clap-rs/clap_generate/blob/master/LICENSE-MIT)[![license](http://img.shields.io/badge/license-APACHE2-blue.svg)](https://github.com/clap-rs/clap_generate/blob/master/LICENSE-APACHE) [![Join the chat at https://gitter.im/clap-rs/clap_generate](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/clap-rs/clap_generate?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

Linux: [![Build Status](https://travis-ci.org/clap-rs/clap_generate.svg?branch=master)](https://travis-ci.org/clap-rs/clap_generate)
Windows: [![Build status](https://ci.appveyor.com/api/projects/status/ejg8c33dn31nhv36/branch/master?svg=true)](https://ci.appveyor.com/project/clap-rs/clap_generate/branch/master)


* [documentation](https://docs.rs/clap_generate/)
* [website](https://clap.rs/)

Table of Contents
=================

- [clap_generate](#clapgenerate)
- [Table of Contents](#table-of-contents)
  - [What's New](#whats-new)
  - [About](#about)
  - [FAQ](#faq)
  - [Features](#features)
  - [Quick Example](#quick-example)
  - [Usage](#usage)
    - [Optional Dependencies / Features](#optional-dependencies--features)
      - [Features enabled by default](#features-enabled-by-default)
      - [Opt-in features](#opt-in-features)
    - [Dependencies Tree](#dependencies-tree)
    - [More Information](#more-information)
  - [How to Contribute](#how-to-contribute)
    - [Compatibility Policy](#compatibility-policy)
      - [Warning about '~' Dependencies](#warning-about--dependencies)
      - [Minimum Version of Rust](#minimum-version-of-rust)
      - [Breaking Changes](#breaking-changes)
  - [License](#license)
  - [Related Crates](#related-crates)
  - [Recent Breaking Changes](#recent-breaking-changes)
    - [Deprecations](#deprecations)

Created by [gh-md-toc](https://github.com/ekalinin/github-markdown-toc)

## What's New


For full details, see [CHANGELOG.md](https://github.com/clap-rs/clap_generate/blob/master/CHANGELOG.md)

## About


## FAQ


## Features

## Quick Example

## Usage

### Optional Dependencies / Features

#### Features enabled by default

#### Opt-in features

### Dependencies Tree

### More Information

## How to Contribute

Details on how to contribute can be found in the [CONTRIBUTING.md](.github/CONTRIBUTING.md) file.

### Compatibility Policy

Because `clap_generate` takes SemVer and compatibility seriously, this is the official policy regarding breaking changes and minimum required versions of Rust.

`clap_generate` will pin the minimum required version of Rust to the CI builds. Bumping the minimum version of Rust is considered a minor breaking change, meaning *at a minimum* the minor version of `clap_generate` will be bumped.

In order to keep from being surprised of breaking changes, it is **highly** recommended to use the `~major.minor.patch` style in your `Cargo.toml` only if you wish to target a version of Rust that is *older* than current stable minus two releases:

```toml
[dependencies]
clap_generate = "~3.0.0"
```

This will cause *only* the patch version to be updated upon a `cargo update` call, and therefore cannot break due to new features, or bumped minimum versions of Rust.

#### Warning about '~' Dependencies

Using `~` can cause issues in certain circumstances.

From @alexcrichton:

Right now Cargo's version resolution is pretty naive, it's just a brute-force search of the solution space, returning the first resolvable graph. This also means that it currently won't terminate until it proves there is not possible resolvable graph. This leads to situations where workspaces with multiple binaries, for example, have two different dependencies such as:

```toml,no_sync

# In one Cargo.toml
[dependencies]
clap_generate = "~3.0.0"

# In another Cargo.toml
[dependencies]
clap_generate = "3.0.0"
```

This is inherently an unresolvable crate graph in Cargo right now. Cargo requires there's only one major version of a crate, and being in the same workspace these two crates must share a version. This is impossible in this location, though, as these version constraints cannot be met.

#### Minimum Version of Rust

`clap_generate` will officially support current stable Rust, minus two releases, but may work with prior releases as well. For example, current stable Rust at the time of this writing is 1.27.0, meaning `clap_generate` is guaranteed to compile with 1.25.0 and beyond (although in practice `clap_generate` is much more conservative).

At the 1.28.0 stable release, `clap_generate` will be guaranteed to compile with 1.26.0 and beyond, etc.

Upon bumping the minimum version of Rust (assuming it's within the stable-2 range), it *must* be clearly annotated in the `CHANGELOG.md`

#### Breaking Changes

`clap_generate` takes a similar policy to Rust and will bump the major version number upon breaking changes with only the following exceptions:

 * The breaking change is to fix a security concern
 * The breaking change is to be fixing a bug (i.e. relying on a bug as a feature)
 * The breaking change is a feature isn't used in the wild, or all users of said feature have given approval *prior* to the change

## License

`clap_generate` is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See the [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) files in this repository for more information.

## Related Crates

There are several excellent crates which can be used with `clap`, I recommend checking them all out! If you've got a crate that would be a good fit to be used with `clap` open an issue and let me know, I'd love to add it!

* [`clap`](https://github.com/kbknapp/clap-rs) - The main `clap` crate!
* [`clap_derive`](https://github.com/clap-rs/clap_derive) - This crate allows you to define a struct, and build a CLI from it! No more "stringly typed" and it uses `clap` behind the scenes 
* [`assert_cli`](https://github.com/assert-rs/assert_cli) - This crate allows you test your CLIs in a very intuitive and functional way!

## Recent Breaking Changes


### Deprecations

