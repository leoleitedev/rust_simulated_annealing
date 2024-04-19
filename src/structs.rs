use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Person {
    pub name: String,
    pub age: u32,
}
