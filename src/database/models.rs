use serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Recipe {
    pub id: i32,
    pub title: String,
    pub body: String
}