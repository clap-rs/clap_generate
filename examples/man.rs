extern crate clap;
extern crate clap_generate;

use clap::{App, AppSettings, Arg, SubCommand};
use clap_generate::gen_manuals;

fn main() {
    let app = App::new("testapp")
        .about("Pointless application")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .author("Alice Person <alice@person.com>")
        .author("Bob Human <bob@human.com>")
        .arg(
            Arg::with_name("debug")
                .short('d')
                .help("Make program output debug messages"),
        )
        .arg(
            Arg::with_name("output")
                .short('o')
                .takes_value(true)
                .help("Output File"),
        )
        .subcommand(
            SubCommand::with_name("foo").arg(Arg::with_name("bar").short('b').long("barr")),
        );

    for manual in gen_manuals(&app) {
        println!("{}", manual.render());
    }
}
