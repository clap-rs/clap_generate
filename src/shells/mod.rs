mod bash;
mod elvish;
mod fish;
mod powershell;
mod zsh;

// Std
use std::io::Write;

// Internal
use self::bash::BashGen;
use self::elvish::ElvishGen;
use self::fish::FishGen;
use self::powershell::PowerShellGen;
use self::zsh::ZshGen;
use clap::App;

pub struct ComplGen<'a, 'b>(&'b App<'a, 'b>)
where
    'a: 'b;

impl<'a, 'b> ComplGen<'a, 'b> {
    pub fn new(app: &'b App<'a, 'b>) -> Self { ComplGen(app) }

    pub fn generate<W: Write>(&self, for_shell: Shell, buf: &mut W) {
        match for_shell {
            Shell::Bash => BashGen::new(self.0).generate_to(buf),
            Shell::Fish => FishGen::new(self.0).generate_to(buf),
            Shell::Zsh => ZshGen::new(self.0).generate_to(buf),
            Shell::PowerShell => PowerShellGen::new(self.0).generate_to(buf),
            Shell::Elvish => ElvishGen::new(self.0).generate_to(buf),
            _ => panic!("Unsupported shell type for generating completions"),
        }
    }
}

// Gets all subcommands including child subcommands in the form of 'name' where the name
// is a single word (i.e. "install")  of the path to said subcommand (i.e.
// "rustup toolchain install")
//
// Also note, aliases are treated as their own subcommands but duplicates of whatever they're
// aliasing.
pub fn all_subcommand_names(p: &App) -> Vec<String> {
    debugln!("all_subcommand_names;");
    let mut subcmds: Vec<_> = subcommands_of(p)
        .iter()
        .map(|&(ref n, _)| n.clone())
        .collect();
    for sc_v in subcommands!(p).map(|s| all_subcommand_names(&s)) {
        subcmds.extend(sc_v);
    }
    subcmds.sort();
    subcmds.dedup();
    subcmds
}

// Gets all subcommands including child subcommands in the form of ('name', 'bin_name') where the name
// is a single word (i.e. "install") of the path and full bin_name of said subcommand (i.e.
// "rustup toolchain install")
//
// Also note, aliases are treated as their own subcommands but duplicates of whatever they're
// aliasing.
pub fn all_subcommands(p: &App) -> Vec<(String, String)> {
    debugln!("all_subcommands;");
    let mut subcmds: Vec<_> = subcommands_of(p);
    for sc_v in subcommands!(p).map(|s| all_subcommands(&s)) {
        subcmds.extend(sc_v);
    }
    subcmds
}

// Gets all subcommands exlcuding child subcommands in the form of (name, bin_name) where the name
// is a single word (i.e. "install") and the bin_name is a space deliniated list of the path to said
// subcommand (i.e. "rustup toolchain install")
//
// Also note, aliases are treated as their own subcommands but duplicates of whatever they're
// aliasing.
pub fn subcommands_of(p: &App) -> Vec<(String, String)> {
    debugln!(
        "subcommands_of: name={}, bin_name={}",
        p.name,
        p.bin_name.as_ref().unwrap()
    );
    let mut subcmds = vec![];

    debugln!(
        "subcommands_of: Has subcommands...{:?}",
        p.has_subcommands()
    );
    if !p.has_subcommands() {
        let mut ret = vec![];
        debugln!("subcommands_of: Looking for aliases...");
        if let Some(ref aliases) = p.aliases {
            for &(n, _) in aliases {
                debugln!("subcommands_of:iter:iter: Found alias...{}", n);
                let mut als_bin_name: Vec<_> = p.bin_name.as_ref().unwrap().split(' ').collect();
                als_bin_name.push(n);
                let old = als_bin_name.len() - 2;
                als_bin_name.swap_remove(old);
                ret.push((n.to_owned(), als_bin_name.join(" ")));
            }
        }
        return ret;
    }
    for sc in subcommands!(p) {
        debugln!(
            "subcommands_of:iter: name={}, bin_name={}",
            sc.name,
            sc.bin_name.as_ref().unwrap()
        );

        debugln!("subcommands_of:iter: Looking for aliases...");
        if let Some(ref aliases) = sc.aliases {
            for &(n, _) in aliases {
                debugln!("subcommands_of:iter:iter: Found alias...{}", n);
                let mut als_bin_name: Vec<_> = p.bin_name.as_ref().unwrap().split(' ').collect();
                als_bin_name.push(n);
                let old = als_bin_name.len() - 2;
                als_bin_name.swap_remove(old);
                subcmds.push((n.to_owned(), als_bin_name.join(" ")));
            }
        }
        subcmds.push((sc.name.clone(), sc.bin_name.as_ref().unwrap().clone()));
    }
    subcmds
}

pub fn get_all_subcommand_paths(p: &App, first: bool) -> Vec<String> {
    debugln!("get_all_subcommand_paths;");
    let mut subcmds = vec![];
    if !p.has_subcommands() {
        if !first {
            let name = &*p.name;
            let path = p.bin_name.as_ref().unwrap().clone().replace(" ", "__");
            let mut ret = vec![path.clone()];
            if let Some(ref aliases) = p.aliases {
                for &(n, _) in aliases {
                    ret.push(path.replace(name, n));
                }
            }
            return ret;
        }
        return vec![];
    }
    for sc in subcommands!(p) {
        let name = &*sc.name;
        let path = sc.bin_name.as_ref().unwrap().clone().replace(" ", "__");
        subcmds.push(path.clone());
        if let Some(ref aliases) = sc.aliases {
            for &(n, _) in aliases {
                subcmds.push(path.replace(name, n));
            }
        }
    }
    for sc_v in subcommands!(p).map(|s| get_all_subcommand_paths(&s, false)) {
        subcmds.extend(sc_v);
    }
    subcmds
}
#[allow(unused_imports)]
use std::ascii::AsciiExt;
use std::fmt;
use std::str::FromStr;

/// Describes which shell to produce a completions file for
#[cfg_attr(feature = "lints", allow(enum_variant_names))]
#[derive(Debug, Copy, Clone)]
pub enum Shell {
    /// Generates a .bash completion file for the Bourne Again SHell (BASH)
    Bash,
    /// Generates a .fish completion file for the Friendly Interactive SHell (fish)
    Fish,
    /// Generates a completion file for the Z SHell (ZSH)
    Zsh,
    /// Generates a completion file for PowerShell
    PowerShell,
    /// Generates a completion file for Elvish
    Elvish,
    #[doc(hidden)]
    __Nonexhaustive,
}

impl Shell {
    /// A list of possible variants in `&'static str` form
    pub fn variants() -> [&'static str; 5] { ["zsh", "bash", "fish", "powershell", "elvish"] }
}

impl FromStr for Shell {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ZSH" | _ if s.eq_ignore_ascii_case("zsh") => Ok(Shell::Zsh),
            "FISH" | _ if s.eq_ignore_ascii_case("fish") => Ok(Shell::Fish),
            "BASH" | _ if s.eq_ignore_ascii_case("bash") => Ok(Shell::Bash),
            "POWERSHELL" | _ if s.eq_ignore_ascii_case("powershell") => Ok(Shell::PowerShell),
            "ELVISH" | _ if s.eq_ignore_ascii_case("elvish") => Ok(Shell::Elvish),
            _ => Err(String::from(
                "[valid values: bash, fish, zsh, powershell, elvish]",
            )),
        }
    }
}

impl fmt::Display for Shell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Shell::Bash => write!(f, "BASH"),
            Shell::Fish => write!(f, "FISH"),
            Shell::Zsh => write!(f, "ZSH"),
            Shell::PowerShell => write!(f, "POWERSHELL"),
            Shell::Elvish => write!(f, "ELVISH"),
            _ => panic!("Unsupported shell type for completion generation"),
        }
    }
}
