use clap::{Arg, App, ArgMatches};

pub fn get_cli_helper<'a>() -> ArgMatches<'a> {
    App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg(
            Arg::with_name("in")
                .short("i")
                .long("in")
                .value_name("in")
                .help("The csv file to read")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::with_name("dimensional-separator")
                .short("d")
                .long("dimensional-separator")
                .value_name("dimensional-separator")
                .help("A separator to break header names allowing you to create deeper objects")
                .takes_value(true)
        )
        .get_matches()
}
