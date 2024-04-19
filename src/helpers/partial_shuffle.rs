use crate::structs::Person;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

pub fn partial_shuffle(persons: &mut Vec<Person>, percentage: f64) {
    let mut rng = thread_rng();
    let records_to_shuffle = (persons.len() as f64 * percentage).round() as usize;
    let mut indices: Vec<usize> = (0..persons.len()).collect();
    indices.shuffle(&mut rng);
    let indices_to_shuffle = &indices[..records_to_shuffle];

    for &index in indices_to_shuffle {
        let swap_index = rng.gen_range(0..persons.len()); // Use gen_range method with correct argument
        persons.swap(index, swap_index);
    }
}
