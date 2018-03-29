extern crate clap;
extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_json;

mod cli;
mod data;

use std::collections::HashMap;
use serde_json::Value as JsonValue;

fn main() {
    let cli_helper = cli::get_cli_helper();

    let csv_file = cli_helper
        .value_of("in")
        .expect("You must specify an input csv with --in");
    let ds = cli_helper.value_of("dimensional-separator");
    let mut csv_reader = csv::Reader::from_file(csv_file).expect("Could not read csv file");

    let headers = csv_reader.headers().unwrap();

    let data: Vec<HashMap<String, JsonValue>> = csv_reader.records() //
        .filter(|row| row.is_ok()) // Skip anything we can't read
        .map(|row| row.unwrap()) // It's now safe to unwrap
        .map(|row| data::row_to_object(&headers, row, ds)) // Turn the row into an object
        .collect();

    println!("{}", serde_json::to_string_pretty(&data).unwrap());
}
