use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io, process};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

fn example() -> anyhow::Result<()> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_reader(io::stdin());

    // println!("{:?}", rdr.headers()?);

    for result in rdr.deserialize() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record: HashMap<String, String> = result?;
        // let record: Player = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}

//cargo run --example csv < examples/data/juventus.csv
