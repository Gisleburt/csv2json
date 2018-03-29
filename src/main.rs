extern crate clap;
extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_json;

use clap::{Arg, App};

use std::collections::HashMap;
use serde_json::Value as JsonValue;

fn dimensional_converter(key: String, value: String) -> (String, JsonValue) {
    let separator = ".";
    if key.contains(separator) {
        let mut parts = key.split(separator);
        let this_key = parts.next().unwrap().to_owned();
        let next_key = parts.collect::<Vec<&str>>().join(".").to_owned();
        let (_, data)  = dimensional_converter(next_key.clone(), value);
        return (
            this_key,
            json!({next_key: data})
        )

    }
    (key, json!(value))
}

fn row_to_object(headers: &Vec<String>, row: Vec<String>) -> HashMap<String, JsonValue> {
    let mut items = HashMap::new();
    row.iter()
        .cloned()
        .zip(headers.iter().cloned())
        .for_each(|(value, key)| {
            let (k, v) = dimensional_converter(key, value);
            items.insert(k, v);
        });
    items
}

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

    let data: Vec<HashMap<String, JsonValue>> = csv_reader.records() //
        .filter(|row| row.is_ok()) // Skip anything we can't read
        .map(|row| row.unwrap()) // It's now safe to unwrap
        .map(|row| row_to_object(&headers, row)) // Turn the row into an object
        .collect();

    println!("{}", serde_json::to_string_pretty(&data).unwrap());
}
