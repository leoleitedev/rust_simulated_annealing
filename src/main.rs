extern crate csv;

mod helpers;
mod structs;

use csv::ReaderBuilder;

use helpers::{partial_shuffle::partial_shuffle, reporting::write_results_to_file};

use std::fs::File;

use structs::Person;

fn main() -> std::io::Result<()> {
    let file = File::open("persons.csv")?;

    let mut persons_original: Vec<Person> = Vec::new();

    let mut persons_shuffled: Vec<Person> = Vec::new();

    let mut average_deviation: u8;

    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    for result in rdr.deserialize::<Person>() {
        let record: Person = match result {
            Ok(record) => record,
            Err(e) => {
                eprintln!("Error deserializing record: {}", e);
                continue;
            }
        };

        persons_original.push(record.clone());
        persons_shuffled.push(record.clone());
    }

    for i in 0..1_000_000 {
        partial_shuffle(&mut persons_shuffled, 0.1);
    }

    write_results_to_file(&persons_shuffled)
}
