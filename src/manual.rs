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

    vec![manual]

    // manual.description = app.about.map(Into::into);
    // manual.authors = app.author.map(Into::into);
}

fn convert_arg(arg: &clap::Arg) -> man::Arg {
    unimplemented!();
}

// impl Manual {
//     pub fn new<'a, 'b>(app: &App<'a, 'b>) -> Manual {
//         let mut man = Manual::new();
//         Manual::recursive(&mut man, app);
//         man
//     }

//     // TODO: Make this less awful
//     fn add_empty_child(&mut self, name: &str) -> &mut Manual {
//         self.children.push((name.into(), Manual::new()));
//         let (_, ref mut manual) = self.children.last_mut().unwrap();
//         manual
//     }
// }

// fn recurse(manual: &mut Manual, app: &App) {
//     manual.name = app.name.clone().into();
//     manual.description = app.about.map(Into::into);
//     manual.authors = app.author.map(Into::into);
//     manual.flags = vec![];
//     manual.options = vec![];

//     for arg in &app.args {
//         if arg.is_set(ArgSettings::TakesValue) {
//             manual.flags.push(arg.into());
//         } else {
//             manual.options.push(arg.into());
//         }
//     }

//     for app in &app.subcommands {
//         let _inner_name: String = app.name.clone();
//         let mut inner = manual.add_empty_child(&app.name);
//         recurse(&mut inner, app);
//     }
// }
