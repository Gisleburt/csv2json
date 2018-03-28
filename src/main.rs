extern crate clap;
extern crate csv;
extern crate serde;
extern crate serde_json;

use clap::{Arg, App};

use std::collections::HashMap;

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
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
//        .arg(
//            Arg::with_name("out")
//                .short("o")
//                .long("out")
//                .value_name("out")
//                .help("The json file to output")
//                .takes_value(true)
//                .required(true)
//        )
        .get_matches();

    let csv_file = matches.value_of("in").expect("You must specify an input csv with --in");
    let mut csv_reader = csv::Reader::from_file(csv_file).expect("Could not read csv file");

    let headers = csv_reader.headers().unwrap();

    let data: Vec<HashMap<String, String>> = csv_reader.records()
        .map(|row| row.unwrap())
        .map(
            |row| {
                let mut items = HashMap::new();
                row.iter()
                    .cloned()
                    .zip(headers.iter().cloned())
                    .for_each(|(a, b)| { let _ = items.insert(b,a); });
                items
            }
        )
        .collect();

    println!("{}", serde_json::to_string_pretty(&data).unwrap());
}
