use crate::helpers::reporting::get_average_age;
use crate::structs::{Group, Person};

pub fn split_into_groups(persons: &Vec<Person>, number_of_groups: u8) -> Vec<Group> {
    let mut groups: Vec<Group> = Vec::new();

    let group_indexes = get_group_indexes(persons, number_of_groups);

    let global_age_average = get_average_age(&persons);

    for i in 0..group_indexes.len() {
        let mut group = Group {
            age_average: 0,
            age_average_deviation: 0,
            participants_count: 0,
            participants: Vec::new(),
        };

        let persons_slice: Vec<Person>;

        if i == 0 {
            persons_slice = persons[0..=group_indexes[i]].to_vec();
        } else {
            persons_slice = persons[(group_indexes[i - 1] + 1)..=group_indexes[i]].to_vec();
        }

        let group_age_average = get_average_age(&persons_slice);
        group.age_average = group_age_average;
        group.age_average_deviation = group_age_average.abs_diff(global_age_average);
        group.participants_count = persons_slice.len();
        group.participants = persons_slice;

        groups.push(group);
    }

    groups
}

pub fn get_group_indexes(persons: &Vec<Person>, number_of_groups: u8) -> Vec<usize> {
    let mut group_indexes: Vec<usize> = Vec::new();

    let group_size = (persons.len() as f64 / number_of_groups as f64).round() as usize;

    let remainder = persons.len() as u8 % number_of_groups;

    let mut end: usize = group_size - 1;

    for group in 1..=number_of_groups {
        if group != 1 {
            end = end + group_size;
        }

        if group <= remainder {
            end += 1;
        }

        group_indexes.push(end);
    }

    group_indexes
}
