use crate::structs::Person;

pub fn split_into_groups(persons: &Vec<Person>, number_of_groups: usize) -> Vec<Vec<Person>> {
    let mut groups: Vec<Vec<Person>> = Vec::new();

    let group_indexes = get_group_indexes(persons, number_of_groups);

    for i in 0..group_indexes.len() {
        let group: Vec<Person>;

        if i == 0 {
            group = persons[0..=group_indexes[i]].to_vec();
        } else {
            group = persons[(group_indexes[i - 1] + 1)..=group_indexes[i]].to_vec();
        }

        groups.push(group);
    }

    groups
}

pub fn get_group_indexes(persons: &Vec<Person>, number_of_groups: usize) -> Vec<usize> {
    let mut group_indexes: Vec<usize> = Vec::new();

    let group_size = (persons.len() as f64 / number_of_groups as f64).round() as usize - 1;

    let remainder = persons.len() % number_of_groups;

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
