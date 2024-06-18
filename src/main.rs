extern crate csv;

mod helpers;
mod structs;

use csv::ReaderBuilder;

use helpers::{
    partial_shuffle::partial_shuffle,
    reporting::{get_worst_deviation, write_results_to_file},
};

use std::{
    fs::File,
    sync::{Arc, Mutex},
    thread,
};

use structs::{Person, Report};

// Number of iterations to find lowest deviation
const MAX_ITERATIONS: u64 = 100_000_000;

const MAX_DEVIATION_GOAL: u8 = 1;

// Location of data source file
const DATA_FILE: &str = "persons.csv";

// Number of groups to distribute people
const NUMBER_OF_GROUPS: u8 = 15;

const APP_PORT: u16 = 8080;

use axum::{routing::get, Json, Router};

async fn get_handler() {
    // Getting number of CPUs available in current machine
    let num_cpus = num_cpus::get();

    // Initializing vector to store threads
    let mut threads: Vec<thread::JoinHandle<Json<Report>>> = Vec::new();

    // Initializing vector to allocate people from CSV into
    let mut persons_original: Vec<Person> = Vec::new();

    // Opening CSV file where the people's names and ages are stored
    let file = File::open(DATA_FILE).expect("Failed to open source data file");

    // Initializing file reader
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    // Iterating over CSV file lines to deserialize people into Person structs
    // and store them in the persons_original vector
    for result in rdr.deserialize::<Person>() {
        let record: Person = match result {
            Ok(record) => record,
            Err(e) => {
                eprintln!("Error deserializing record: {}", e);
                continue;
            }
        };

        persons_original.push(record);
    }

    // Initializing best_shuffle vector to store best combinations as we find them. It's
    // initial value is the original persons vector
    let best_shuffle: Vec<Person> = persons_original.clone();

    // Initializing smallest deviation variable starting with the value of the worst deviation
    // of the original persons vector.
    let smallest_deviation: u8 = get_worst_deviation(&persons_original, NUMBER_OF_GROUPS);

    let best_shuffle_arc_mutex: Arc<Mutex<Vec<Person>>> = Arc::new(Mutex::new(best_shuffle));

    let smallest_deviation_arc_mutex: Arc<Mutex<u8>> = Arc::new(Mutex::new(smallest_deviation));

    // At this point the persons_original vector is filled with people from the CSV file
    // Now we will create threads according to the number of CPUs to find the best
    // combination of people in groups
    for thread_number in 0..num_cpus {
        let best_shuffle: Arc<Mutex<Vec<Person>>> = Arc::clone(&best_shuffle_arc_mutex);
        let smallest_deviation: Arc<Mutex<u8>> = Arc::clone(&smallest_deviation_arc_mutex);

        let single_thread = thread::spawn(move || {
            // Reporting initial results as the current best shuffle
            let mut report: Json<Report> = write_results_to_file(
                &best_shuffle.lock().unwrap(),
                NUMBER_OF_GROUPS,
                thread_number as u64,
            );

            let mut best_shuffle_local = best_shuffle.lock().unwrap().clone();
            let mut smallest_deviation_local = *smallest_deviation.lock().unwrap();

            for i in 0..MAX_ITERATIONS {
                // Calculating progress based on current iteration and max iteration
                let progress: f64 = i as f64 / MAX_ITERATIONS as f64;

                // Setting shuffling percentage as the complement of current progress:
                // This means, the longer the process has been running, the smallest
                // Will be the number of elements moved around
                let shuffle_percentage = 1.0 - progress;

                // Shuffle best combination with the shuffle percentage rate to try
                // to find a better combination
                let current_persons_shuffle =
                    partial_shuffle(&best_shuffle_local, shuffle_percentage);

                // Getting worst age average deviation of any group in the shuffled
                // vector
                let worst_deviation =
                    get_worst_deviation(&current_persons_shuffle, NUMBER_OF_GROUPS);

                // If the current worst deviation is smaller than the best smallest
                // deviation, then store it, and output reports to file in results
                // folder
                if worst_deviation < smallest_deviation_local {
                    if worst_deviation < *smallest_deviation.lock().unwrap() {
                        best_shuffle_local = current_persons_shuffle.clone();

                        *smallest_deviation.lock().unwrap() = worst_deviation;

                        *best_shuffle.lock().unwrap() = current_persons_shuffle;

                        report = write_results_to_file(&best_shuffle_local, NUMBER_OF_GROUPS, i);

                        println!("Thread id: {thread_number}; Worst deviation: {worst_deviation}; Smallest deviation: {}; Progress: {progress}", smallest_deviation.lock().unwrap());

                        if worst_deviation <= MAX_DEVIATION_GOAL {
                            break;
                        }
                    } else {
                        smallest_deviation_local = *smallest_deviation.lock().unwrap();
                    }
                }
            }
            report
        });
        threads.push(single_thread);
    }

    for single_thread in threads {
        let _ = single_thread.join().unwrap();
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(get_handler));

    let ip_address = format!("0.0.0.0:{}", APP_PORT);

    let listener = tokio::net::TcpListener::bind(ip_address).await.unwrap();

    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
