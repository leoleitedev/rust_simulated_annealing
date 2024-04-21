use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Person {
    pub name: String,
    pub age: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Group {
    pub age_average: u8,
    pub age_average_deviation: u8,
    pub participants_count: usize,
    pub participants: Vec<Person>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Report {
    pub age_average: u8,
    pub age_average_worst_deviation: u8,
    pub participants_count: usize,
    pub groups: Vec<Group>,
}
