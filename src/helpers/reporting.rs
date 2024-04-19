use std::{fs::File, io::Error, io::Write};

use chrono::Utc;

use crate::{helpers::grouping::split_into_groups, structs::Person};
use serde_json;

fn get_persons_count(persons: &Vec<Person>) -> u32 {
    match u32::try_from(persons.len()) {
        Ok(count) => count,
        Err(_) => panic!("Failed to convert the length of persons to u32"),
    }
}

fn get_total_age(persons: &Vec<Person>) -> u32 {
    persons.iter().map(|p| p.age).sum()
}

pub fn get_average_deviation(persons: &Vec<Person>) -> u8 {
    let persons_count = get_persons_count(&persons);

    let total_age: u32 = get_total_age(&persons);

    let average_age = total_age / persons_count;

    let mut average_deviation: u32 = 0;

    average_deviation as u8
}

pub fn extract_metrics(persons: &Vec<Person>) {
    let total_records: usize = persons.len();
    let total_age: u32 = persons.iter().map(|p| p.age).sum();
    let average_age = total_age / total_records as u32;

    println!("Total records: {}", total_records);
    println!("Total age: {}", total_age);
    println!("Average age: {}", average_age);
}

pub fn write_results_to_file(persons: &Vec<Person>) -> std::io::Result<()> {
    let groups = split_into_groups(&persons, 10);

    let timestamp = Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string();

    // Create the file name with the timestamp
    let file_name = format!("results/output_{}.json", timestamp);

    let mut file = File::create(&file_name)?;
    // Serialize groups to JSON
    let json_data = serde_json::to_string(&groups)?;

    // Write JSON data to the file
    file.write_all(json_data.as_bytes())?;

    println!("Data has been written to {}", &file_name);

    // Finish writing and close the file
    file.flush()?;

    println!("\n");

    extract_metrics(persons);

    Ok(())
}
