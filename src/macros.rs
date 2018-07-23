macro_rules! w {
    ($buf:expr, $to_w:expr) => {
        match $buf.write_all($to_w) {
            Ok(..) => (),
            Err(..) => panic!("Failed to write to completions file"),
        }
    };
}

macro_rules! get_zsh_arg_conflicts {
    ($app:expr, $arg:ident, $msg:ident) => {
        if let Some(ref conf_vec) = $arg.blacklist {
            let mut v = vec![];
            for arg_name in conf_vec {
                let arg = find!($app, arg_name).expect($msg);
                if let Some(s) = arg.short {
                    v.push(format!("-{}", s));
                }
                if let Some(l) = arg.long {
                    v.push(format!("--{}", l));
                }
            }
            v.join(" ")
        } else {
            String::new()
        }
    };
}

#[cfg(feature = "debug")]
#[cfg_attr(feature = "debug", macro_use)]
#[cfg_attr(feature = "debug", allow(unused_macros))]
mod debug_macros {
    macro_rules! debugln {
        ($fmt:expr) => (println!(concat!("DEBUG:clap:", $fmt)));
        ($fmt:expr, $($arg:tt)*) => (println!(concat!("DEBUG:clap:",$fmt), $($arg)*));
    }
    macro_rules! sdebugln {
        ($fmt:expr) => (println!($fmt));
        ($fmt:expr, $($arg:tt)*) => (println!($fmt, $($arg)*));
    }
    macro_rules! debug {
        ($fmt:expr) => (print!(concat!("DEBUG:clap:", $fmt)));
        ($fmt:expr, $($arg:tt)*) => (print!(concat!("DEBUG:clap:",$fmt), $($arg)*));
    }
    macro_rules! sdebug {
        ($fmt:expr) => (print!($fmt));
        ($fmt:expr, $($arg:tt)*) => (print!($fmt, $($arg)*));
    }
}

#[cfg(not(feature = "debug"))]
#[cfg_attr(not(feature = "debug"), macro_use)]
mod debug_macros {
    macro_rules! debugln {
        ($fmt:expr) => {};
        ($fmt:expr, $($arg:tt)*) => {};
    }
    macro_rules! sdebugln {
        ($fmt:expr) => {};
        ($fmt:expr, $($arg:tt)*) => {};
    }
    macro_rules! debug {
        ($fmt:expr) => {};
        ($fmt:expr, $($arg:tt)*) => {};
    }
}

macro_rules! args {
    ($app:expr, $how:ident) => {
        $app.args.$how()
    };
    ($app:expr) => {
        args!($app, iter)
    };
}

macro_rules! args_mut {
    ($app:expr) => {
        args!($app, iter_mut)
    };
}

macro_rules! flags {
    ($app:expr, $how:ident) => {
        $app.args
            .$how()
            .filter(|a| !a.settings.is_set(::clap::ArgSettings::TakesValue))
            .filter(|a| a.short.is_some() || a.long.is_some())
            .filter(|a| !a.help_heading.is_some())
    };
    ($app:expr) => {
        flags!($app, iter)
    };
}

#[allow(unused_macros)]
macro_rules! flags_mut {
    ($app:expr) => {
        flags!($app, iter_mut)
    };
}

macro_rules! opts {
    ($app:expr, $how:ident) => {
        $app.args
            .$how()
            .filter(|a| a.settings.is_set(::clap::ArgSettings::TakesValue))
            .filter(|a| a.short.is_some() || a.long.is_some())
            .filter(|a| !a.help_heading.is_some())
    };
    ($app:expr) => {
        opts!($app, iter)
    };
}

#[allow(unused_macros)]
macro_rules! opts_mut {
    ($app:expr) => {
        opts!($app, iter_mut)
    };
}

macro_rules! positionals {
    ($app:expr, $how:ident) => {
        $app.args
            .$how()
            .filter(|a| !a.help_heading.is_some())
            .filter(|a| !(a.short.is_some() || a.long.is_some()))
    };
    ($app:expr) => {
        positionals!($app, iter)
    };
}

#[allow(unused_macros)]
macro_rules! positionals_mut {
    ($app:expr) => {
        positionals!($app, iter_mut)
    };
}

#[allow(unused_macros)]
macro_rules! custom_headings {
    ($app:expr, $how:ident) => {
        $app.args.$how().filter(|a| (a.help_heading.is_some()))
    };
    ($app:expr) => {
        custom_headings!($app, iter)
    };
}

#[allow(unused_macros)]
macro_rules! custom_headings_mut {
    ($app:expr) => {
        custom_headings!($app, iter_mut)
    };
}

macro_rules! subcommands {
    ($app:expr, $how:ident) => {
        $app.subcommands.$how()
    };
    ($app:expr) => {
        subcommands!($app, iter)
    };
}

// Finds an arg by name
macro_rules! find {
    ($app:expr, $name:expr, $what:ident) => {
        $what!($app).find(|a| &a.name == $name)
    };
    ($app:expr, $name:expr) => {
        $app.args.iter().find(|a| &a.name == $name)
    };
}

macro_rules! find_subcmd {
    ($app:expr, $sc:expr) => {{
        subcommands!($app).find(|a| match_alias!(a, $sc, &*a.name))
    }};
}

macro_rules! shorts {
    ($app:expr) => {{
        _shorts_longs!($app, short)
    }};
}

macro_rules! longs {
    ($app:expr) => {{
        $app.args
            .iter()
            .filter(|a| a.long.is_some())
            .map(|a| a.long.unwrap())
            .chain(
                $app.args
                    .iter()
                    .filter(|a| a.aliases.is_some())
                    .flat_map(|a| a.aliases.as_ref().unwrap().iter().map(|als| als.0)),
            )
    }};
}

macro_rules! _shorts_longs {
    ($app:expr, $what:ident) => {{
        $app.args.iter().filter_map(|a| a.$what)
    }};
}

macro_rules! _names {
    (@args $app:expr) => {{
        $app.args.iter().map(|a| &*a.name)
    }};
    (@sc $app:expr) => {{
        $app.subcommands.iter().map(|s| &*s.name).chain(
            $app.subcommands
                .iter()
                .filter(|s| s.aliases.is_some())
                .flat_map(|s| s.aliases.as_ref().unwrap().iter().map(|&(n, _)| n)),
        )
    }};
}

macro_rules! sc_names {
    ($app:expr) => {{
        _names!(@sc $app)
    }};
}

macro_rules! match_alias {
    ($a:expr, $to:expr, $what:expr) => {{
        $what == $to
            || ($a.aliases.is_some()
                && $a
                    .aliases
                    .as_ref()
                    .unwrap()
                    .iter()
                    .any(|alias| alias.0 == $to))
    }};
}
