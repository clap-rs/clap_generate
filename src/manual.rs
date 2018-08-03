use clap;
use man;

/// Generate a man page from a `clap::App` instance.
// NOTE(yw): in future versions we should allow this to recurse and generate
// man pages for subcommands. We're returning a vector now to support this in
// future versions.
pub fn gen_manuals(app: &clap::App) -> Vec<man::Manual> {
    let mut manual = man::Manual::new(&app.name);

    for about in &app.about {
        manual = manual.about(about.to_string());
    }

    // Assume multiple authors are passed separated by newline. Worst case the
    // formatting comes out slightly different.
    for authors in &app.author {
        for author in authors.split("\n") {
            manual = manual.author(man::Author::new(author));
        }
    }

    for arg in &app.args {
        if let Some(_index) = arg.index {
            let mut positional_arg = man::Arg::new(arg.name);
            manual = manual.arg(positional_arg);
        } else if arg.is_set(clap::ArgSettings::TakesValue) {
            let mut opt = man::Opt::new(arg.name);
            if let Some(help) = get_help(arg) {
                opt = opt.help(&help);
            }
            if let Some(short) = arg.short {
                opt = opt.short(&format!("-{}", short.to_string()));
            }
            if let Some(long) = arg.long {
                opt = opt.long(&format!("--{}", long));
            }
            manual = manual.option(opt);
        } else {
            let mut flag = man::Flag::new();
            if let Some(help) = get_help(arg) {
                flag = flag.help(&help);
            }
            if let Some(short) = arg.short {
                flag = flag.short(&format!("-{}", short.to_string()));
            }
            if let Some(long) = arg.long {
                flag = flag.long(&format!("--{}", long));
            }
            manual = manual.flag(flag);
        }
    }

    vec![manual]
}

fn get_help(arg: &clap::Arg) -> Option<String> {
    if let Some(help) = arg.long_help {
        Some(help.into())
    } else if let Some(help) = arg.help {
        Some(help.into())
    } else {
        None
    }
}
