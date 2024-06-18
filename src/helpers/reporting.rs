use std::{
    fs::{self, File},
    io::Write,
};

use axum::Json;
use chrono::Utc;

use crate::{
    helpers::grouping::{get_group_indexes, split_into_groups},
    structs::{Group, Person, Report},
};

use serde_json;

fn get_persons_count(persons: &[Person]) -> u32 {
    match u32::try_from(persons.len()) {
        Ok(count) => count,
        Err(_) => panic!("Failed to convert the length of persons to u32"),
    }
}

fn get_total_age(persons: &[Person]) -> u32 {
    let sum: u64 = persons.iter().map(|p| p.age as u64).sum();

    match u32::try_from(sum) {
        Ok(result) => result,
        Err(_) => panic!("Failed to convert the total age to u32"),
    }
}

pub fn get_average_age(persons: &[Person]) -> u8 {
    let persons_count: u32 = get_persons_count(persons);
    let total_age: u32 = get_total_age(persons);

    u8::try_from(total_age / persons_count).unwrap()
}

pub fn get_worst_deviation(persons: &[Person], number_of_groups: u8) -> u8 {
    let average_age: u8 = get_average_age(persons);

    let group_indexes = get_group_indexes(persons, number_of_groups);

    let mut worst_deviation: usize = 0;

    for group_index in 0..group_indexes.len() {
        let mut start_index = 0;
        let end_index = group_indexes[group_index];

        if group_index != 0 {
            start_index = group_indexes[group_index - 1] + 1
        }

        let mut group_total_age = 0;

        let group_participants_count = end_index - start_index + 1;

        for person in &persons[start_index..=end_index] {
            group_total_age += usize::from(person.age);
        }

        let group_age_average = group_total_age / group_participants_count;
        let group_age_average_deviation = group_age_average.abs_diff(usize::from(average_age));

        if group_age_average_deviation > worst_deviation {
            worst_deviation = group_age_average_deviation
        }
    }

    worst_deviation as u8
}

pub fn extract_metrics(persons: &[Person], number_of_groups: u8) {
    let total_records: u32 = get_persons_count(persons);
    let total_age: u32 = get_total_age(persons);
    let average_age = total_age / total_records;
    let worst_deviation = get_worst_deviation(persons, number_of_groups);

    println!("Total records: {total_records}; Total age: {total_age}; Average age: {average_age}; Worst deviation: {worst_deviation}");
}

pub fn write_results_to_file(
    persons: &[Person],
    number_of_groups: u8,
    iteration: u64,
) -> Json<Report> {
    let groups: Vec<Group> = split_into_groups(persons, number_of_groups);

    let report = Report {
        age_average: get_average_age(persons),
        age_average_worst_deviation: get_worst_deviation(persons, number_of_groups),
        participants_count: persons.len(),
        groups,
    };

    println!("\n");

    println!("Iteration {}", iteration);
    extract_metrics(persons, number_of_groups);

    let timestamp = Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string();

    if fs::metadata("results").is_err() {
        fs::create_dir("results").expect("Failed to create path")
    }

    // Allocate the file name with the timestamp into string variable
    let file_name = format!("results/output_{}.json", timestamp);

    // Create the results file
    let mut file = match File::create(&file_name) {
        Ok(file) => file,
        Err(err) => panic!("Failed to create output file: {}", err),
    };

    // Serialize groups to JSON string
    let json_data = match serde_json::to_string(&report) {
        Ok(json) => json,
        Err(err) => panic!("Failed to serialize groups into a JSON string: {}", err),
    };

    let write_to_file_error_message = format!("Failed to write to file {}", file_name);

    // Write JSON data to the file
    file.write_all(json_data.as_bytes())
        .expect(&write_to_file_error_message);

    println!("Data has been written to {}", &file_name);

    // Finish writing and close the file
    file.flush().unwrap();

    Json(report)
}
