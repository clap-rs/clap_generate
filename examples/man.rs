extern crate clap;
extern crate clap_generate;

use clap::{App, Arg};
use clap_generate::gen_manuals;

fn main() {
    let app = App::new("testapp")
        .about("Pointless application")
        .author("Alice Person <alice@person.com>\nBob Human <bob@human.com>")
        .arg(
            Arg::with_name("debug")
                .short('d')
                .help("Make program output debug messages"),
        )
        .arg(
            Arg::with_name("output")
                .short('o')
                .long("out")
                .takes_value(true)
                .help("Output File"),
        );

    for manual in gen_manuals(&app) {
        println!("{}", manual.render());
    }
}
